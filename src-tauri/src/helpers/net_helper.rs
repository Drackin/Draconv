#[tauri::command]
pub async fn check_connection() -> bool {
    let response = reqwest::get("https://www.gstatic.com/generate_204");

    if response.await.is_ok() {
        return true
    }

    false
}