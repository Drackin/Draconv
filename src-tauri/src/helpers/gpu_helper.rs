use std::vec;
use wgpu::{Backends, InstanceDescriptor, RequestAdapterOptions};

use crate::helpers::{ffmpeg_helper::Encoder, settings_helper::load_settings};

async fn detect_gpu() -> Result<u32, String> {
    let instance = wgpu::Instance::new(&InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    });

    let adapter_option = instance
        .request_adapter(&RequestAdapterOptions {
            // Request high-performance GPU if available (discrete GPU)
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            // but if not, fallback to CPU or integrated GPU
            force_fallback_adapter: false,
        })
        .await
        .map_err(|e| e.to_string())?;

    let info = adapter_option.get_info();
    Ok(info.vendor)
}

// GPU Vendor IDs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GpuVendor {
    AMD = 0x1002,
    NVIDIA = 0x10DE,
    INTEL = 0x8086,
    APPLE = 0x106B,
    UNKNOWN,
}

// I realized that decoding also benefits from hardware acceleration, so this function maps GPU vendors to their respective hardware acceleration methods.
pub fn get_hw_accel_method(vendor: GpuVendor) -> &'static str {
    // return "videotoolbox" whatever the GPU vendor is on macOS since it's the only option available
    if cfg!(target_os = "macos") {
        return "videotoolbox";
    }

    match vendor {
        GpuVendor::AMD => "amf",
        GpuVendor::NVIDIA => "cuda",
        GpuVendor::INTEL => "qsv",
        _ => "auto",
    }
}

// Get GPU vendor
pub async fn get_vendor() -> GpuVendor {
    let gpu_vendor_id = detect_gpu().await;

    match gpu_vendor_id {
        Ok(vendor) => match vendor {
            0x1002 => GpuVendor::AMD,
            0x10DE => GpuVendor::NVIDIA,
            0x8086 => GpuVendor::INTEL,
            0x106B => GpuVendor::APPLE,
            _ => GpuVendor::UNKNOWN,
        },
        Err(_) => GpuVendor::UNKNOWN,
    }
}

// Map base codec to GPU hardware encoder based on vendor
pub fn get_gpu_hw_encoder(base_codec: &str, vendor: GpuVendor) -> Option<&'static str> {

    if cfg!(target_os = "macos") {
        return match base_codec {
            "libx264" => Some("h264_videotoolbox"),
            "libx265" => Some("hevc_videotoolbox"),
            _ => None,
        }
    }
    /*
        LIBX264 -> H264 (MPEG-4 AVC etc.)
        LIBX265 -> Better compression, bit efficiency, color depth than H.264
        LIBVPX-VP9 -> WEBM format, better compression than H.264
        LIBSVTAV1 -> AV1; next-gen codec, better compression than VP9 and H.265
    */
    match (base_codec, vendor) {
        // NVIDIA
        ("libx264", GpuVendor::NVIDIA) => Some("h264_nvenc"),
        ("libx265", GpuVendor::NVIDIA) => Some("hevc_nvenc"),
        ("libvpx-vp9", GpuVendor::NVIDIA) => Some("vp9_vaapi"),
        ("libsvtav1", GpuVendor::NVIDIA) => Some("av1_nvenc"),

        // INTEL
        ("libx264", GpuVendor::INTEL) => Some("h264_qsv"),
        ("libx265", GpuVendor::INTEL) => Some("hevc_qsv"),
        ("libvpx-vp9", GpuVendor::INTEL) => Some("vp9_qsv"),
        ("libsvtav1", GpuVendor::INTEL) => Some("av1_qsv"),

        // AMD
        ("libx264", GpuVendor::AMD) => Some("h264_amf"),
        ("libx265", GpuVendor::AMD) => Some("hevc_amf"),
        ("libvpx-vp9", GpuVendor::AMD) => Some("vp9_vaapi"),
        ("libsvtav1", GpuVendor::AMD) => Some("av1_amf"),

        _ => None,
    }
}

pub async fn select_best_encoder() -> (Encoder, String, Vec<String>, String) {
    let mut settings = load_settings();

    if settings.conversion_mode == "lossless" || settings.conversion_mode != "hwaccel" {
        return (
            Encoder::CPU,
            settings.default_encoder.clone(),
            vec![],
            "none".into(),
        );
    }

    let mut vendor_id = get_vendor().await;
    if cfg!(target_os = "macos") {
        vendor_id = GpuVendor::APPLE;
    }

    let hw_accel_method = get_hw_accel_method(vendor_id);

    if cfg!(target_os = "macos") && settings.default_encoder == "libsvtav1" {
        settings.default_encoder = "libx265".into();
    }

    if let Some(encoder_name) = get_gpu_hw_encoder(settings.default_encoder.as_str(), vendor_id) {
        match vendor_id {
            GpuVendor::AMD => (
                Encoder::AMF,
                encoder_name.to_string(),
                vec![
                    "-usage".into(),
                    "transcoding".into(),
                    "-quality".into(),
                    "balanced".into(),
                    "-rc".into(),
                    "cqp".into(),
                ],
                hw_accel_method.to_string(),
            ), // AMD AMF
            GpuVendor::NVIDIA => (
                Encoder::NVENC,
                encoder_name.to_string(),
                vec![
                    "-preset".into(),
                    "p4".into(),
                    "-cq".into(),
                    "23".into(),
                    "-rc".into(),
                    "vbr".into(),
                    "-spatial-aq".into(),
                    "1".into(), // Affects visual quality, speed loss is minimal
                ],
                hw_accel_method.to_string(),
            ), // NVIDIA NVENC
            GpuVendor::INTEL => (
                Encoder::QSV,
                encoder_name.to_string(),
                vec![
                    "-preset".into(),
                    "medium".into(),
                    "-global_quality".into(),
                    "23".into(),
                    "-look_ahead".into(),
                    "0".into(), // 0 increases speed
                ],
                hw_accel_method.to_string(),
            ), // INTEL QSV


            GpuVendor::APPLE => {
                let mut mac_args: Vec<String> = vec![
                    "-allow_sw".into(), // Allow software fallback if hardware encoder fails
                    "1".into(),
                ];

                // APPLE SILICON KONTROLÜ
                if cfg!(target_arch = "aarch64") {
                    mac_args.push("-q:v".into());
                    mac_args.push("65".into()); // CQ mode on Apple Silicon
                } else {
                    mac_args.push("-b:v".into());
                    mac_args.push("6M".into()); // Bitrate mode for Intel Macs
                }

                (
                    Encoder::APPLE,
                    encoder_name.to_string(),
                    mac_args,
                    hw_accel_method.to_string(),
                )
            },
            
            GpuVendor::UNKNOWN => (
                Encoder::CPU,
                settings.default_encoder,
                vec![],
                "none".into(),
            ),
        }
    } else {
        (
            Encoder::CPU,
            settings.default_encoder,
            vec![],
            "none".into(),
        )
    }
}
