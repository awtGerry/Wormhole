use std::fs;
use std::path::Path;
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
    result
}

pub fn file() -> Vec<DirDetails> {
    let mut result: Vec<DirDetails> = Vec::new();
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let dir_path = user.home_dir().to_str().unwrap();
    let dir = fs::read_dir(dir_path).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if !is_dotfile(&path) {
            let dir_details = create_dir_details(path.to_str().unwrap());
            result.push(dir_details);
        }
    }
    result
}
