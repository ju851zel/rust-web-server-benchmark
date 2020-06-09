use std::collections::HashMap;
//todo change below to better implementation
pub struct Request {
    start_line: StartLine,
    headers: HashMap<String, String>,
    body: String,
}

enum RequestType {
    GET,
    POST,
    //todo add more cases
}

struct StartLine {
    method: RequestType,
    path: String,
    version: String,
}


fn determine_request_type(request: &str) -> Result<StartLine, &'static str> {
    let error = "Could not parse request. Wrong Format";

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

    let req_version = match first_line_content.get(1) {
        Some(version) => version,
        _ => return Err(error),
    };

    Ok(StartLine {
        method: req_type,
        path: req_path.to_string(),
        version: req_version.to_string(),
    })
}



