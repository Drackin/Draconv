use std::vec;

use tauri::AppHandle;
use wgpu::{Backends, InstanceDescriptor, RequestAdapterOptions};

use super::{ffmpeg_helper::Encoder, settings_helper::load_settings};

async fn detect_gpu() -> Result<u32, String> {
    let instance = wgpu::Instance::new(&InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    });

    let adapter_option = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .map_err(|e| e.to_string())?;

    let info = adapter_option.get_info();
    Ok(info.vendor)
}

#[derive(Debug)]
pub enum GpuVendor {
    AMD = 0x1002,
    NVIDIA = 0x10DE,
    INTEL = 0x8086,
    UNKNOWN,
}

pub async fn get_vendor() -> GpuVendor {
    let gpu_vendor_id = detect_gpu().await;

    match gpu_vendor_id {
        Ok(vendor) => match vendor {
            0x1002 => GpuVendor::AMD,
            0x10DE => GpuVendor::NVIDIA,
            0x8086 => GpuVendor::INTEL,
            _ => GpuVendor::UNKNOWN,
        },
        Err(e) => {
            eprintln!("GPU detection error: {}", e);
            GpuVendor::UNKNOWN
        }
    }
}

pub async fn select_best_encoder(app: &AppHandle) -> (Encoder, Vec<&'static str>) {
    let settings = load_settings(app.to_owned());

    if settings.conversion_mode == "lossless" {
        return (Encoder::CPU, vec![]);
    }

    if settings.conversion_mode != "hwaccel" {
        return (Encoder::CPU, vec![]);
    }

    let vendor_id = get_vendor().await;

    match vendor_id {
        GpuVendor::AMD => (Encoder::AMF, vec!["-quality", "quality"]), // AMD AMF: lossless and speed balance
        GpuVendor::NVIDIA => (Encoder::NVENC, vec!["-preset", "llhq"]), // NVIDIA NVENC: llhq = low-latency high quality
        GpuVendor::INTEL => (Encoder::QSV, vec!["-global_quality", "51"]), // QSV: best quality possible
        GpuVendor::UNKNOWN => (Encoder::CPU, vec![]),
    }
}
