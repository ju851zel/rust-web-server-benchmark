use std::collections::HashMap;

//todo change below to better implementation
#[derive(Debug)]
pub struct Request {
    start_line: StartLine,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn get_first_line(request: &str) -> Result<StartLine, String> {
        let error = "Could not parse request. Wrong Format".to_string();

        let lines: Vec<&str> = request.split("\r\n").collect();
        let first_line = match lines.get(0) {
            Some(line) => line,
            None => return Err(error),
        };
        let first_line_content: Vec<&str> = first_line.split_whitespace().collect();
        let req_type = match first_line_content.get(0) {
            Some(&"GET") => RequestType::GET,
            Some(&"POST") => RequestType::POST,
            _ => return Err(error),
        };

        let req_path = match first_line_content.get(1) {
            Some(path) => path,
            _ => return Err(error),
        };

        let req_version = match first_line_content.get(2) {
            Some(version) => version,
            _ => return Err(error),
        };

        Ok(StartLine {
            method: req_type,
            path: req_path.to_string(),
            version: req_version.to_string(),
        })
    }
}

#[derive(Debug)]
pub enum RequestType {
    GET,
    POST,
    //todo add more cases
}

#[derive(Debug)]
pub struct StartLine {
    method: RequestType,
    path: String,
    version: String,
}




