use futures_util::StreamExt;
use reqwest::Client;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{cmp::min, io::Write};
#[cfg(target_os = "linux")]
use tar::Archive;
use tauri::{Emitter, Manager};
use thiserror::Error;
#[cfg(target_os = "linux")]
use xz::read::XzDecoder;

#[derive(Debug, Error)]
pub enum FFmpegError {
    #[error("[FFmpeg Install] IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("[FFmpeg Install] Request Error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("[FFmpeg Install] Archive Error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("[FFmpeg Install] Error: {0}")]
    Other(String),
}

macro_rules! ffmpeg_err {
    ($msg:expr) => {
        FFmpegError::Other($msg.to_string())
    };
}

use crate::tools::app_handle::app;

#[tauri::command]
pub fn is_ffmpeg_installed() -> bool {
    app()
        .path()
        .app_local_data_dir()
        .ok()
        .map(|dir| {
            let ffmpeg_path = if cfg!(target_os = "windows") {
                dir.join("bin/ffmpeg.exe")
            } else {
                dir.join("bin/ffmpeg")
            };
            ffmpeg_path.exists()
        })
        .unwrap_or(false)
}

fn ffmpeg_url() -> &'static str {
    if cfg!(target_os = "windows") {
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip"
    } else if cfg!(target_os = "linux") {
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz"
    } else {
        "https://evermeet.cx/ffmpeg/getrelease/zip"
    }
}

#[tauri::command]
pub async fn install_ffmpeg() -> Result<(), String> {
    match start_install_ffmpeg().await {
        Ok(_) => Ok(()),
        Err(e) => {
            app().emit("ffmpeg-failed", e.to_string()).ok();
            Err(e.to_string())
        }
    }
}

pub async fn start_install_ffmpeg() -> Result<(), FFmpegError> {
    let url = ffmpeg_url();

    let client = Client::new();
    let res = client.get(url).send().await?;

    let total_size = res.content_length().unwrap();

    println!(
        "Starting ffmpeg download from {}",
        (total_size as f64 / (1024.0 * 1024.0))
    );

    app().emit("ffmpeg-install-state", "downloading").unwrap();

    let ffmpeg_dir = app()
        .path()
        .app_local_data_dir()
        .expect("Error resolving resource dir")
        .join("bin");
    let down_path = if cfg!(target_os = "linux") {
        ffmpeg_dir.join("ffmpeg.tar.xz")
    } else {
        ffmpeg_dir.join("ffmpeg.zip")
    };

    if !ffmpeg_dir.exists() {
        fs::create_dir_all(ffmpeg_dir).expect("error creating bin directory");
    }

    let mut file = fs::File::create(down_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;

        file.write_all(&chunk)
            .map_err(|_| ffmpeg_err!("Error while downloading file."))?;

        let new = min(downloaded + (chunk.len() as u64), total_size);

        downloaded = new;

        let progress = ((downloaded as f64 / total_size as f64) * 100.0).min(100.0) as i32;

        app().emit("ffmpeg-install-progress", progress).unwrap()
    }

    if let Err(e) = setup_ffmpeg().await {
        app()
            .emit("ffmpeg-failed", format!("Failed to setup ffmpeg: {}", e))
            .unwrap()
    }

    Ok(())
}

pub async fn setup_ffmpeg() -> Result<(), FFmpegError> {
    let ffmpeg_dir = app()
        .path()
        .app_local_data_dir()
        .expect("Error resolving resource dir")
        .join("bin");

    app().emit("ffmpeg-install-state", "installing").unwrap();

    #[cfg(windows)]
    {
        let archive = ffmpeg_dir.join("ffmpeg.zip");

        let file = fs::File::open(archive)?;

        let mut zip_archive = zip::ZipArchive::new(&file).expect("Error reading ffmpeg archive");

        zip_archive.extract(&ffmpeg_dir)?;

        let extracted_dir = fs::read_dir(&ffmpeg_dir)?
            .find_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();

                if path.is_dir() && path.file_name()?.to_string_lossy().contains("ffmpeg") {
                    Some(path)
                } else {
                    None
                }
            })
            .ok_or_else(|| ffmpeg_err!("Could not find extracted ffmpeg folder"))?;
        let unpacked = extracted_dir.join("bin").join("ffmpeg.exe");
        let new_file = ffmpeg_dir.join("ffmpeg.exe");

        fs::copy(unpacked, &new_file)?;
    }

    #[cfg(unix)]
    {
        let new_file = ffmpeg_dir.join("ffmpeg");

        if cfg!(target_os = "linux") {
            let archive = ffmpeg_dir.join("ffmpeg.tar.xz");
            let archive_file = fs::File::open(archive).unwrap();

            let tar = XzDecoder::new(archive_file);
            let mut tar_xz = Archive::new(tar);

            tar_xz
                .unpack(&ffmpeg_dir)
                .map_err(|_| ffmpeg_err!("Error decompressing ffmpeg archive"))?;

            let unpacked = ffmpeg_dir
                .join("ffmpeg-master-latest-linux64-gpl")
                .join("bin")
                .join("ffmpeg");
            fs::copy(unpacked, &new_file)?;
        } else if cfg!(target_os = "macos") {
            let archive = ffmpeg_dir.join("ffmpeg.zip");

            let file = fs::File::open(archive)?;

            let mut zip_archive = zip::ZipArchive::new(&file)?;

            zip_archive.extract(&ffmpeg_dir)?;
        }

        let metadata = fs::metadata(&new_file)?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755); // executable permission
        fs::set_permissions(&new_file, perms).unwrap();
    };

    Ok(clean_installation())
}

pub fn clean_installation() {
    let ffmpeg_dir = app()
        .path()
        .app_local_data_dir()
        .expect("Error resolving resource dir")
        .join("bin");

    app().emit("ffmpeg-install-state", "cleaning").unwrap();

    let ffmpeg_archive = if cfg!(target_os = "linux") {
        ffmpeg_dir.join("ffmpeg.tar.xz")
    } else {
        ffmpeg_dir.join("ffmpeg.zip")
    };

    fs::remove_file(ffmpeg_archive).unwrap();
    #[cfg(target_os = "windows")]
    fs::remove_dir_all(ffmpeg_dir.join("ffmpeg-master-latest-win64-gpl")).unwrap();
    #[cfg(target_os = "linux")]
    fs::remove_dir_all(ffmpeg_dir.join("ffmpeg-master-latest-linux64-gpl")).unwrap();

    app().emit("ffmpeg-install-state", "completed").unwrap()
}
