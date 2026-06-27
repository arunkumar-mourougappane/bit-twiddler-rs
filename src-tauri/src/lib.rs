mod commands;

use commands::{cron, crypto, dialog, markdown, qr, sql, yaml};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            crypto::generate_hashes,
            crypto::hash_file,
            crypto::stop_file_watch,
            qr::generate_qr,
            markdown::render_markdown,
            yaml::convert_yaml,
            sql::format_sql,
            cron::describe_cron,
            dialog::open_file_dialog,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
