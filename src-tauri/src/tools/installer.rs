use futures_util::StreamExt;
use reqwest::Client;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{cmp::min, io::Write};
#[cfg(unix)]
use tar::Archive;
use tauri::{Emitter, Manager};
use thiserror::Error;
#[cfg(unix)]
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
    } else if cfg!(target_arch = "aarch64") {
        "https://github.com/eugeneware/ffmpeg-static/releases/download/b6.1.1/ffmpeg-darwin-arm64"
    } else {
        "https://github.com/eugeneware/ffmpeg-static/releases/download/b6.1.1/ffmpeg-darwin-x64"
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
    let res = client.get(url).send().await?.error_for_status()?;

    // Some mirrors (including the macOS endpoint) use chunked transfer encoding,
    // so a response is allowed to omit Content-Length.
    let total_size = res.content_length().filter(|size| *size > 0);

    match total_size {
        Some(size) => println!(
            "Starting ffmpeg download from {} ({:.2} MiB)",
            url,
            size as f64 / (1024.0 * 1024.0)
        ),
        None => println!("Starting ffmpeg download from {} (size unknown)", url),
    }

    app()
        .emit("ffmpeg-install-state", "downloading")
        .map_err(|error| ffmpeg_err!(format!("Could not update installer state: {error}")))?;

    let ffmpeg_dir = app()
        .path()
        .app_local_data_dir()
        .map_err(|error| ffmpeg_err!(format!("Error resolving app data directory: {error}")))?
        .join("bin");
    let down_path = if cfg!(target_os = "linux") {
        ffmpeg_dir.join("ffmpeg.tar.xz")
    } else if cfg!(target_os = "macos") {
        ffmpeg_dir.join("ffmpeg.download")
    } else {
        ffmpeg_dir.join("ffmpeg.zip")
    };

    if !ffmpeg_dir.exists() {
        fs::create_dir_all(&ffmpeg_dir)?;
    }

    let mut file = fs::File::create(down_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;

        file.write_all(&chunk)
            .map_err(|_| ffmpeg_err!("Error while downloading file."))?;

        let new = total_size
            .map(|size| min(downloaded + (chunk.len() as u64), size))
            .unwrap_or(downloaded + (chunk.len() as u64));

        downloaded = new;

        let progress = total_size
            .filter(|size| *size > 0)
            .map(|size| ((downloaded as f64 / size as f64) * 100.0).min(100.0) as i32)
            .unwrap_or(0);

        app()
            .emit("ffmpeg-install-progress", progress)
            .map_err(|error| ffmpeg_err!(format!("Could not update download progress: {error}")))?;
    }

    if total_size.is_none() {
        app()
            .emit("ffmpeg-install-progress", 100)
            .map_err(|error| ffmpeg_err!(format!("Could not finish download progress: {error}")))?;
    }

    setup_ffmpeg().await?;

    Ok(())
}

pub async fn setup_ffmpeg() -> Result<(), FFmpegError> {
    let ffmpeg_dir = app()
        .path()
        .app_local_data_dir()
        .map_err(|error| ffmpeg_err!(format!("Error resolving app data directory: {error}")))?
        .join("bin");

    app()
        .emit("ffmpeg-install-state", "installing")
        .map_err(|error| ffmpeg_err!(format!("Could not update installer state: {error}")))?;

    #[cfg(windows)]
    {
        let archive = ffmpeg_dir.join("ffmpeg.zip");

        let file = fs::File::open(archive)?;

        let mut zip_archive = zip::ZipArchive::new(&file)?;

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
            let archive_file = fs::File::open(archive)?;

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
            let downloaded = ffmpeg_dir.join("ffmpeg.download");
            fs::rename(downloaded, &new_file)?;
        }

        let metadata = fs::metadata(&new_file)?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755); // executable permission
        fs::set_permissions(&new_file, perms)?;
    };

    clean_installation()
}

pub fn clean_installation() -> Result<(), FFmpegError> {
    let ffmpeg_dir = app()
        .path()
        .app_local_data_dir()
        .map_err(|error| ffmpeg_err!(format!("Error resolving app data directory: {error}")))?
        .join("bin");

    app()
        .emit("ffmpeg-install-state", "cleaning")
        .map_err(|error| ffmpeg_err!(format!("Could not update installer state: {error}")))?;

    let ffmpeg_archive = if cfg!(target_os = "linux") {
        ffmpeg_dir.join("ffmpeg.tar.xz")
    } else if cfg!(target_os = "macos") {
        ffmpeg_dir.join("ffmpeg.download")
    } else {
        ffmpeg_dir.join("ffmpeg.zip")
    };

    if ffmpeg_archive.exists() {
        fs::remove_file(ffmpeg_archive)?;
    }
    #[cfg(target_os = "windows")]
    if ffmpeg_dir.join("ffmpeg-master-latest-win64-gpl").exists() {
        fs::remove_dir_all(ffmpeg_dir.join("ffmpeg-master-latest-win64-gpl"))?;
    }
    #[cfg(target_os = "linux")]
    if ffmpeg_dir.join("ffmpeg-master-latest-linux64-gpl").exists() {
        fs::remove_dir_all(ffmpeg_dir.join("ffmpeg-master-latest-linux64-gpl"))?;
    }

    app()
        .emit("ffmpeg-install-state", "completed")
        .map_err(|error| ffmpeg_err!(format!("Could not update installer state: {error}")))?;

    Ok(())
}
