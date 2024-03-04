// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![shutdown_system, init_default_os_behavior])
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

#[tauri::command]
fn init_default_os_behavior() {
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .args(["/C", "start", "explorer.exe"])
        .spawn()
        .expect("failed to execute process");

    #[cfg(not(target_os = "windows"))]
    std::process::Command::new("open")
        .args(["-a", "Finder"])
        .spawn()
        .expect("failed to execute process");
}