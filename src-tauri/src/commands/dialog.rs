use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn open_file_dialog(app_handle: tauri::AppHandle) -> Option<String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    app_handle.dialog().file().pick_file(move |path| {
        let _ = tx.send(path);
    });
    rx.await.ok().flatten().map(|p| p.to_string())
}
