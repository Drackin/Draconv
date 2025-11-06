use std::sync::OnceLock;
use tauri::AppHandle;

static GLOBAL_APP: OnceLock<AppHandle> = OnceLock::new();

pub fn init_global_app(app: AppHandle) {
    GLOBAL_APP.set(app).expect("AppHandle is already initialized");
}

pub fn app() -> &'static AppHandle {
    GLOBAL_APP.get().expect("AppHandle is not initialized")
}