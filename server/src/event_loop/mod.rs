use std::sync::Arc;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::response::Response;
use crate::request::Request;
use futures::io::{ErrorKind};

type Files = Arc<HashMap<String, Vec<u8>>>;
type Buffer = [u8; 2048];

mod ffi;
mod nbserver;
mod unsafe_c;


pub fn start_server(ip: String, port: i32, dir: Files) {
    nbserver::start_server(ip, port, dir)
}


pub fn start_old_server(ip: String, port: i32, dir: Files) {
    let address = format!("{}:{}", ip, port);

    println!("Nonblocking server listening for incoming requests on {}", address);

    let mut reading_conns: Vec<(TcpStream, Buffer)> = vec![];
    let mut writing_conns: Vec<(TcpStream, Vec<u8>)> = vec![];

    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(err) => panic!(err)
    };

    match listener.set_nonblocking(true) {
        Err(err) => panic!(err),
        _ => {}
    };

    loop {
        let conn = handle_reading_conns(&mut reading_conns);
        if let Some((stream, buffer, finished_read)) = conn {
            if finished_read {
                writing_conns.push((stream, create_response(buffer, dir.clone())));
            } else {
                reading_conns.push((stream, buffer));
            }
        }

        let conn = handle_writing_conns(&mut writing_conns);
        if let Some((stream, buffer, finished_write)) = conn {
            if !finished_write {
                writing_conns.push((stream, buffer));
            }
        }

        match get_connection(&listener) {
            Some(stream) => reading_conns.push((stream, [0; 2048])),
            _ => {}
        }
    }
}

fn handle_reading_conns(conns: &mut Vec<(TcpStream, Buffer)>) -> Option<(TcpStream, Buffer, bool)> {
    if conns.is_empty() { return None; };

    println!("Working on incoming connection");

    let (mut stream, mut buffer) = conns.remove(0);
    let bytes_read = stream.read(&mut buffer);

    let bytes_read = match bytes_read {
        Ok(val) => val,
        Err(err) if err.kind() == ErrorKind::WouldBlock => { 0 }
        Err(err) => {
            println!("{}", err);
            0
        }
    };

    Some(if bytes_read > 0 {
        println!("Read {} bytes, but not all available", bytes_read);
        (stream, buffer, false)
    } else {
        println!("Finished read{}", bytes_read);
        (stream, buffer, true)
    })
}

fn handle_writing_conns(conns: &mut Vec<(TcpStream, Vec<u8>)>) -> Option<(TcpStream, Vec<u8>, bool)> {
    if conns.is_empty() { return None; }

    println!("Working on writing connection");

    let (mut stream, mut buffer) = conns.remove(0);
    let bytes_wrote = stream.write(&mut buffer);

    let bytes_wrote = match bytes_wrote {
        Ok(val) => { val }
        Err(err) if err.kind() == ErrorKind::WouldBlock => { 0 }
        Err(err) => {
            println!("{}", err);
            0
        }
    };

    Some(if bytes_wrote == buffer.len() {
        buffer = Vec::new();
        println!("Did not write all {} bytes", bytes_wrote);
        (stream, buffer, true)
    } else if bytes_wrote > 0 {
        buffer = buffer.into_iter().skip(bytes_wrote).collect();
        println!("Wrote {} bytes", bytes_wrote);
        (stream, buffer, false)
    } else {
        println!("Did not write all {} bytes", bytes_wrote);
        (stream, buffer, true)
    })
}

fn get_connection(listener: &TcpListener) -> Option<TcpStream> {
    match listener.accept() {
        Ok(con) => return Some(con.0),
        Err(err) if err.kind() != ErrorKind::WouldBlock =>
            { println!("Error in accept: {:#?}", err.kind()); }
        _ => {}
    }
    None
}


fn create_response(buffer: Buffer, files: Files) -> Vec<u8> {
    let mut response = Response::default_ok();

    let utf8_buffer = String::from_utf8(buffer.to_vec()); //todo https
    if utf8_buffer.is_err() {
        println!("Request could not be interpreted as utf-8");
        return Response::default_bad_request().make_sendable();
    };

    let request = Request::read_request(&utf8_buffer.unwrap());
    if request.is_err() {
        println!("Request is invalid");
        return Response::default_bad_request().make_sendable();
    };

    let req_file = request.unwrap().request_identifiers.path;
    if !files.contains_key(&req_file) {
        println!("Requested source could not be found");
        return Response::default_not_found().make_sendable();
    }

    let req_file_content = files.get(&req_file).unwrap();
    response.add_content_type(req_file);
    response.body = req_file_content.clone();
    response.make_sendable()
}
