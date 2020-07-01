use std::collections::HashMap;
use std::sync::Arc;
use std::net::{TcpListener, TcpStream};
use server::ThreadPool;
use std::io::{Read, Write};
use crate::response::Response;
use crate::request::Request;
use crate::Directory;

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

fn handle_connection(mut stream: TcpStream, dir: Directory) {
    let mut buffer = [0; 2048];

    if let Err(err) = stream.read(&mut buffer) {
        println!("Threaded: Error while processing request. Ignoring request");
        return;
    }
    let response = Request::create_response(buffer, dir);
    send_response(stream, response);
}

fn send_response(mut stream: TcpStream, mut response: Response) {
    let worked = stream.write(&response.make_sendable());
    if let Err(err) =  worked {
         println!("Error while sending response: {}",err)
    }
    stream.flush().unwrap();
}