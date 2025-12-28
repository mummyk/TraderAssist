// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Testing my code
#[tauri::command]
fn list_file(path: &str) -> Vec<String> {
    let path = std::path::Path::new(path); // Make sure to import std::path::Path or use this

    path.read_dir()
        .unwrap()
        .map(|entries| entries.unwrap().file_name().to_str().unwrap().to_owned())
        .collect::<Vec<String>>() // Fixed here
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
