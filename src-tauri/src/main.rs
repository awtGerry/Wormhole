// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use Wormhole::test;

// call files.rs
mod files;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn home_dir() -> Vec<test::DirDetails> {
    test::file()
}

#[tauri::command]
fn open_dir(dirname: &str) -> Vec<String> {
    files::move_to(dirname)
}

#[tauri::command]
fn go_back(current_path: &str) -> Vec<String> {
    files::go_back(current_path)
}

#[tauri::command]
fn send_dirs() -> Vec<test::DirDetails> {
    test::file()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
                        home_dir,
                        open_dir,
                        go_back,
                        send_dirs,
                        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
