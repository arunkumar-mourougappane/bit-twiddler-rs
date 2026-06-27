use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn open_file_dialog(app_handle: tauri::AppHandle) -> Option<String> {
    let file = app_handle.dialog().file().blocking_pick_file();
    file.map(|p| p.to_string())
}
