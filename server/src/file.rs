use std::collections::HashMap;
use std::fs::read_dir;
use std::io::{Error as IoError, ErrorKind};
use std::path::{Path, PathBuf};
use std::{env, fs};
use std::error::Error;


/// Loads all static files into memory
pub fn load_static_files(dir: &Path) -> Result<HashMap<String, Vec<u8>>, String> {
    let provided_directory = load_directory(Path::new(&dir))?;
    let static_respources = load_static_resources()?;

    let static_files = provided_directory.into_iter().chain(static_respources).collect();

    Ok(static_files)
}

/// Loads the files from the dynamic resources directory into memory
pub fn load_dynamic_files() -> Result<HashMap<String, String>, String> {
    let current_dir = get_current_dir()?;
    let resources_dir = PathBuf::from(string_from_path(current_dir)? + "/resources/templates");

    let mut result = HashMap::new();

    let files = match fs::read_dir(&resources_dir) {
        Ok(files) => files,
        Err(_) => return Err("Could not read dynamic files from resources. Make sure the directory lays in the current directory".to_string())
    };

    for entry in files {
        let entry = match entry {
            Ok(entry) => entry.path(),
            Err(_) => continue
        };

        if entry.is_file() {
            let file_name = match entry.file_name() {
                Some(file_name) => {
                    match file_name.to_str() {
                        Some(file_name) => format!("/{}", file_name),
                        None => continue
                    }
                }
                None => continue
            };

            match fs::read_to_string(&entry) {
                Ok(file_content) => result.insert(file_name, file_content),
                Err(_) => continue
            };
        }
    }
    Ok(result)
}

/// Loads the files in the path from the filesystem into memory
fn load_directory(path: &Path) -> Result<HashMap<String, Vec<u8>>, String> {
    return match get_all_files_in_dir(path) {
        Err(error) => {
            let err = format!("Could not read files in path {}: {}", path.display(), error);
            Err(err)
        }
        Ok(list) => Ok(list)
    };
}

/// Loads the files from the static resources directory into memory
fn load_static_resources() -> Result<HashMap<String, Vec<u8>>, String> {
    let current_dir = get_current_dir()?;
    match load_directory(&PathBuf::from(string_from_path(current_dir)? + "/resources/static")) {
        Ok(ok) => return Ok(ok),
        Err(err) => return Err("Make sure the resources folder exists in the current directory".to_string())
    }
}

/// Get all files in a directory.
///
/// Returns a map consisting of all the files in the root level of the provided dir
/// More specific return a hashmap containing the filename as key and the file content as utf-8 value
fn get_all_files_in_dir(path: &Path) -> Result<HashMap<String, Vec<u8>>, IoError> {
    if path.is_dir() {
        read_directory_rec(path)
    } else {
        let mut result = HashMap::with_capacity(1);
        let file = read_file(path)?;
        result.insert(format!("/{}", file.0), file.1);
        Ok(result)
    }
}

/// Reads a specific file recursively from the path into Memory
fn read_directory_rec(path: &Path) -> Result<HashMap<String, Vec<u8>>, IoError> {
    let mut result = HashMap::with_capacity(8);

    for entry in read_dir(path)? {
        let file = entry?.path();
        if file.is_file() {
            let file = read_file(&file)?;
            result.insert(format!("/{}", file.0), file.1);
        } else {
            let map = read_directory_rec(&file)?;
            for f in map {
                result.insert(format!("/{}{}", file_or_dir_name(&file)?, f.0), f.1);
            }
        }
    }
    Ok(result)
}

/// Reads a specific file from the path into Memory
fn read_file(file: &Path) -> Result<(String, Vec<u8>), IoError> {
    let filename = file_or_dir_name(file)?;
    let file_content = std::fs::read(file)?;
    Ok((filename, file_content))
}

/// Determines wheter the file is a file or directory
fn file_or_dir_name(file: &Path) -> Result<String, IoError> {
    Ok(file
        .file_name()
        .ok_or(IoError::new(ErrorKind::Other, "Filename no valid os string"))?
        .to_str()
        .ok_or(IoError::new(ErrorKind::Other, "Filename is no valid utf-8"))?
        .to_string())
}

/// Converts a PathBuf into a String
fn string_from_path(path: PathBuf) -> Result<String, String> {
    match path.into_os_string().into_string() {
        Ok(path) => Ok(path),
        Err(_) => Err("Could not convert path to string.".to_string())
    }
}

/// Returns the current directory as a String
fn get_current_dir() -> Result<PathBuf, String> {
    match env::current_dir() {
        Ok(current_dir) => Ok(current_dir),
        Err(_) => Err("Could not get current directory.".to_string())
    }
}

