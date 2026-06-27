#[tauri::command]
pub fn describe_cron(expression: String) -> Result<String, String> {
    cronspeak::describe(&expression).map_err(|e| e.to_string())
}
