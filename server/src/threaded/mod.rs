use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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
    let stats = Arc::new(ServerStats { request_results: Mutex::new(vec![]) });

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
        let stats2 = stats.clone();
        pool.execute(move|| {
            let mut results = stats2.request_results.lock().unwrap();
            let result = stat_wrapper(handle_connection,connection, dir);
            results.push(result.unwrap());
        });

        println!("{:?}", stats);
    }
}

fn stat_wrapper(f: fn(TcpStream, Directory) -> Option<(u32, String)>, mut stream: TcpStream, dir: Directory) -> Option<RequestResult> {
    let date = Utc::now().date();
    let start = Instant::now();
    let connection_result = f(stream, dir);
    let duration = start.elapsed().as_millis();
    Some(RequestResult { response_code: connection_result.unwrap().0, response_time: duration, time: date, requested_resource: connection_result.unwrap().1})
}

fn handle_connection(mut stream: TcpStream, dir: Directory) -> Option<(u32, String)> {
    let mut buffer = [0; 2048];

    if let Err(err) = stream.read(&mut buffer) {
        println!("Threaded: Error while processing request. Ignoring request");
        return None
    }

    let mut response = Request::create_response(buffer, dir);
    send_response(stream, &mut response.0);
    Some((response.0.response_identifiers.method.id, response.1))
}

fn send_response(mut stream: TcpStream, response: &mut Response) {
    let worked = stream.write(&response.make_sendable());
    if let Err(err) =  worked {
         println!("Error while sending response: {}",err)
    }
    stream.flush().unwrap();
}