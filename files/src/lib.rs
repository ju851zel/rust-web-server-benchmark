use std::collections::HashMap;
use std::fs::read_dir;
use std::io::{Error as IoError, ErrorKind, Error};
use std::path::Path;

/// Get all files in a directory.
///
/// Returns a map consisting of all the files in the root level of the provided dir
/// More specific return a hashmap containing the filename as key and the file content as utf-8 value
pub fn get_all_files_in_dir(path: &Path) -> Result<HashMap<String, Vec<u8>>, IoError> {
    let mut result = HashMap::with_capacity(8);

    if !path.is_dir() { return error("Provided path is no directory"); }

    //todo make something like that, but not working yet
    // let res :Result<Vec<(String,String)>, IoError> = read_dir(path)?
    //     .map(|entry| entry?.path())
    //     .filter(|path| path.is_file())
    //     .map(|path | {
    //         (path.file_name()?.into_result()?.to_str().into_result()?,
    //         read_to_string(path)?)
    //     }).collect();

    for entry in read_dir(path)? {
        let file_path = entry?.path();
        if file_path.is_file() {
            let filename = file_path
                .file_name()
                .ok_or("Filename no valid os string".to_string())
                .map_err(|_error| std::io::Error::new(ErrorKind::Other, ""))?
                .to_str()
                .ok_or("Filename is no valid utf-8")
                .map_err(|_error| std::io::Error::new(ErrorKind::Other, ""))?
                .to_string();


            println!("filename::: {:#?}", filename);
            let file_content = std::fs::read(file_path)?;

            result.insert("/".to_string() + filename.as_str(), file_content);
        } else {
            // todo add recursive visiting
        }
    }
    Ok(result)
}


// todo finish loading of directory in memory and returning it
pub fn load_directory(path: &str) -> HashMap<String, Vec<u8>> {
    match get_all_files_in_dir(Path::new(path)) {
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

/// Helper function to wrap a string as error, in order to use ? operator in other functions
fn error(message: &str) -> Result<HashMap<String, Vec<u8>>, Error> {
    return Result::Err(IoError::new(ErrorKind::Other, message));
}

