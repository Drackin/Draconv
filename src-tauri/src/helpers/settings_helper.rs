use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
// Make sure you have AppLocalData in the use statement
use tauri::{path::BaseDirectory, AppHandle, Manager};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub conversion_mode: String,
    pub open_when_finished: bool,
}

fn get_settings_path(app: &AppHandle) -> PathBuf {
    let settings_path = app
        .path()
        .resolve("settings/default.json", BaseDirectory::Resource)
        .expect("error resolving settings file");

    if let Some(parent) = settings_path.parent() {
        if !parent.exists() {
            let default = Settings {
                conversion_mode: "normal".into(),
                open_when_finished: true,
            };
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
pub fn load_settings(app: AppHandle) -> Settings {
    let path = get_settings_path(&app);

    let data = fs::read_to_string(path).expect("error reading settings file");

    serde_json::from_str(&data).expect("invalid json")
}

#[tauri::command]
pub fn save_settings(new_settings: Settings, app: AppHandle) {
    let path = get_settings_path(&app);

    let json = serde_json::to_string_pretty(&new_settings).expect("error serializing settings");

    fs::write(path, json).expect("error writing settings file");
}
