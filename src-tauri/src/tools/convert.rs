use image::imageops::FilterType;
use image::GenericImageView;
use serde_json::json;
use std::fs;
use std::process::Stdio;
use std::{path::Path, sync::Arc};
use tauri::{path::BaseDirectory, Emitter, Manager};
use tauri_plugin_opener::open_path;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::Notify;
use uuid::Uuid;

use crate::helpers::{
    ffmpeg_helper::{ffmpeg_builder, get_video_duration_millis, time_to_seconds},
    settings_helper,
};

use crate::tools::{app_handle::app, error::Error, stopwatch};

pub async fn exec_conversion(
    id: Uuid,
    path: &str,
    extension: &str,
    category: &str,
    cancel_notify: &Arc<Notify>,
) -> Result<(), Error> {
    let file = Path::new(&path);
    let stem = file
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(Error::InvalidPath)?;
    let full_path = file.parent().unwrap().join(stem);
    let output_path = format!("{}.{}", full_path.to_str().unwrap().to_owned(), &extension);

    let args = ffmpeg_builder(&path, &output_path, &extension).await;
    let settings = settings_helper::load_settings();

    let _ = app().emit("job-started", true);

    match category {
        "video" | "audio" => {
            let ffmpeg_path = app()
                .path()
                .resolve(
                    if cfg!(target_os = "windows") {
                        "bin/ffmpeg.exe"
                    } else {
                        "bin/ffmpeg"
                    },
                    BaseDirectory::AppLocalData,
                )
                .map_err(|e| Error::Io(e.to_string()))?;

            let mut timer = stopwatch::Stopwatch::new();

            timer.start();

            let mut command = Command::new(&ffmpeg_path);

            command
                .args(&args)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());

            #[cfg(target_os = "windows")]
            #[allow(unused_imports)]
            {
                use std::os::windows::process::CommandExt;
                command.creation_flags(0x08000000);
            }

            let mut child = command.spawn()?;

            let stdout = child.stdout.take().ok_or(Error::FfmpegStdout)?;
            let mut reader = BufReader::new(stdout).lines();

            let total_duration_sec = get_video_duration_millis(ffmpeg_path, &path);

            let loop_result = loop {
                tokio::select! {
                    line = reader.next_line() => {
                        match line {
                            Ok(Some(line_str)) => {
                                let parts: Vec<&str> = line_str.split("=").collect();

                                if parts.len() == 2 && parts[0] == "out_time" {
                                    let current_sec = time_to_seconds(parts[1].trim());
                                    let percentage = ((current_sec / total_duration_sec) * 100.0).min(100.0) as i32;

                                    let _ = app().emit("job-progress", json!({ "id": id.to_string(), "progress": percentage }));
                                }
                            },

                            Ok(None) => break Ok(()),
                            Err(e) => break Err(Error::Io(e.to_string()))
                        }
                    },

                    _ = cancel_notify.notified() => {
                        child.kill().await.map_err(|e| Error::Io(e.to_string()))?;
                        let _ = child.wait().await;

                        let file = Path::new(&output_path);

                        if file.exists() {
                            fs::remove_file(file).ok();
                        }

                        break Err(Error::ConversionCancelled);
                    }
                }
            };

            if let Err(e) = loop_result {
                return Err(e);
            }

            let output = child.wait_with_output().await?;

            if output.status.success() {
                let total_time = timer.elapsed().as_secs();
                let data = json!({
                    "id": id.to_string(),
                    "total_time": total_time,
                    "input_file": path,
                    "new_file_path": output_path
                });

                let _ = app().emit("job-completed", data);

                timer.reset();

                if settings.open_when_finished {
                    open_path(output_path, None::<&str>).unwrap();
                }
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr).to_string();
                let file = Path::new(&output_path);

                if file.exists() {
                    fs::remove_file(file).ok();
                }

                let error_line = error_message
                    .split("\n")
                    .find(|line| line.contains("Encoder not found"))
                    .is_some();

                if error_line {
                    return Err(Error::FfmpegFailed("Your GPU does not support hardware acceleration for this encoder. Please try to change the encoder or use CPU (Normal) encoding.".to_string()));
                } else {
                    return Err(Error::FfmpegFailed(error_message));
                }
            }
        }

        "image" => {
            //if cancel_notify.notified().await { return Err(Error::ConversionCancelled); }

            let mut img = image::open(file).unwrap();

            if extension == "ico" {
                let (width, height) = img.dimensions();
                let max_size = 256;
                let scale = (max_size as f32 / width.max(height) as f32).min(1.0);

                let new_width = (width as f32 * scale).round() as u32;
                let new_height = (height as f32 * scale).round() as u32;

                img = img.resize(new_width, new_height, FilterType::Lanczos3);
            }

            img.save(&output_path)
                .map_err(|e| Error::ImageError(e.to_string()))?;
            let _ = app().emit("job-completed", json!({ "id": id }));

            if settings.open_when_finished {
                open_path(&output_path, None::<&str>).unwrap();
            }
        }

        _ => {
            return Err(Error::InvalidCategory(category.to_string()));
        }
    }

    Ok(())
}
