#[tauri::command]
pub fn convert_yaml(input: String, mode: String) -> Result<String, String> {
    match mode.as_str() {
        "yaml-to-json" => {
            let value: serde_yaml::Value =
                serde_yaml::from_str(&input).map_err(|e| e.to_string())?;
            serde_json::to_string_pretty(&value).map_err(|e| e.to_string())
        }
        "json-to-yaml" => {
            let value: serde_json::Value =
                serde_json::from_str(&input).map_err(|e| e.to_string())?;
            let yaml_value: serde_yaml::Value =
                serde_json::from_value(value).map_err(|e| e.to_string())?;
            serde_yaml::to_string(&yaml_value).map_err(|e| e.to_string())
        }
        _ => Err("Invalid mode. Use 'yaml-to-json' or 'json-to-yaml'.".to_string()),
    }
}
