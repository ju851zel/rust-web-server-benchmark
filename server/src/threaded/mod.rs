use std::collections::HashMap;
use std::sync::Arc;
use std::net::{TcpListener, TcpStream};
use server::ThreadPool;
use std::io::{Read, Write};
use crate::response::Response;
use crate::request::Request;

mod server;

/// Starts the server listening on the address,
/// with the amount of threads provided by thread_pool_size.
pub fn start_server(ip: String, port: i32, thread_pool_size: i32, dir: Arc<HashMap<String, Vec<u8>>>) {
    let pool = ThreadPool::new(thread_pool_size as usize);

    let address = format!("{}:{}", ip, port);

    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(err) => {
            println!("Threaded: TCP bind error. Consider a restart of the programm");
            return;
        }
    };
    for stream in listener.incoming() {
        let connection = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("Threaded: Connection error. Ignoring request");
                continue;
            }
        };

        let dir = dir.clone();
        pool.execute(|| {
            handle_connection(connection, dir);
        });
    }
}

fn handle_connection(mut stream: TcpStream, dir: Arc<HashMap<String, Vec<u8>>>) {
    let mut buffer = [0; 2048];
    let mut response = Response::default_ok();

    if let Err(err) = stream.read(&mut buffer) {
        println!("Threaded: Error while processing request. Ignoring request");
        return;
    }

    let request = match String::from_utf8(buffer.to_vec()) {
        Ok(string) => string,
        Err(_) => {
            println!("Threaded: Request could not be interpreted as string.");
            send_response(stream, Response::default_bad_request());
            return;
        }
    };

    let key = match Request::read_request(&request) {
        Ok(request) => request.request_identifiers.path,
        Err(err) => {
            println!("Threaded: Request could not be interpreted as string.");
            send_response(stream, Response::default_bad_request());
            return;
        }
    };

    match dir.get(&key) {
        Some(resource) => {
            response.add_content_type(key);
            response.body = resource.clone();
            send_response(stream, response);
        }
        None => {
            println!("Threaded: Requested resource {} could not be found.", key);
            send_response(stream, Response::default_not_found());
            return;
        }
    }
}

fn send_response(mut stream: TcpStream, mut response: Response) {
    stream.write(&response.make_sendable()).unwrap();
    stream.flush().unwrap();
}