// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// call files.rs
mod files;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn home_dir() -> Vec<String> {
    files::read_home_dir()
    // format!("{}", res)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![home_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
