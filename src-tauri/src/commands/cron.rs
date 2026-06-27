use cron_descriptor::cronparser::cron_expression_descriptor::get_description_cron;

#[tauri::command]
pub fn describe_cron(expression: String) -> Result<String, String> {
    get_description_cron(&expression).map_err(|e| e.s)
}
