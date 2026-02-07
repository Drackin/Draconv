use std::sync::OnceLock;
use tauri::AppHandle;

// Have to use a global app handle to avoid lifetime issues with Tauri's AppHandle
// such as passing `app: AppHandle` to every function that needs it.

static GLOBAL_APP: OnceLock<AppHandle> = OnceLock::new();

pub fn init_global_app(app: AppHandle) {
    GLOBAL_APP
        .set(app)
        .expect("AppHandle is already initialized");
}

pub fn app() -> &'static AppHandle {
    GLOBAL_APP.get().expect("AppHandle is not initialized")
}
