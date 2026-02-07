use crate::helpers::{gpu_helper::select_best_encoder, settings_helper::load_settings};
use std::{path::PathBuf, process::Command};

#[derive(Debug)]
pub enum Encoder {
    CPU,
    NVENC,
    AMF,
    QSV,
    APPLE,
}

struct CodecProfile {
    video: String,
    audio: &'static str,
    disable_video: bool,
    hwaccel_supported: bool,
    arguments: Vec<String>,
}

fn get_codec_profile(format: &str) -> CodecProfile {
    let settings = load_settings();
    let default_enc = settings.default_encoder;

    match format {
        "mp4" => CodecProfile {
            video: default_enc,
            audio: "aac",
            disable_video: false,
            hwaccel_supported: true,
            arguments: vec![],
        },
        "webm" => CodecProfile {
            video: "libvpx-vp9".into(),
            audio: "libopus",
            disable_video: false,
            hwaccel_supported: false,
            arguments: vec![],
        },
        "mov" => CodecProfile {
            video: default_enc,
            audio: "aac",
            disable_video: false,
            hwaccel_supported: true,
            arguments: vec![],
        },
        "avi" => CodecProfile {
            video: "mpeg4".into(),
            audio: "mp3",
            disable_video: false,
            hwaccel_supported: false,
            arguments: vec!["-quality".into(), "quality".into()],
        },
        "mkv" => CodecProfile {
            video: default_enc,
            audio: "libopus",
            disable_video: false,
            hwaccel_supported: true,
            arguments: vec![],
        },
        "flv" => CodecProfile {
            video: "flv".into(),
            audio: "mp3",
            disable_video: false,
            hwaccel_supported: false,
            arguments: vec![],
        },
        "wmv" => CodecProfile {
            video: "msmpeg4".into(),
            audio: "wmav2",
            disable_video: false,
            hwaccel_supported: false,
            arguments: vec![],
        },

        "mp3" => CodecProfile {
            video: "none".into(),
            audio: "libmp3lame",
            disable_video: true,
            hwaccel_supported: false,
            arguments: vec![],
        },
        "aac" => CodecProfile {
            video: "none".into(),
            audio: "aac",
            disable_video: true,
            hwaccel_supported: false,
            arguments: vec![],
        },
        "flac" => CodecProfile {
            video: "none".into(),
            audio: "flac",
            disable_video: true,
            hwaccel_supported: false,
            arguments: vec![],
        },
        "wav" => CodecProfile {
            video: "none".into(),
            audio: "pcm_s16le",
            disable_video: true,
            hwaccel_supported: false,
            arguments: vec![],
        },
        "ogg" => CodecProfile {
            video: "none".into(),
            audio: "libopus",
            disable_video: true,
            hwaccel_supported: false,
            arguments: vec![],
        },

        _ => CodecProfile {
            video: default_enc,
            audio: "aac",
            disable_video: false,
            hwaccel_supported: true,
            arguments: vec![],
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

pub async fn ffmpeg_builder(input: &str, output_path: &str, output_format: &str) -> Vec<String> {
    let profile = get_codec_profile(output_format);
    let settings = load_settings();

    let lossless = settings.conversion_mode == "lossless";

    let (gpu_type, hw_accel_encoder, encoder_params, hw_accel_method) = select_best_encoder().await;

    let use_hw = !profile.disable_video
        && profile.hwaccel_supported
        && matches!(gpu_type, Encoder::NVENC | Encoder::AMF | Encoder::QSV);

    let mut cmd = vec![
        "-y".to_string(),
        "-progress".to_string(),
        "pipe:1".to_string(),
        "-nostats".to_string(),
        "-loglevel".to_string(),
        "error".to_string(),
    ];

    if use_hw && hw_accel_method != "none" {
        cmd.push("-hwaccel".to_string());
        cmd.push(hw_accel_method.clone());

        // Intel and Nvidia hwaccel methods need specific output formats
        if hw_accel_method == "cuda" {
            cmd.push("-hwaccel_output_format".to_string());
            cmd.push("cuda".to_string());
        }

        if hw_accel_method == "qsv" {
            cmd.push("-hwaccel_output_format".to_string());
            cmd.push("qsv".to_string());
        }
    }

    cmd.push("-i".to_string());
    cmd.push(input.to_string());

    if !profile.disable_video {
        let encoder = if use_hw {
            hw_accel_encoder
        } else {
            profile.video
        };

        cmd.push("-c:v".to_string());
        cmd.push(encoder);

        if use_hw {
            cmd.extend(encoder_params.iter().map(|s| s.to_string()));
        } else if lossless {
            let crf = if output_format == "webm" { "30" } else { "18" };
            cmd.extend(vec!["-crf".into(), crf.into()]);
        } else {
            cmd.extend(vec![
                "-crf".into(),
                "23".into(),
                "-preset".into(),
                "medium".into(),
            ]);
        }
    } else {
        cmd.push("-vn".to_string()); // if no video
    }
    cmd.extend(profile.arguments);

    // Audio codec
    cmd.push("-c:a".to_string());
    cmd.push(profile.audio.to_string());

    cmd.push(output_path.to_string());

    cmd
}
