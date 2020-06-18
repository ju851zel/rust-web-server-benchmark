use std::collections::HashMap;
use std::collections::hash_map::Entry;

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

        Ok(Request {
            start_line,
            headers,
            body: "".to_string(), //todo change to real implementation
        })
    }
    fn get_first_line(lines: &Vec<&str>) -> Result<StartLine, String> {
        //todo Jörg do you know a better solution instead of the many matches?
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
        let map: HashMap<String, String> = lines.into_iter()
            .map(|line| -> Vec<&str> { line.split(": ").collect() })
            // todo| Jörg replace the unwrap (a line below) with a map or sth like that
            // todo| and return the error from the function when error occurs
            .map(|vec| (vec.first().unwrap().to_string(),
                        vec.iter()
                            .skip(1)
                            .map(|s| *s)
                            .collect::<Vec<&str>>().join("")))
            .filter(|line| !(*line).0.is_empty() && !(*line).1.is_empty())
            .collect();
        Ok(map)
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
    fn get_headers_test() {
        let request = vec![
            "GET /hello HTTP/1.1\r\n",
            "Host: localhost:8080\r\n",
            "Connection: keep-alive\r\n",
            "Cache-Control: max-age=0\r\n",
            "DNT: 1\r\n",
            "Upgrade-Insecure-Requests: 1\r\n",
            "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.97 Safari/537.36\r\n",
            "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9\r\n",
            "Sec-Fetch-Site: none\r\n",
            "Sec-Fetch-Mode: navigate\r\n",
            "Sec-Fetch-User: ?1\r\n",
            "Sec-Fetch-Dest: document\r\n",
        ];

        let mut result= HashMap::new();
        result.insert("Host", "localhost:8080\r\n");
        result.insert("Connection", "keep-alive\r\n");
        result.insert("Cache-Control", "max-age=0\r\n");
        result.insert("DNT", "1\r\n");
        result.insert("Upgrade-Insecure-Requests", "1\r\n");
        result.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.97 Safari/537.36\r\n");
        result.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9\r\n");
        result.insert("Sec-Fetch-Site", "none\r\n");
        result.insert("Sec-Fetch-Mode", "navigate\r\n");
        result.insert("Sec-Fetch-User", "?1\r\n");
        result.insert("Sec-Fetch-Dest", "document\r\n");

        let result: HashMap<String, String> =
            result.iter()
                .map(|str| (str.0.to_string(), str.1.to_string()))
                .collect();
        assert_eq!(Request::get_headers(&request).unwrap(), result)
    }
}