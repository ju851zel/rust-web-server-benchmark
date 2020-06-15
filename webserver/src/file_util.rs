use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::read_dir;
use std::io::{Error as IoError, ErrorKind, Error};
use std::path::Path;

/// Get all files in a directory.
///
/// Returns a map consisting of all the files in the root level of the provided dir
/// More specific return a hashmap containing the filename as key and the file content as utf-8 value
pub fn get_all_files_in_dir(path: &Path) -> Result<HashMap<String, String>, IoError> {
    let mut result = HashMap::with_capacity(8);

    if !path.is_dir() { return error("Provided path is no directory"); }

    for entry in read_dir(path)? {
        let file_path = entry?.path();
        if file_path.is_file() {
            let filename = file_path.to_str();
            if filename.is_none() {
                return error(format!("Filename of the file : {:?} no valid utf-8", file_path).as_str());
            };
            let filename = filename.unwrap();
            result.insert(filename.to_string(), read_to_string(file_path)?);
        } else {
            // todo add recursive visiting
        }
    }
    Ok(result)
}

/// Helper function to wrap a string as error, in order to use ? operator in other functions
fn error(message: &str) -> Result<HashMap<String, String>, Error> {
    return Result::Err(IoError::new(ErrorKind::Other, message));
}


// GET /hello HTTP/1.1\r\n
// Host: localhost:8080\r\n
// Connection: keep-alive\r\n
// Cache-Control: max-age=0\r\n
// DNT: 1\r\n
// Upgrade-Insecure-Requests: 1\r\n
// User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.97 Safari/537.36\r\n
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9\r\n
// Sec-Fetch-Site: none\r\n
// Sec-Fetch-Mode: navigate\r\n
// Sec-Fetch-User: ?1\r\n
// Sec-Fetch-Dest: document\r\n
// Accept-Encoding: gz
//
//
// GET / HTTP/1.1\r\n
// Host: localhost:8080\r\n
// Connection: keep-alive\r\n
// DNT: 1\r\n
// Upgrade-Insecure-Requests: 1\r\n
// User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.97 Safari/537.36\r\n
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9\r\n
// Sec-Fetch-Site: none\r\n
// Sec-Fetch-Mode: navigate\r\n
// Sec-Fetch-User: ?1\r\n
// Sec-Fetch-Dest: document\r\n
// Accept-Encoding: gzip, deflate, br\r\n
// Accept-Languag
