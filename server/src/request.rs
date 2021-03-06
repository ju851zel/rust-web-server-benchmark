use std::collections::HashMap;
use std::error;
use core::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// The object used in all the servers, to represent the http request.
#[derive(Debug)]
pub struct Request {
    pub request_identifiers: RequestIdentifiers,
    pub headers: HashMap<String, String>,
    pub body: String,
}

/// The identifier for the request, containing http method, version and path
///
/// E.g. GET, Http1.1, /example.html
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub struct RequestIdentifiers {
    pub method: RequestType,
    pub path: String,
    pub version: String,
}

/// The http request type
///
///E.g. GET
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum RequestType {
    Get,
    Post,
}

#[derive(Debug)]
struct InvalidRequest {
    message: String
}

impl fmt::Display for InvalidRequest {
    /// formatting the Invalid request.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse request.{}", self.message)
    }
}

impl error::Error for InvalidRequest {
    /// setting the error message.
    fn description(&self) -> &str {
        &self.message
    }
}

/// Parses the byte Vector stream into Request object.
pub fn parse_request(buffer: Vec<u8>) -> Result<Request> {
    let raw_request = match String::from_utf8(buffer) {
        Ok(string) => string,
        Err(_) => return Err(Box::new(Box::new(InvalidRequest{message: "Request could not be interpreted as string.".to_string()})))
    };

    read_request(&raw_request)
}

/// Reads the contents from the byte Vector into the Request object.
fn read_request(buffer: &str) -> Result<Request> {
    let lines: Vec<&str> = buffer.split("\r\n").collect();
    let request_identifiers = get_request_identifiers(&lines)?;
    let headers = get_headers(&lines.into_iter().skip(1).collect())?;

    Ok(Request {
        request_identifiers,
        headers,
        body: "".to_string(), //todo change to real implementation
    })
}

/// Reads the request identifier from the http request
fn get_request_identifiers(lines: &Vec<&str>) -> Result<RequestIdentifiers> {
    let first_line_content: Vec<&str> = lines
        .get(0).ok_or(InvalidRequest{message: "First line does not confirm HTTP protocol.".to_string()})?
        .split_whitespace().collect();

    let req_type = match first_line_content
        .get(0)
        .ok_or(InvalidRequest{message: "HTTP Type not specified".to_string()})? {
        &"GET" => RequestType::Get,
        &"POST" => RequestType::Post,
        _ => RequestType::Get
    };

    let req_path = first_line_content.get(1)
        .ok_or(InvalidRequest{message: "path not provided".to_string()})?;
    let req_path = req_path.split("?").collect::<Vec<&str>>();
    let req_path = req_path.get(0)
        .ok_or(InvalidRequest{message: "path not provided".to_string()})?;
    let req_version = first_line_content.get(2)
        .ok_or(InvalidRequest{message: "http version not specified.".to_string()})?;

    Ok(RequestIdentifiers {
        method: req_type,
        path: req_path.to_string(),
        version: req_version.to_string(),
    })
}

/// Reads the header from the http request
fn get_headers(lines: &Vec<&str>) -> Result<HashMap<String, String>> {
    Ok(lines.into_iter()
        .skip(1)
        .map(|line| -> Vec<&str> { line.splitn(2, ": ").collect() })
        .filter(|header_pair| header_pair.len() == 2)
        .map(|header_pair|
            ((header_pair.get(0).unwrap().to_string()), header_pair.get(1).unwrap().to_string())
        )
        .collect()
    )
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

        let mut result = HashMap::new();
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
        assert_eq!(get_headers(&request).unwrap(), result)
    }

    #[test]
    fn get_headers_test_no_colon() {
        let request = vec![
            "GET /hello HTTP/1.1\r\n",
            "Host: localhost:8080\r\n",
            "Connection keep-alive\r\n",
        ];

        let mut result = HashMap::new();
        result.insert("Host", "localhost:8080\r\n");

        let result: HashMap<String, String> =
            result.iter()
                .map(|str| (str.0.to_string(), str.1.to_string()))
                .collect();
        assert_eq!(get_headers(&request).unwrap(), result)
    }

    #[test]
    fn get_headers_test_colon_in_value() {
        let request = vec![
            "GET /hello HTTP/1.1\r\n",
            "Host: localhost:8080\r\n",
            "UselessHeader: Colon:Test\r\n"
        ];

        let mut result = HashMap::new();
        result.insert("Host", "localhost:8080\r\n");
        result.insert("UselessHeader", "Colon:Test\r\n");

        let result: HashMap<String, String> =
            result.iter()
                .map(|str| (str.0.to_string(), str.1.to_string()))
                .collect();
        assert_eq!(get_headers(&request).unwrap(), result)
    }

    #[test]
    fn get_headers_test_empty() {
        let request = vec![
            "GET /hello HTTP/1.1\r\n"
        ];

        let mut result = HashMap::new();

        assert_eq!(get_headers(&request).unwrap(), result)
    }

    #[test]
    fn get_request_identifiers_test() {
        let request = vec![
            "GET /hello HTTP/1.1\r\n"
        ];

        let mut result = RequestIdentifiers {
            method: RequestType::Get,
            path: "/hello".to_string(),
            version: "HTTP/1.1".to_string(),
        };

        assert_eq!(get_request_identifiers(&request).unwrap(), result)
    }

    #[test]
    fn get_request_identifiers_test_missing_path() {
        let request = vec![
            "GET HTTP/1.1\r\n"
        ];

        assert_eq!(get_request_identifiers(&request).err().unwrap().to_string(), "Could not parse request.http version not specified.")
    }

    #[test]
    fn get_request_identifiers_test_empty() {
        let request = vec![];

        assert_eq!(get_request_identifiers(&request).err().unwrap().to_string(), "Could not parse request.First line does not confirm HTTP protocol.")
    }
}