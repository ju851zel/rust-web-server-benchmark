use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use server::ThreadPool;
use std::io::Read;
use crate::response::send_response;
use crate::{StaticFiles, DynamicFiles};
use crate::threaded::server::{ServerStats, RequestResult, ServerFiles};
use std::time::Instant;
use chrono::Utc;
use crate::request::parse_request;
use crate::threaded::request_handler::handle_request;
use crate::threaded::controller::error_controller::error_response_400;

pub mod server;
mod request_handler;
mod controller;

/// Starts the threaded server listening on the address,
/// with the amount of threads provided by thread_pool_size.
pub fn start_server(ip: String, port: i32, thread_pool_size: i32, static_files: StaticFiles, dynamic_files: DynamicFiles) {
    let pool = ThreadPool::new(thread_pool_size as usize);

    let address = format!("{}:{}", ip, port);
    let stats = Arc::new(ServerStats { request_results: Mutex::new(vec![]) });

    let server_files = ServerFiles{static_files, dynamic_files};

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

        let server_files = server_files.clone();
        let stats_change = stats.clone();
        let stats_view = stats.clone();
        pool.execute(move|| {
            let result = stat_wrapper(handle_connection, connection, server_files, stats_view);
            let mut results = stats_change.request_results.lock().unwrap();
            match result {
                Some(res) => results.push(res),
                _ => ()
            }
        });
    }
}

/// Wraps the functionality to handle the request to retrieve its stats
fn stat_wrapper(f: fn(TcpStream, ServerFiles, Arc<ServerStats>) -> Option<(u32, String)>, stream: TcpStream, server_files: ServerFiles, stats: Arc<ServerStats>) -> Option<RequestResult> {
    let date = Utc::now().naive_local();
    let start = Instant::now();
    let connection_result = f(stream, server_files, stats);
    let duration = start.elapsed().as_millis();

    match connection_result {
        Some(res) => Some(RequestResult { response_code: res.0, duration: duration, time: date, requested_resource: res.1}),
        None => None
    }
}

/// Handles a single connection.
/// Checking the request of correctness and returning the requested file
fn handle_connection(mut stream: TcpStream, server_files: ServerFiles, stats: Arc<ServerStats>) -> Option<(u32, String)> {
    let mut buffer = [0; 2048];

    if let Err(_) = stream.read(&mut buffer) {
        println!("Threaded: Error while processing request. Ignoring request");
        return None
    }

    let request = match parse_request(buffer.to_vec()) {
        Ok(req) => req,
        Err(e) => {
            send_response(stream, &mut error_response_400(format!("{}", e), server_files.dynamic_files));
            return None
        }
    };

    let mut response = handle_request(&request, server_files, stats);

    send_response(stream, &mut response);
    Some((response.response_identifiers.method.id, request.request_identifiers.path))
}
