// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let version = &app.package_info().version;
            window.set_title(&format!("SSMT4 v{}", version)).unwrap();
            window.set_size(tauri::Size::Logical(tauri::LogicalSize { width: 1000.0, height: 671.0 })).unwrap();
            window.set_resizable(true).unwrap();
            window.center().unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
