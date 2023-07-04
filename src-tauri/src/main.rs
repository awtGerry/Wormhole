// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use Wormhole::test;
use Wormhole::files;

// call files.rs
// mod files;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn home_dir() -> Vec<files::DirDetails> {
    files::home_file()
}

#[tauri::command]
fn open_dir(dirname: &str) -> Vec<files::DirDetails> {
    files::move_to(dirname)
}

#[tauri::command]
fn create_dir(dirname: &str) {
    files::create_dir(dirname)
}

#[tauri::command]
fn create_file(filename: &str) {
    files::create_file(filename)
}

#[tauri::command]
fn go_back() -> Vec<files::DirDetails> {
    files::go_back()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
                        home_dir,
                        open_dir,
                        create_dir,
                        create_file,
                        go_back,
                        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
