use std::collections::HashMap;
use std::sync::Arc;
use std::net::{TcpListener, TcpStream};
use server::ThreadPool;
use crate::my_server::requests::Request;
use std::io::{Read, Write};

mod server;
mod requests;

/// Starts the server listening on the address,
/// with the amount of threads provided by thread_pool_size.
pub fn start_server(ip: String, port: i32, thread_pool_size: i32, dir: Arc<HashMap<String, String>>) {
    let pool = match ThreadPool::new(thread_pool_size as usize) {
        Ok(pool) => pool,
        Err(err) => panic!(err)
    };

    let address = format!("{}:{}", ip,port);

    println!("My server listening for incoming requests on {}", address);

    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(err) => panic!(err)
    };
    for stream in listener.incoming() {
        let connection = stream.unwrap();
        let dir = dir.clone();
        pool.execute(|| {
            handle_connection(connection, dir);
        });
    }
}

fn handle_connection(mut stream: TcpStream, dir: Arc<HashMap<String, String>>) {
    let mut buffer = [0; 2048];
    let response200 = "HTTP/1.1 200 OK\r\n\r\n";
    let response404 = "HTTP/1.1 404 Not found\r\n\r\n";

    stream.read(&mut buffer).unwrap();
    let buffer = String::from_utf8(buffer.to_vec());
    let request = Request::read_request(&buffer.unwrap());
    println!("Request came in, request: {:#?}", request);
    let path = match request {
        Ok(request) => request.start_line.path,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let response = if dir.contains_key(&path) {
        println!("return content to path: {}", path);
        format!("{}{}", response200, dir.get(&path).unwrap())
    } else {
        println!("Could not find path {} in {:#?}", path, dir);
        response404.to_string()
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}