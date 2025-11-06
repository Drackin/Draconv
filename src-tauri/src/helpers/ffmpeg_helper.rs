use std::{path::PathBuf, process::Command};

use crate::tools::app_handle::app;

use super::{gpu_helper::select_best_encoder, settings_helper::load_settings};

#[derive(Debug)]
pub enum Encoder {
    CPU,
    NVENC,
    AMF,
    QSV,
}

struct CodecProfile {
    video: &'static str,
    audio: &'static str,
    disable_video: bool,
    hwaccel_supported: bool,
}

fn get_codec_profile(format: &str) -> CodecProfile {
    match format {
        "mp4" => CodecProfile {
            video: "libx264",
            audio: "aac",
            disable_video: false,
            hwaccel_supported: true,
        },
        "webm" => CodecProfile {
            video: "libvpx-vp9",
            audio: "libopus",
            disable_video: false,
            hwaccel_supported: false,
        },
        "mov" => CodecProfile {
            video: "libx264",
            audio: "aac",
            disable_video: false,
            hwaccel_supported: true,
        },
        "avi" => CodecProfile {
            video: "mpeg4",
            audio: "mp3",
            disable_video: false,
            hwaccel_supported: true,
        },
        "mkv" => CodecProfile {
            video: "libx264",
            audio: "aac",
            disable_video: false,
            hwaccel_supported: true,
        },
        "flv" => CodecProfile {
            video: "flv",
            audio: "mp3",
            disable_video: false,
            hwaccel_supported: false,
        },
        "wmv" => CodecProfile {
            video: "msmpeg4",
            audio: "wmav2",
            disable_video: false,
            hwaccel_supported: false,
        },

        "mp3" => CodecProfile {
            video: "none",
            audio: "libmp3lame",
            disable_video: true,
            hwaccel_supported: false,
        },
        "aac" => CodecProfile {
            video: "none",
            audio: "aac",
            disable_video: true,
            hwaccel_supported: false,
        },
        "flac" => CodecProfile {
            video: "none",
            audio: "flac",
            disable_video: true,
            hwaccel_supported: false,
        },
        "wav" => CodecProfile {
            video: "none",
            audio: "pcm_s16le",
            disable_video: true,
            hwaccel_supported: false,
        },
        "ogg" => CodecProfile {
            video: "none",
            audio: "libopus",
            disable_video: true,
            hwaccel_supported: false,
        },

        _ => CodecProfile {
            video: "libx264",
            audio: "aac",
            disable_video: false,
            hwaccel_supported: true,
        },
    }
}

pub fn time_to_seconds(time_str: &str) -> f64 {
    let parts: Vec<&str> = time_str.split(":").collect();
    if parts.len() != 3 {
        return 0.0;
    }
    let hours = parts[0].parse::<f64>().unwrap_or(0.0);
    let minutes = parts[1].parse::<f64>().unwrap_or(0.0);
    let seconds = parts[2].parse::<f64>().unwrap_or(0.0);

    hours * 3600.0 + minutes * 60.0 + seconds
}

pub fn get_video_duration_millis(ffmpeg_path: PathBuf, path: &str) -> f64 {
    let duration_output = Command::new(&ffmpeg_path)
        .args(&["-i", path])
        .output()
        .map_err(|e| e.to_string())
        .unwrap();

    let duration_info = String::from_utf8_lossy(&duration_output.stderr);

    duration_info
        .lines()
        .find(|line| line.contains("Duration:"))
        .and_then(|line| {
            let parts: Vec<&str> = line.split(",").collect();
            let duration_part = parts[0].trim().split(" ").last().unwrap_or("00:00:00.0");
            return Some(time_to_seconds(duration_part));
        })
        .unwrap_or(0.0)
}

pub async fn ffmpeg_builder(
    input: &str,
    output_path: &str,
    output_format: &str,
) -> Vec<String> {
    let profile = get_codec_profile(output_format);
    let settings = load_settings(app().to_owned());

    let lossless = settings.conversion_mode == "lossless";

    let mut cmd = vec![
        "-progress".to_string(),
        "pipe:1".to_string(),
        "-nostats".to_string(),
        "-loglevel".to_string(),
        "error".to_string(),
        "-i".to_string(),
        input.to_string(),
    ];

    if !profile.disable_video {
        let (encoder, encoder_params) = select_best_encoder(app()).await;

        let encoder_str = match encoder {
            Encoder::CPU => profile.video,
            Encoder::NVENC => {
                if profile.hwaccel_supported {
                    "h264_nvenc"
                } else {
                    profile.video
                }
            }
            Encoder::AMF => {
                if profile.hwaccel_supported {
                    "h264_amf"
                } else {
                    profile.video
                }
            }
            Encoder::QSV => {
                if profile.hwaccel_supported {
                    "h264_qsv"
                } else {
                    profile.video
                }
            }
        };

        cmd.push("-c:v".to_string());
        cmd.push(encoder_str.to_string());

        // Lossless CPU params
        if lossless && matches!(encoder, Encoder::CPU) {
            cmd.push("-crf".to_string());
            cmd.push("0".to_string());
        }

        // HWAccel / GPU params
        if profile.hwaccel_supported {
            cmd.extend(encoder_params.iter().map(|s| s.to_string()));
        }
    } else {
        cmd.push("-vn".to_string()); // if no video
    }

    // Audio codec
    cmd.push("-c:a".to_string());
    cmd.push(profile.audio.to_string());

    cmd.push(output_path.to_string());

    cmd
}