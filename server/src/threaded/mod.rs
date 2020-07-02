use std::collections::HashMap;
use std::sync::Arc;
use std::net::{TcpListener, TcpStream};
use server::ThreadPool;
use std::io::{Read, Write};
use crate::response::Response;
use crate::request::Request;
use crate::Directory;
use crate::threaded::server::{ServerStats, RequestResult};
use std::time::{Instant, SystemTime};
use chrono::Utc;
use std::path::Path;
use std::{fs, env};

mod server;

/// Starts the server listening on the address,
/// with the amount of threads provided by thread_pool_size.
pub fn start_server(ip: String, port: i32, thread_pool_size: i32, dir: Arc<HashMap<String, Vec<u8>>>) {
    let pool = ThreadPool::new(thread_pool_size as usize);

    let address = format!("{}:{}", ip, port);
    let stats = Arc::new(ServerStats { request_results: vec![] });

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
        let stats = stats.clone();
        pool.execute(|| {
            handle_connection(connection, dir, stats);
        });
    }
}

fn handle_connection(mut stream: TcpStream, dir: Directory, stats: Arc<ServerStats>) {
    let mut buffer = [0; 2048];

    if let Err(err) = stream.read(&mut buffer) {
        println!("Threaded: Error while processing request. Ignoring request");
        return;
    }

    let start = Instant::now();
    let mut response = Request::create_response(buffer, dir);

    send_response(stream, &mut response);
    let duration = start.elapsed().as_millis();
    let b = Utc::now().date();
    let a = RequestResult { response_code: response.response_identifiers.method.id, response_time: duration, time: b, requested_resource: "a".to_string()};
}

fn send_response(mut stream: TcpStream, response: &mut Response) {
    let worked = stream.write(&response.make_sendable());
    if let Err(err) =  worked {
         println!("Error while sending response: {}",err)
    }
    stream.flush().unwrap();
}