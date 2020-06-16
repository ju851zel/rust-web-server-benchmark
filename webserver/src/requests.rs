use std::collections::HashMap;

//todo change below to better implementation
#[derive(Debug)]
pub struct Request {
    pub start_line: StartLine,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn read_request(buffer: &str) -> Result<Request, String> {
        let lines: Vec<&str> = buffer.split("\r\n").collect();
        let start_line = Request::get_first_line(&lines)?;
        let headers = Request::get_headers(&lines.into_iter().skip(1).collect())?;
        let headers = Request::remove_empty(headers);

        Ok(Request {
            start_line,
            headers,
            body: "".to_string(), //todo change to real implementation
        })
    }
    fn get_first_line(lines: &Vec<&str>) -> Result<StartLine, String> {
        let error = "Could not parse request. Wrong Format".to_string();
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

    fn get_headers(lines: &Vec<&str>) -> Result<HashMap<String, String>, String> {
        let error = "Could not parse request. Wrong Format".to_string();
        let mut map = HashMap::new();
        for line in lines {
            let line_contents: Vec<&str> = line.split(": ").collect();
            let key = match line_contents.first() {
                Some(key) => key,
                None => return Err(format!("Could not parse request. The header value: ${}, does not conform to the http protocol", line)),
            };
            let value = line_contents.iter()
                .skip(1)
                .map(|s| *s)
                .collect::<Vec<&str>>().join("");
            map.insert(key.to_string(), value);
        };
        Ok(map)
    }

    fn remove_empty(headers: HashMap<String, String>) -> HashMap<String, String> {
        headers.into_iter()
            .filter(|line| !(*line).0.is_empty() && !(*line).1.is_empty())
            .collect()
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
    pub method: RequestType,
    pub path: String,
    pub version: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_empty_test() {
        let mut request: HashMap<String,String> = HashMap::new();
        request.insert("Host".to_string(), "localhost:8000".to_string());
        request.insert("".to_string(), "".to_string());
        request.insert("".to_string(), "Test".to_string());
        request.insert("Test".to_string(), "".to_string());
        let request = Request::remove_empty(request);
        assert_eq!(request.len(), 1)
    }
}