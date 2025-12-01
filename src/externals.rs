use std::env;
use std::fs;
use std::path::Path;
use is_executable::IsExecutable;

fn get_paths() -> Vec<String> {
    let mut split_paths: Vec<String> = Vec::new();
    match env::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                split_paths.push(path.to_string_lossy().to_string());
            }
        }
        None => {}
    }
    return split_paths;
}

fn is_executable(path: & String) -> bool {
    let path = Path::new(path);
    return path.is_executable()
}

pub fn lookup(command: &String) -> Option<String> {
    let paths = get_paths();
    for path in paths{
        let command_path = format!("{}/{}", path, command);
        match fs::exists(&command_path){
            Ok(file_exist) => {
                if file_exist && is_executable(&command_path) {
                    return Some(command_path);
                }
                continue;
            },
            Err(_err) => {
                // println!("Failed to access {}: {}", command_path, err.to_string());
                continue;
            }
        };
    };
    None
}