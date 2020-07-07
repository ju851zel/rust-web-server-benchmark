use std::collections::HashMap;
use std::fs::read_dir;
use std::io::{Error as IoError, ErrorKind};
use std::path::{Path, PathBuf};
use std::{env, fs};

/// Get all files in a directory.
///
/// Returns a map consisting of all the files in the root level of the provided dir
/// More specific return a hashmap containing the filename as key and the file content as utf-8 value
pub fn get_all_files_in_dir(path: &Path) -> Result<HashMap<String, Vec<u8>>, IoError> {
    if path.is_dir() {
        read_directory(path)
    } else {
        let mut result = HashMap::with_capacity(1);
        let file = read_file(path)?;
        result.insert(format!("/{}", file.0), file.1);
        Ok(result)
    }
}

fn read_file(file: &Path) -> Result<(String, Vec<u8>), IoError> {
    let filename = file_or_dir_name(file)?;
    let file_content = std::fs::read(file)?;
    Ok((filename, file_content))
}

fn file_or_dir_name(file: &Path) -> Result<String, IoError> {
    Ok(file
        .file_name()
        .ok_or(IoError::new(ErrorKind::Other, "Filename no valid os string"))?
        .to_str()
        .ok_or(IoError::new(ErrorKind::Other, "Filename is no valid utf-8"))?
        .to_string())
}

fn read_directory(path: &Path) -> Result<HashMap<String, Vec<u8>>, IoError> {
    let mut result = HashMap::with_capacity(8);

    for entry in read_dir(path)? {
        let file = entry?.path();
        if file.is_file() {
            let file = read_file(&file)?;
            result.insert(format!("/{}", file.0), file.1);
        } else {
            let map = read_directory(&file)?;
            for f in map {
                result.insert(format!("/{}{}", file_or_dir_name(&file)?, f.0), f.1);
            }
        }
    }
    Ok(result)
}

pub fn load_resources() -> Result<HashMap<String, String>, IoError> {
    let current_dir = env::current_dir()?;
    let resources_dir = PathBuf::from(current_dir.into_os_string().into_string().unwrap() + "/src/threaded/resources");
    let mut result = HashMap::with_capacity(8);

    for entry in fs::read_dir(&resources_dir)? {
        let entry = entry?.path();
        if entry.is_file() {
            let file_name = format!("/{}", entry.strip_prefix(&resources_dir).unwrap().as_os_str().to_str().unwrap());
            let file = fs::read_to_string(&entry)?;
            result.insert(file_name, file);
        }
    }
    Ok(result)
}

// todo finish loading of directory in memory and returning it
pub fn load_directory(path: &Path) -> HashMap<String, Vec<u8>> {
    match get_all_files_in_dir(path) {
        Err(err) => {
            println!("Error getting files in dir: {:?}", err);
            panic!(); //todo
        }
        Ok(list) => {
            println!("Successfully read dir in memory: {:#?}", list.keys());
            return list;
        }
    }
}

