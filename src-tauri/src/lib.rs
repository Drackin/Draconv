use image::imageops::FilterType;
use image::GenericImageView;
use serde::Serialize;
use serde_json::json;
use tauri_plugin_opener::open_path;
use std::process::Stdio;
use std::{env, fs::metadata, path::Path, sync::Arc};
use tauri::{path::BaseDirectory, AppHandle, Emitter, Listener, Manager, State, Window};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::{oneshot, Mutex};

mod helpers;
mod tools;

use helpers::ffmpeg_helper::{ffmpeg_builder, get_video_duration_millis, time_to_seconds};
use helpers::settings_helper::{self, load_settings, save_settings};
use helpers::net_helper::check_connection;
use tools::installer::{install_ffmpeg, is_ffmpeg_installed};
use tools::stopwatch;

#[derive(Debug, Serialize, thiserror::Error)]
enum Error {
    #[error("Path is not a file: {0}")]
    NotAFile(String),
    #[error("Invalid file path")]
    InvalidPath,
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Another conversion is already in progress")]
    ConversionInProgress,
    #[error("Conversion was cancelled by the user")]
    ConversionCancelled,
    #[error("FFmpeg process failed: {0}")]
    FfmpegFailed(String),
    #[error("Could not capture FFmpeg stdout")]
    FfmpegStdout,
    #[error("Image processing error: {0}")]
    ImageError(String),
    #[error("Invalid category for conversion: {0}")]
    InvalidCategory(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

#[derive(Debug, Serialize)]
struct FileDetails {
    full_file_name: String,
    file_extension: String,
    file_name: String,
    dir_path: String,
}

#[tauri::command]
fn check_type(path: &str) -> bool {
    let md = metadata(path).unwrap();
    md.is_file()
}

#[tauri::command]
fn get_file_name(path: &str) -> Result<FileDetails, Error> {
    let file_path = Path::new(path);

    if !file_path.is_file() {
        return Err(Error::NotAFile(path.to_string()));
    }

    let full_file_name = file_path.file_name().and_then(|s| s.to_str()).ok_or(Error::InvalidPath)?.to_string();
    let file_extension = file_path.extension().and_then(|s| s.to_str()).ok_or(Error::InvalidPath)?.to_string();
    let file_name = file_path.file_stem().and_then(|s| s.to_str()).ok_or(Error::InvalidPath)?.to_string();
    let dir_path = file_path.parent().and_then(|s| s.to_str()).ok_or(Error::InvalidPath)?.to_string();

    Ok(FileDetails {
        full_file_name,
        file_extension,
        file_name,
        dir_path
    })
}

#[tauri::command]
fn get_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args
}

#[derive(Clone)]
struct ConversionState {
    killer: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

// BIGGEST HISTORICAL FUNCTION
#[tauri::command]
async fn convert(
    path: String,
    extension: String,
    category: &str,
    app: AppHandle,
    window: Window,
    state: State<'_, ConversionState>,
) -> Result<String, Error> {
    if state.killer.lock().await.is_some() {
        return Err(Error::ConversionInProgress);
    }

    let file = Path::new(&path);

    let stem = file.file_stem().and_then(|s| s.to_str()).ok_or(Error::InvalidPath)?;
    
    let full_path = file.parent().unwrap().join(stem);
    let output_path = format!("{}.{}", full_path.to_str().unwrap().to_owned(), &extension);
    
    let args = ffmpeg_builder(&path, &output_path, &extension).await;
    let settings = settings_helper::load_settings(app.clone());
    
    
    let _ = window.emit("conversion-started", true);
    
    
    let (killer_tx, mut killer_rx) = oneshot::channel::<()>();
    
    match category {
        "video" | "audio" => {
            let ffmpeg_path = app
                .path()
                .resolve(
                    if cfg!(target_os = "windows") { "bin/ffmpeg.exe" } else { "bin/ffmpeg" },
                    BaseDirectory::AppData,
                ).map_err(|e| Error::Io(e.to_string()))?;


            let mut timer = stopwatch::Stopwatch::new();

            timer.start();

            let mut command = Command::new(&ffmpeg_path);

            command.args(&args)
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

            {
                let mut killer_lock = state.killer.lock().await;
                *killer_lock = Some(killer_tx);
            }

            let app_clone = app.clone();

            let conversion_result: Result<(), Error> = loop {
                tokio::select! {
                    line = reader.next_line() => {
                        match line {
                            Ok(Some(line_str)) => {
                                let parts: Vec<&str> = line_str.split("=").collect();

                                if parts.len() == 2 && parts[0] == "out_time" {
                                    let current_sec = time_to_seconds(parts[1].trim());
                                    let percentage = ((current_sec / total_duration_sec) * 100.0).min(100.0) as i32;

                                    let _ = window.emit("conversion-progress", percentage);
                                }
                            },

                            Ok(None) => break Ok(()),
                            Err(e) => break Err(Error::Io(e.to_string()))
                        }
                    },

                    _ = &mut killer_rx => {
                        child.kill().await.map_err(|e| Error::Io(e.to_string()))?;
                        let _ = app.emit("conversion-cancelled", true);
                        break Err(Error::ConversionCancelled);
                    }
                }
            };

            *state.killer.lock().await = None;

            conversion_result?;

            let output = child.wait_with_output().await?;

            if output.status.success() {
                let total_time = timer.elapsed().as_secs();
                let data = json!({
                    "total_time": total_time,
                    "new_file_path": output_path
                });
                let _ = app_clone.emit("conversion-finished", data.to_string());

                timer.reset();

                if settings.open_when_finished {
                    open_path(output_path, None::<&str>).unwrap();
                }
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr).to_string();
                let _ = window.emit("conversion-error", &error_message);
                return Err(Error::FfmpegFailed(error_message))
            }
        }

        "image" => {
            let file: &Path = Path::new(&path);
            let file_stem = file.file_stem().unwrap().to_str().unwrap();
            let full_path = file.parent().unwrap().join(file_stem);
            let output_path = format!("{}.{}", full_path.to_str().unwrap().to_owned(), &extension);

            let mut img = image::open(file).unwrap();

            if extension == "ico" {
                let (width, height) = img.dimensions();
                let max_size = 256;
                let scale = (max_size as f32 / width.max(height) as f32).min(1.0);

                let new_width = (width as f32 * scale).round() as u32;
                let new_height = (height as f32 * scale).round() as u32;

                img = img.resize(new_width, new_height, FilterType::Lanczos3);
            }

            img.save(output_path).map_err(|e| Error::ImageError(e.to_string()))?;
            let _ = window.emit("conversion-finished", true);
        }

        _ => {
            return Err(Error::InvalidCategory(category.to_string()));
        }
    }

    Ok("Media conversion started.".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ConversionState {
            killer: Arc::new(Mutex::new(None)),
        })
        .plugin(tauri_plugin_prevent_default::debug())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let handle = app.handle().clone();
            tools::app_handle::init_global_app(handle);

            let state = app.state::<ConversionState>().clone();
            let state_clone = state.killer.clone();

            app.listen("cancel-conversion", move |_| {
                let state = state_clone.clone();

                tauri::async_runtime::spawn(async move {

                    if let Some(killer) = state.lock().await.take() {
                        let _ = killer.send(());
                    }
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_file_name,
            check_type,
            get_args,
            convert,
            load_settings,
            save_settings,
            install_ffmpeg,
            is_ffmpeg_installed,
            check_connection,
            
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
