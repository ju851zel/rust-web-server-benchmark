use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use server::ThreadPool;
use std::io::{Read, Write};
use crate::response::Response;
use crate::Directory;
use crate::threaded::server::{ServerStats, RequestResult};
use std::time::Instant;
use chrono::Utc;
use crate::request::parse_request;
use crate::threaded::request_handler::handle_request;
use crate::threaded::controller::error_controller::error_response_400;
use std::error::Error;

mod server;
mod request_handler;
mod controller;

/// Starts the server listening on the address,
/// with the amount of threads provided by thread_pool_size.
pub fn start_server(ip: String, port: i32, thread_pool_size: i32, dir: Arc<HashMap<String, Vec<u8>>>) {
    let pool = ThreadPool::new(thread_pool_size as usize);

    let address = format!("{}:{}", ip, port);
    let stats = Arc::new(ServerStats { request_results: Mutex::new(vec![]) });

    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(_) => {
            println!("Threaded: TCP bind error. Consider a restart of the programm");
            return;
        }
    };
    for stream in listener.incoming() {
        let connection = match stream {
            Ok(stream) => stream,
            Err(_) => {
                println!("Threaded: Connection error. Ignoring request");
                continue;
            }
        };

        let dir = dir.clone();
        let stats_change = stats.clone();
        let stats_view = stats.clone();
        pool.execute(move|| {
            let result = stat_wrapper(handle_connection, connection, dir, stats_view);
            let mut results = stats_change.request_results.lock().unwrap();
            match result {
                Some(res) => results.push(res),
                _ => ()
            }
        });
    }
}

fn stat_wrapper(f: fn(TcpStream, Directory, Arc<ServerStats>) -> Option<(u32, String)>, stream: TcpStream, dir: Directory, stats: Arc<ServerStats>) -> Option<RequestResult> {
    let date = Utc::now().naive_local();
    let start = Instant::now();
    let connection_result = f(stream, dir, stats);
    let duration = start.elapsed().as_millis();

    match connection_result {
        Some(res) => Some(RequestResult { response_code: res.0, response_time: duration, time: date, requested_resource: res.1}),
        None => None
    }
}

fn handle_connection(mut stream: TcpStream, dir: Directory, stats: Arc<ServerStats>) -> Option<(u32, String)> {
    let mut buffer = [0; 2048];

    if let Err(_) = stream.read(&mut buffer) {
        println!("Threaded: Error while processing request. Ignoring request");
        return None
    }

    let request = match parse_request(buffer) {
        Ok(req) => req,
        Err(e) => {
            send_response(stream, &mut error_response_400(e.description().to_string()));
            return None
        }
    };

    let mut response = handle_request(&request, dir, stats);

    send_response(stream, &mut response);
    Some((response.response_identifiers.method.id, request.request_identifiers.path))
}

fn send_response(mut stream: TcpStream, response: &mut Response) {
    let worked = stream.write(&response.make_sendable());
    if let Err(err) =  worked {
         println!("Error while sending response: {}",err)
    }
    stream.flush().unwrap();
}