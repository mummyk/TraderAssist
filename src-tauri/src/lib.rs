mod upload_function;

use tauri_plugin_sql::{Migration, MigrationKind};
use upload_function::{
    delete_symbol, fetch_github_data_command, fetch_yfinance_data_command, get_available_symbols,
    get_symbol_data, process_chart_folder, rename_symbol,
};

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
            // Local folder commands
            process_chart_folder,
            get_available_symbols,
            get_symbol_data,
            delete_symbol,
            rename_symbol,
            // yFinance commands
            fetch_yfinance_data_command,
            // GitHub commands
            fetch_github_data_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
