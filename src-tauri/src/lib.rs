mod commands;
use tauri_plugin_sql::{Migration, MigrationKind};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create_settings_table",
        sql: "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:settings.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::process_chart_folder,
            commands::get_available_symbols,
            commands::get_symbol_data,
            commands::delete_symbol,
            commands::rename_symbol,
            greet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
