#![allow(unused)]
#![allow(unused_imports)]

use std::fs::File;
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
                        let path = path.to_str().unwrap();
                        result.push(path.to_string());
                    }
                    Err(_) => {}
                }
            }
            result
        }
        Err(_) => Vec::new(),
    }
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

pub fn move_to(file_name: &str) {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let dir = user.home_dir().join(file_name);
    match env::set_current_dir(dir) {
        Ok(_) => {
            println!("File opened: {}", file_name);
            create_file("test.txt");
        }
        Err(_) => {
            println!("File not found");
        }
    }
}
