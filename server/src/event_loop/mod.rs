use std::sync::Arc;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::response::Response;
use crate::request::{Request, parse_request};
use futures::io::{ErrorKind};
use std::error::Error;

type Files = Arc<HashMap<String, Vec<u8>>>;
type Buffer = [u8; 2048];

mod ffi;
mod nbserver;
mod unsafe_c;


pub fn start_server(ip: String, port: i32, dir: Files) {
    nbserver::start_server(ip, port, dir)
}


fn create_response(buffer: Buffer, files: Files) -> Vec<u8> {
    let mut response = Response::default_ok();

    let request = parse_request(buffer);
    if request.is_err() {
        println!("{}",request.err().unwrap().description().to_string());
        return Response::default_bad_request().make_sendable();
    };

    let req_file = request.unwrap().request_identifiers.path;
    if !files.contains_key(&req_file) {
        println!("Requested source could not be found");
        return Response::default_not_found().make_sendable();
    }

    let req_file_content = files.get(&req_file).unwrap();
    response.add_content_type(req_file);
    response.body = req_file_content.clone();
    response.make_sendable()
}
