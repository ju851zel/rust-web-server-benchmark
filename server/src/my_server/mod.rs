use std::collections::HashMap;
use std::sync::Arc;
use std::net::{TcpListener, TcpStream};
use server::ThreadPool;
use crate::my_server::requests::Request;
use std::io::{Read, Write};
use std::str::FromStr;
use std::any::Any;
use std::convert::TryInto;
use std::panic::resume_unwind;

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

//    let response = match dir.get(&path) {
//        Some(value) => {
//            response200.append(&mut value.clone());
//            response200.clone()
//        }
//        None => response404.clone()
//    };

    let stats = String::from("/stats");

    let mut response = match path {
        stats => {
            println!("gregregregr");
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n".as_bytes().to_vec()
        }
        _ => response404
    };

    let timber_resources: HashMap<&str, Vec<u8>> = [("Norway", "{\"name\": \"a\"}".as_bytes().to_vec())].iter().cloned().collect();
    let timber_resources2: HashMap<&str, &str> = [("Norway", "gre")].iter().cloned().collect();



    let a = [(1, 2)];
    // <&str, &mut vec<u8>
    let stats:  ServerStats = ServerStats { responses: [(200, 2), (404, 1)].iter().cloned().collect() };
    // parse_to_json(stats);

    response.append(&mut parse_to_json(stats).as_bytes().to_vec());

    // println!("{}", parse_to_json(stats));

    stream.write(&response).unwrap();
    stream.flush().unwrap();
}

struct ServerStats {
    responses: HashMap<i32, i32>
}

fn parse_to_json(stats: ServerStats) -> String {
    let mut result_json = "{".to_string();

    stats.responses.iter()
        .for_each(|x| {
            result_json = format!("{}\"{}\": \"{}\",", result_json, x.0, x.1);
        });

    if result_json.ends_with(",") {
        result_json = format!("{}", &result_json[0..result_json.len()-1]);
    }

    let mut result_json = format!("{}{}", result_json, "}");

    result_json
}

