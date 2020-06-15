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
}

struct StartLine {
    method: RequestType,
    path: String,
    version: String,
}


