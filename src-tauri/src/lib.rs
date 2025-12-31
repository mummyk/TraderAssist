mod commands;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
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
