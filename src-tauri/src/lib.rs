use serde::Serialize;
use std::{env, fs::metadata, path::Path};
use tauri::{Manager, State};
use uuid::Uuid;

mod helpers;
mod tools;

use helpers::{
    net_helper::check_connection,
    queue_helper::PipelineManager,
    settings_helper::{load_settings, reset_settings, save_settings},
};
use tools::installer::{install_ffmpeg, is_ffmpeg_installed};

use crate::{helpers::queue_helper::JobRequest, tools::error::Error};

#[derive(Debug, Serialize)]
struct FileDetails {
    id: String,
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
fn get_file_data(path: &str) -> Result<FileDetails, Error> {
    let file_path = Path::new(path);

    if !file_path.is_file() {
        return Err(Error::NotAFile(path.to_string()));
    }

    let full_file_name = file_path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or(Error::InvalidPath)?
        .to_string();
    let file_extension = file_path
        .extension()
        .and_then(|s| s.to_str())
        .ok_or(Error::InvalidPath)?
        .to_string();
    let file_name = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(Error::InvalidPath)?
        .to_string();
    let dir_path = file_path
        .parent()
        .and_then(|s| s.to_str())
        .ok_or(Error::InvalidPath)?
        .to_string();

    Ok(FileDetails {
        id: Uuid::new_v4().to_string(),
        full_file_name,
        file_extension,
        file_name,
        dir_path,
    })
}

#[tauri::command]
fn get_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args
}

#[tauri::command]
async fn add_job(
    id: String,
    path: String,
    extension: String,
    category: String,
    manager: State<'_, PipelineManager>,
) -> Result<String, String> {
    manager
        .add_job(JobRequest {
            id,
            path,
            extension,
            category,
        })
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn convert(
    id: String,
    path: String,
    extension: String,
    category: String,
    manager: State<'_, PipelineManager>,
) -> Result<(), String> {
    let _ = manager
        .add_job(JobRequest {
            id,
            path,
            extension,
            category,
        })
        .await
        .map_err(|e| e.to_string())?;

    manager.try_dispatch();

    Ok(())
}

#[tauri::command]
async fn add_all_jobs(
    files: Vec<JobRequest>,
    manager: State<'_, PipelineManager>,
) -> Result<(), Error> {
    manager.add_jobs(files).await;

    Ok(())
}

#[tauri::command]
async fn cancel_job(id: String, manager: State<'_, PipelineManager>) -> Result<(), Error> {
    manager.cancel_job(id).await
}

#[tauri::command]
async fn cancel_all_jobs(manager: State<'_, PipelineManager>) -> Result<(), Error> {
    manager.cancel_all().await;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            let window = app.get_webview_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }))
        .plugin(tauri_plugin_prevent_default::debug())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let handle = app.handle().clone();
            tools::app_handle::init_global_app(handle);

            let settings = load_settings();

            let manager = PipelineManager::new(settings.max_concurrency);
            app.manage(manager);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_file_data,
            check_type,
            get_args,
            load_settings,
            save_settings,
            reset_settings,
            install_ffmpeg,
            is_ffmpeg_installed,
            check_connection,
            add_job,
            convert,
            add_all_jobs,
            cancel_job,
            cancel_all_jobs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
