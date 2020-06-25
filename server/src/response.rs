use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;

#[derive(Debug)]
pub struct Response {
    pub response_identifiers: ResponseIdentifiers,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ResponseIdentifiers {
    pub method: ResponseType,
    pub version: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ResponseType {
    name: String,
    id: u32,
}

impl ResponseType {
    fn ok() -> Self { Self { name: "OK".to_string(), id: 200 } }
    fn not_found() -> Self { Self { name: "Not Found".to_string(), id: 404 } }
    fn bad_request() -> Self { Self { name: "Bad Request".to_string(), id: 400 } }
}


impl ResponseIdentifiers {
    fn make_sendable(&self) -> Vec<u8> {
        let ident = format!("HTTP/{} {} {}\r\n", self.version, self.method.id, self.method.name);
        ident.as_bytes().to_vec()
    }
}

impl Response {
    pub fn default_ok() -> Self {
        Self {
            response_identifiers: ResponseIdentifiers {
                method: ResponseType::ok(),
                version: "1.1".to_string(),
            },
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn default_not_found() -> Self {
        Self {
            response_identifiers: ResponseIdentifiers {
                method: ResponseType::not_found(),
                version: "1.1".to_string(),
            },
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn default_bad_request() -> Self {
        Self {
            response_identifiers: ResponseIdentifiers {
                method: ResponseType::bad_request(),
                version: "1.1".to_string(),
            },
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn make_sendable(&mut self) -> Vec<u8> {
        let mut ident = self.response_identifiers.make_sendable();
        let mut headers = self.make_headers_sendable();
        let mut vec = Vec::with_capacity(1);
        vec.append(&mut ident);
        vec.append(&mut headers);
        vec.append(&mut "\r\n".as_bytes().to_vec());
        vec.append(&mut self.body);
        vec
    }

    pub fn add_header(&mut self, header_key: &str, header_value: &str) {
        self.headers.insert(header_key.to_string(), header_value.to_string());
    }
    pub fn add_content_type(&mut self, file: String) {
        let file_ending = Path::new(&file).extension().and_then(OsStr::to_str).unwrap();
        let content_type = match file_ending {
            "json" => "text/json",
            "js" => "application/javascript",
            "png"=> "img/png",
            "jpeg"=> "img/jpeg",
            "html"=> "text/html",
            "txt" => "text/plain",
            "css" => "text/css",
            _ => "text/plain",
        };
        self.add_header("content-type",content_type);
    }

    pub fn make_headers_sendable(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.headers.len() * 4 * 40);
        for pair in &self.headers {
            vec.append(&mut pair.0.as_bytes().to_vec());
            vec.append(&mut ": ".as_bytes().to_vec());
            vec.append(&mut pair.1.as_bytes().to_vec());
            vec.append(&mut "\r\n".as_bytes().to_vec());
        }
        vec
    }
}


#[cfg(test)]
mod response_identifiers_test {
    use super::*;

    const CORRECT_OK: &str = "HTTP/1.1 200 OK\r\n";
    const CORRECT_NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n";
    const CORRECT_BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request\r\n";

    #[test]
    fn make_sendable_test() {
        let response_ident = ResponseIdentifiers { method: ResponseType::ok(), version: "1.1".to_string() };
        assert_eq!(CORRECT_OK,
                   String::from_utf8(response_ident.make_sendable()).unwrap());
        let response_ident = ResponseIdentifiers { method: ResponseType::not_found(), version: "1.1".to_string() };
        assert_eq!(CORRECT_NOT_FOUND,
                   String::from_utf8(response_ident.make_sendable()).unwrap());
        let response_ident = ResponseIdentifiers { method: ResponseType::bad_request(), version: "1.1".to_string() };
        assert_eq!(CORRECT_BAD_REQUEST,
                   String::from_utf8(response_ident.make_sendable()).unwrap());
    }
}


#[cfg(test)]
mod response_test {
    use super::*;

    const CORRECT_OK: &str = "HTTP/1.1 200 OK\r\n";
    const CORRECT_NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n";
    const CORRECT_BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request\r\n";

    #[test]
    fn make_sendable_simple_tests() {
        assert_eq!(format!("{}\r\n", CORRECT_OK),
                   String::from_utf8(Response::default_ok().make_sendable()).unwrap());
        assert_eq!(format!("{}\r\n", CORRECT_NOT_FOUND),
                   String::from_utf8(Response::default_not_found().make_sendable()).unwrap());
        assert_eq!(format!("{}\r\n", CORRECT_BAD_REQUEST),
                   String::from_utf8(Response::default_bad_request().make_sendable()).unwrap());
    }


    #[test]
    fn response_with_header_tests() {
        let mut response = Response::default_ok();
        response.add_header("content-type", "text/json");
        response.body = "Hello".to_string().into_bytes();
        assert_eq!(format!("{}content-type: text/json\r\n\r\nHello", CORRECT_OK),
                   String::from_utf8(response.make_sendable()).unwrap());
    }

    #[test]
    fn make_headers_sendable_test() {
        let mut response = Response::default_ok();
        response.add_header("content-type", "text/json");
        let result = "content-type: text/json\r\n";
        assert_eq!(result,
                   String::from_utf8(response.make_headers_sendable()).unwrap());
    }

    #[test]
    fn add_header_test() {
        let mut response = Response::default_ok();
        response.add_header("content-type", "text/json");
        assert_eq!(response.headers.contains_key("content-type"), true);
        assert_eq!(response.headers.get("content-type").unwrap(), "text/json");
    }
}


