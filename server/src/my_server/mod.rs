use std::collections::HashMap;
use std::sync::Arc;
use std::net::{TcpListener, TcpStream};
use server::ThreadPool;
use crate::my_server::request::{Request};
use std::io::{Read, Write};
use crate::my_server::response::Response;

mod server;
mod request;
mod response;

/// Starts the server listening on the address,
/// with the amount of threads provided by thread_pool_size.
pub fn start_server(ip: String, port: i32, thread_pool_size: i32, dir: Arc<HashMap<String, Vec<u8>>>) {
    let pool = match ThreadPool::new(thread_pool_size as usize) {
        Ok(pool) => pool,
        Err(err) => panic!(err)
    };

    let address = format!("{}:{}", ip, port);

    println!("My server listening for incoming requests on {}", address);

    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(err) => panic!(err)
    };
    for stream in listener.incoming() {
        println!("{:#?}", stream);
        let connection = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("{:#?}", e);
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
    stream.read(&mut buffer).unwrap();
    let buffer = String::from_utf8(buffer.to_vec()); //todo https

    if buffer.is_err() {
        println!("Response could not be interpreted as string");
        send_response(stream, Response::default_bad_request());
        return;
    };

    let request = Request::read_request(&buffer.unwrap());
    println!("Request came in, request: {:#?}", request);

    if request.is_err() {
        println!("Response could not be interpreted as string");
        send_response(stream, Response::default_bad_request());
        return;
    };

    let key = request.unwrap().request_identifiers.path;

    if !dir.contains_key(&key) {
        println!("Requested source could not be found");
        send_response(stream, Response::default_not_found());
        return;
    }

    let resource = dir.get(&key).unwrap();
    response.add_content_type(key);
    response.body = resource.clone();
    send_response(stream, response);
}

fn send_response(mut stream: TcpStream, mut response: Response) {
    stream.write(&response.make_sendable()).unwrap();
    stream.flush().unwrap();
}