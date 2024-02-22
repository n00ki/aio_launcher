// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![shutdown_system])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// This function will be called from the JavaScript side to shutdown the operating system
#[tauri::command]
fn shutdown_system() {
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .args(["/C", "shutdown", "/s", "/t", "0"])
        .spawn()
        .expect("failed to execute process");

    #[cfg(not(target_os = "windows"))]
    std::process::Command::new("shutdown")
        .args(["-h", "now"])
        .spawn()
        .expect("failed to execute process");
}
