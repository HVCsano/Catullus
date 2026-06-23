use tauri::AppHandle;

#[tauri::command]
pub async fn generate_full_cache(app: AppHandle) {}
