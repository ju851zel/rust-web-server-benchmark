use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::read_dir;
use std::io::{Error as IoError, ErrorKind, Error};
use std::path::Path;

/// Get all files in a directory.
///
/// Returns a map consisting of all the files in the root level of the provided dir
/// More specific return a hashmap containing the filename as key and the file content as utf-8 value
pub fn get_all_files_in_dir(path: &Path) -> Result<Box<HashMap<String, String>>, IoError> {
    let mut result = Box::new(HashMap::with_capacity(8));

    if !path.is_dir() { return error("Provided path is no directory"); }

    for entry in read_dir(path)? {
        let file_path = entry?.path();

        if file_path.is_file() {
            let filename = file_path.to_str();
            if filename.is_none() {
                return error("Provided path is no directory");
            };
            let filename = filename.unwrap();
            result.insert(filename.to_string(), read_to_string(path)?);
        } else {
            // todo add recursive visiting
        }
    }
    Ok(result)
}

/// Helper function to wrap a string as error, in order to use ? operator in other functions
fn error(message: &str) -> Result<Box<HashMap<String, String>>, Error> {
    return Result::Err(IoError::new(ErrorKind::Other, message));
}

