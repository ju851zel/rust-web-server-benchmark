use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;
use std::fs;

/// The object used in all the servers, to represent the http response.
#[derive(Debug)]
pub struct Response {
    pub response_identifiers: ResponseIdentifiers,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

/// The identifier for the response, containing http method and version
///
/// E.g. Not_Found - 404 Http1.1
#[derive(Debug, Eq, PartialEq)]
pub struct ResponseIdentifiers {
    pub method: ResponseType,
    pub version: String,
}

/// The http response type
///
///E.g. Not_Found - 404
#[derive(Debug, Eq, PartialEq)]
pub struct ResponseType {
    name: String,
    pub id: u32,
}

impl ResponseType {
    /// The default 200 - OK response
    fn ok() -> Self { Self { name: "OK".to_string(), id: 200 } }
    /// The default 404 - Not Found response
    fn not_found() -> Self { Self { name: "Not Found".to_string(), id: 404 } }
    /// The default 400 - Bad Request response
    fn bad_request() -> Self { Self { name: "Bad Request".to_string(), id: 400 } }
}


impl ResponseIdentifiers {
    /// Makes the response identifyer into a sendable byte vector
    fn make_sendable(&self) -> Vec<u8> {
        let ident = format!("HTTP/{} {} {}\r\n", self.version, self.method.id, self.method.name);
        ident.as_bytes().to_vec()
    }
}

impl Response {
    /// Creates the default OK 200 response
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

    /// Creates the default Not Found 404 response
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

    /// Creates the default Bad Request 400 response
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

    /// Creates a default error page response
    pub fn dynamic_error_response(&mut self, error_message: String) {
        let mut map = HashMap::new();
        let error_code: &str = &self.response_identifiers.method.id.to_string();
        let error_code_full: &str = &format!("{} {}", error_code, self.response_identifiers.method.name);

        map.insert("{{ErrorCode}}", error_code_full);
        map.insert("{{ErrorMessage}}", &error_message);

        insert_dynamic_html(self, "_dist/error_page.html", map);
    }

    /// Makes the response into a sendable byte vector
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

    /// Adds a specific header into the response
    pub fn add_header(&mut self, header_key: &str, header_value: &str) {
        self.headers.insert(header_key.to_string(), header_value.to_string());
    }

    /// Adds the content type into the response
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

    /// Makes the response header into a sendable byte vector
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

// todo comment what does it do after reimplementing the fs::read_string into correct
pub fn insert_dynamic_html(mut response: &mut Response, path: &str, templating_replacements: HashMap<&str, &str>) {
    response.add_content_type("_.html".to_string());
    let mut resource = fs::read_to_string(path).unwrap();

    for replacements in templating_replacements {
        resource = resource.replace(replacements.0, &replacements.1);
    }

    response.body = resource.as_bytes().to_vec();
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


