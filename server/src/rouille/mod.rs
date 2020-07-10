use crate::Directory;
use std::thread;
use std::net::TcpListener;
use crate::response::{Response, send_response};
use std::io::{ Read};
use crate::request::parse_request;


pub fn start_server(ip: String, port: i32, dir: Directory) {
    let address = format!("{}:{}", ip, port);

    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(_) => {
            println!("Threaded: TCP bind error. Consider a restart of the programm");
            return;
        }
    };
    for stream in listener.incoming() {
        let mut connection = match stream {
            Ok(stream) => stream,
            Err(_) => {
                println!("Connection error. Ignoring request");
                continue;
            }
        };

        let dir = dir.clone();

        match thread::Builder::new().spawn(move || {
            let mut buffer = [0; 2048];
            match connection.read(&mut buffer) {
                Ok(request) => { request }
                Err(_) => {
                    println!("Connection error. Could not read from socket");
                    return;
                }
            };
            let request = match parse_request(buffer.to_vec()) {
                Ok(request) => request,
                Err(_) => {
                    let mut response = Response::default_bad_request();
                    send_response(connection, &mut response);
                    return;
                }
            };
            let mut response = match dir.get(&request.request_identifiers.path) {
                Some(resource) => {
                    let mut response = Response::default_ok();
                    &response.add_content_type(request.request_identifiers.path);
                    response.body = resource.clone();
                    response
                }
                None => {
                    let mut response = Response::default_not_found();
                    send_response(connection, &mut response);
                    return;
                }
            };
            println!("sending response");
            send_response(connection, &mut response);
        }) {
            Err(_) => {
                println!("No more resources for creating thread");
                return;
            }
            _ => {}
        }
    }
}