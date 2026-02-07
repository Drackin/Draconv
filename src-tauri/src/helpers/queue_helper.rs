use serde_json::json;
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use tauri::Emitter;
use tokio::sync::{Mutex, Notify, Semaphore};
use uuid::Uuid;

use crate::tools::{app_handle::app, convert, error::Error};

// Job object for each file
struct PipelineJob {
    pub id: Uuid,
    pub path: String,
    pub extension: String,
    pub category: String,
    //pub cancel_notifier: Arc<Notify> (Notifier created while process, to decrease the RAM usage)
}

#[derive(serde::Deserialize)]
pub struct JobRequest {
    pub id: String,
    pub path: String,
    pub extension: String,
    pub category: String,
}

struct PipelineState {
    queue: Mutex<VecDeque<PipelineJob>>,
    running_jobs: Mutex<HashMap<Uuid, Arc<Notify>>>,
    current_limit: Mutex<usize>,
}

#[derive(Clone)]
pub struct PipelineManager {
    inner: Arc<PipelineState>,
    semaphore: Arc<Semaphore>,
}

impl PipelineManager {
    pub fn new(max_concurrency: usize) -> Self {
        Self {
            inner: Arc::new(PipelineState {
                queue: Mutex::new(VecDeque::new()),
                running_jobs: Mutex::new(HashMap::new()),
                current_limit: Mutex::new(max_concurrency),
            }),
            semaphore: Arc::new(Semaphore::new(max_concurrency)),
        }
    }

    pub async fn add_job(&self, job: JobRequest) -> Result<String, std::io::Error> {
        let id = Uuid::parse_str(&job.id).unwrap();

        let job = PipelineJob {
            id,
            path: job.path,
            extension: job.extension,
            category: job.category,
        };

        {
            let mut queue = self.inner.queue.lock().await;
            queue.push_back(job);
        }

        let _ = app().emit("job-queued", json!({ "id": id.to_string() }));

        Ok(id.to_string())
    }

    pub async fn add_jobs(&self, jobs: Vec<JobRequest>) {
        let mut queue = self.inner.queue.lock().await;

        for job in jobs {
            let id = Uuid::parse_str(&job.id).unwrap();

            queue.push_back(PipelineJob {
                id,
                path: job.path,
                extension: job.extension,
                category: job.category,
            });

            let _ = app().emit("job-queued", json!({ "id": id.to_string() }));
        }

        drop(queue);
        self.try_dispatch();
    }

    pub async fn cancel_job(&self, id: String) -> Result<(), Error> {
        let job_id = Uuid::parse_str(&id).map_err(|_| Error::JobNotFound)?;

        // First, searching on running jobs if the file is in process
        {
            let running = self.inner.running_jobs.lock().await;

            if let Some(notify) = running.get(&job_id) {
                notify.notify_one();
                return Ok(());
            }
        }

        // Then searching on queued jobs if file is not in process yet
        {
            let mut queue = self.inner.queue.lock().await;

            if let Some(pos) = queue.iter().position(|job| job.id == job_id) {
                queue.remove(pos);

                let _ = app().emit("job-cancelled", json!({ "id": job_id.to_string() }));
                return Ok(());
            }
        }

        Err(Error::JobNotFound)
    }

    pub async fn cancel_all(&self) {
        {
            let mut queue = self.inner.queue.lock().await;

            queue.clear();
        }

        {
            let running = self.inner.running_jobs.lock().await;

            for (_, notify) in running.iter() {
                notify.notify_one();
            }
        }

        let _ = app().emit("jobs-cancelled", true);
    }

    pub fn try_dispatch(&self) {
        let manager = self.clone();

        tauri::async_runtime::spawn(async move {
            manager.dispatch_loop().await;
        });
    }

    async fn dispatch_loop(&self) {
        loop {
            // Checking the semaphore for available slots
            let permit = match self.semaphore.clone().try_acquire_owned() {
                Ok(p) => p,
                Err(_) => break,
            };

            // Then checking for available jobs
            let job: Option<PipelineJob> = {
                let mut queue = self.inner.queue.lock().await;
                queue.pop_front()
            };

            match job {
                Some(job) => {
                    // If both are true
                    let cancel_notify = Arc::new(Notify::new()); // Line 29

                    {
                        let mut running = self.inner.running_jobs.lock().await;

                        running.insert(job.id, cancel_notify.clone());
                    }

                    let manager_clone = self.clone();
                    let notify_clone = cancel_notify.clone();

                    tauri::async_runtime::spawn(async move {
                        let _permit = permit;

                        let _ = app().emit("job-started", json!({ "id": job.id.to_string() }));
                        let result = convert::exec_conversion(
                            job.id,
                            &job.path,
                            &job.extension,
                            &job.category,
                            &notify_clone,
                        )
                        .await;

                        match result {
                            Ok(_) => (),
                            Err(Error::ConversionCancelled) => {
                                let _ = app()
                                    .emit("job-cancelled", json!({ "id": job.id.to_string() }));
                            }
                            Err(e) => {
                                let _ = app().emit(
                                    "job-failed",
                                    json!({ "id": job.id.to_string(), "error": e.to_string() }),
                                );
                            }
                        }

                        {
                            let mut running = manager_clone.inner.running_jobs.lock().await;
                            running.remove(&job.id);
                        }

                        drop(_permit);

                        manager_clone.try_dispatch();
                    });
                }
                None => {
                    drop(permit);
                    let _ = app().emit("all-jobs-completed", true);
                    break;
                }
            }
        }
    }

    pub async fn set_concurrency(&self, new_limit: usize) {
        let mut current_limit = self.inner.current_limit.lock().await;

        if new_limit > *current_limit {
            let difference = new_limit - *current_limit;
            self.semaphore.add_permits(difference);

            self.try_dispatch();
        } else if new_limit < *current_limit {
            let difference = *current_limit - new_limit;
            let sem = self.semaphore.clone();

            // if user decreases the limit, need to acquire the extra permits to reduce the available slots
            tokio::spawn(async move {
                if let Ok(permits) = sem.acquire_many(difference as u32).await {
                    permits.forget();
                }
            });
        }

        *current_limit = new_limit;
    }
}
