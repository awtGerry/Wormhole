#![allow(unused)]
#![allow(unused_imports)]

use std::{fs, fs::File, path::Path};
use std::{env, error};
use users::{get_user_by_uid, get_current_uid, os::unix::UserExt};

// TODO: solve this errors
use serde::{ser::Serializer, Deserialize, Serialize};

// #[derive(Debug, thiserror::Error)]
// pub enum Error {
//     #[error(transparent)]
//     Io(#[from] std::io::Error),
// }

// impl Serialize for Error {
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where S: Serializer {
//         serializer.serialize_str(self.to_string().as_ref())
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Dir {
    pub name: String,
    pub path: String,
    pub metadata: String,
}

fn dir_stats(dir_path: &str) -> Dir {
    let path = Path::new(dir_path);
    let _type = metadata(dir_path);
    let _name = path.file_name().unwrap().to_str().unwrap();
    let dir = Dir {
        name: _name.to_string(),
        path: dir_path.to_string(),
        metadata: _type,
    };
    dir
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

pub fn create_file(name: &str) -> bool {
    let file = File::create(name);
    match file {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn create_folder(name: &str) -> bool {
    let folder = std::fs::create_dir(name);
    match folder {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn move_to(dir_path: &str) -> Vec<String> {
    match std::fs::read_dir(dir_path) {
        Ok(paths) => {
            let mut result = Vec::new();
            for path in paths {
                match path {
                    Ok(path) => {
                        let path = path.path();
                        if !is_dotfile(&path) {
                            let path = path.to_str().unwrap();
                            result.push(path.to_string());
                        }
                    }
                    Err(_) => {}
                }
            }
            result
        }
        Err(_) => Vec::new(),
    }
}

// pub fn read_home_dir() -> Vec<Dir> {
//     let user = get_user_by_uid(get_current_uid()).unwrap();
//     match std::fs::read_dir(user.home_dir()) {
//         Ok(paths) => {
//             let mut result = Vec::new();
//             for path in paths {
//                 match path {
//                     Ok(path) => {
//                         let path = path.path();
//                         if !is_dotfile(&path) {
//                             let path = path.to_str().unwrap();
//                             let dir = dir_stats(path);
//                             result.push(dir);
//                         }
//                     }
//                     Err(_) => {}
//                 }
//             }
//             result
//         }
//         Err(_) => Vec::new(),
//     }
// }

pub fn read_home_dir() -> Vec<String> {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    match std::fs::read_dir(user.home_dir()) {
        Ok(paths) => {
            let mut result = Vec::new();
            for path in paths {
                match path {
                    Ok(path) => {
                        let path = path.path();
                        if !is_dotfile(&path) {
                            let path = path.to_str().unwrap();
                            let _type = metadata(path);
                            result.push(path.to_string());
                        }
                    }
                    Err(_) => {}
                }
            }
            result
        }
        Err(_) => Vec::new(),
    }
}

pub fn go_back() -> Vec<String> {
    if let Some(parent) = Path::new(".").parent() {
        if let Err(e) = env::set_current_dir(parent) {
            println!("Error: {}", e);
            Vec::new()
        } else {
            let path = parent.to_str().unwrap();
            move_to(path)
        }
    } else {
        Vec::new()
    }
}
