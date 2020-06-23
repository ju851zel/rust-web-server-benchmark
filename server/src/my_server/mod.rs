use std::collections::HashMap;
use std::sync::Arc;
use std::net::{TcpListener, TcpStream};
use server::ThreadPool;
use crate::my_server::requests::Request;
use std::io::{Read, Write};
use std::borrow::{Borrow, BorrowMut};

mod server;
mod requests;

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
    let mut response200 = "HTTP/1.1 200 OK\r\n\r\n".as_bytes().to_vec();
    let response404 = "HTTP/1.1 404 Not found\r\n\r\n".as_bytes().to_vec();

    stream.read(&mut buffer).unwrap();
    let buffer = String::from_utf8(buffer.to_vec());
    let buffer = match buffer {
        Ok(value) => value,
        Err(err) => {
            println!("Not valid request: {}", err);
            return;
        }
    };

    let request = Request::read_request(&buffer);

    println!("Request came in, request: {:#?}", request);
    let path = match request {
        Ok(request) => request.request_identifiers.path,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let response = match dir.get(&path) {
        Some(value) => {
            response200.append(&mut value.clone());
            response200
        }
        None => response404
    };

    stream.write(&response).unwrap();
    stream.flush().unwrap();
}