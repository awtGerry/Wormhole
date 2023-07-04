#![allow(unused)]
#![allow(unused_imports)]

use std::{
    fs,
    env,
    path::Path,
    path::PathBuf
};
use users::{get_user_by_uid, get_current_uid, os::unix::UserExt};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DirDetails {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub metadata: String,
}

fn is_dotfile(path: &Path) -> bool {
    let file = path.file_name().unwrap().to_str().unwrap();
    file.to_string().starts_with('.')
}

fn metadata(dir_path: &str) -> String {
    let mut result = String::new();
    match fs::metadata(dir_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                result = "file".to_string();
            } else if metadata.is_dir() {
                result = "dir".to_string()
            } else {
                result = "else".to_string()
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
    result
}

fn create_dir_details(dir_path: &str) -> DirDetails {
    let dir_name = Path::new(dir_path).file_name().unwrap().to_str().unwrap();
    let size = fs::metadata(dir_path).unwrap().len();
    let metadata = metadata(dir_path);
    DirDetails {
        path: dir_path.to_string(),
        name: dir_name.to_string(),
        size,
        metadata,
    }
}

pub fn move_to(dir_path: &str) -> Vec<DirDetails> {
    let mut result: Vec<DirDetails> = Vec::new();
    match fs::read_dir(dir_path) {
        Ok(dir) => {
            for entry in dir {
                let entry = entry.unwrap();
                let path = entry.path();
                if !is_dotfile(&path) {
                    let dir_details = create_dir_details(path.to_str().unwrap());
                    result.push(dir_details);
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
    set_current_dir(dir_path);
    result
}

fn set_current_dir(dir_path: &str) {
    match std::env::set_current_dir(dir_path) {
        Ok(_) => {}
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

pub fn home_file() -> Vec<DirDetails> {
    let mut result: Vec<DirDetails> = Vec::new();
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let dir_path = user.home_dir().to_str().unwrap();
    move_to(dir_path)
}

pub fn create_dir(dirname: &str) {
    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.display());
    let dir_path = current_dir.join(dirname);
    println!("Directory path: {}", dir_path.display());
    match fs::create_dir(dir_path) {
        Ok(_) => {}
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

pub fn create_file(filename: &str) {
    let current_dir = std::env::current_dir().unwrap();
    let file_path = current_dir.join(filename);
    match fs::File::create(file_path) {
        Ok(_) => {
            println!("File created successfully");
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

pub fn go_back() -> Vec<DirDetails> {
    let mut result: Vec<DirDetails> = Vec::new();
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let previous_dir = current_dir
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"));
    move_to(previous_dir.to_str().unwrap())
}
