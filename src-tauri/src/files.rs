#![allow(unused)]
#![allow(unused_imports)]

use std::{fs, fs::File, path::Path};
use std::env;
use users::{get_user_by_uid, get_current_uid, os::unix::UserExt};

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

fn is_dotfile(path: &Path) -> bool {
    let file = path.file_name().unwrap().to_str().unwrap();
    file.to_string().starts_with('.')
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

// pub fn move_to(file_name: &str) -> Vec<String> {
//     // let user = get_user_by_uid(get_current_uid()).unwrap();
//     // let dir = user.home_dir().join(file_name);
//     let mut result = Vec::new();
//     match env::set_current_dir(file_name) {
//         Ok(_) => {
//             // result = file_name.to_string();
//             let last = Path::new(file_name).file_name().unwrap().to_str().unwrap();
//             println!("File name: {}", last);
//             create_file("test.txt");
//         }
//         Err(_) => {
//             result = "File not found".to_string();
//         }
//     }
//     result.to_string()
// }

pub fn move_to(dir_path: &str) -> Vec<String> {
    // let user = get_user_by_uid(get_current_uid()).unwrap();
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
