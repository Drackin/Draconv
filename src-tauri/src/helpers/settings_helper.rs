use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, Manager};

use crate::{helpers::queue_helper::PipelineManager, tools::app_handle::app};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub conversion_mode: String,
    pub max_concurrency: usize,
    pub open_when_finished: bool,
    pub default_encoder: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            conversion_mode: "normal".into(),
            max_concurrency: 1,
            open_when_finished: true,
            default_encoder: "libx265".into(),
        }
    }
}

fn get_settings_path() -> PathBuf {
    let settings_path = app()
        .path()
        .resolve("settings/default.json", BaseDirectory::AppLocalData)
        .expect("error resolving settings file");

    if let Some(parent) = settings_path.parent() {
        if !parent.exists() || !settings_path.exists() {
            let default = Settings::default();
            let json = serde_json::to_string_pretty(&default).expect("error serializing settings");
            fs::create_dir_all(parent).expect("error creating settings directory");
            let settings_clone = settings_path.clone();
            fs::write(settings_clone, json).expect("error creating settings file");
            let settings_clone = settings_path.clone();

            return settings_clone;
        }
    }

    settings_path
}

#[tauri::command]
pub fn reset_settings() {
    let path = get_settings_path();
    let default = Settings::default();
    let json = serde_json::to_string_pretty(&default).expect("error serializing settings");
    fs::write(path, json).expect("error writing settings file");
}

#[tauri::command]
pub fn load_settings() -> Settings {
    let path = get_settings_path();

    let data = fs::read_to_string(&path).unwrap_or_else(|_| "{}".to_string());

    let settings = serde_json::from_str(&data).unwrap_or_default();

    let new_json = serde_json::to_string_pretty(&settings).expect("error serializing settings");

    // have to rewrite the settings file if it was missing fields
    if data.trim() != new_json.trim() {
        fs::write(&path, new_json).expect("error writing settings file");
    }

    settings
}

#[tauri::command]
pub async fn save_settings(new_settings: Settings) {
    let path = get_settings_path();
    let current_settings = load_settings();

    // Need to update pipeline manager if concurrency changed
    if new_settings.max_concurrency != current_settings.max_concurrency {
        let pipeline_manager: tauri::State<PipelineManager> = app().state();
        pipeline_manager
            .set_concurrency(new_settings.max_concurrency)
            .await;
    }

    let json = serde_json::to_string_pretty(&new_settings).expect("error serializing settings");

    fs::write(path, json).expect("error writing settings file");
}
