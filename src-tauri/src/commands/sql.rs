use sqlformat::{format, FormatOptions, Indent, QueryParams};

#[tauri::command]
pub fn format_sql(sql: String, _language: String) -> Result<String, String> {
    let options = FormatOptions {
        indent: Indent::Spaces(2),
        uppercase: Some(true),
        lines_between_queries: 2,
        ..FormatOptions::default()
    };
    Ok(format(&sql, &QueryParams::None, &options))
}
