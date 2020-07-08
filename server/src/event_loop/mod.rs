use crate::response::Response;
use crate::request::{Request, parse_request};
use crate::event_loop::ffi::{Queue, Event, ListenerEvent};
use std::sync::Arc;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::error::Error;
use core::ptr;
use std::io::{Read, ErrorKind, Write};

type Files = Arc<HashMap<String, Vec<u8>>>;
type Buffer = [u8; 2048];

mod ffi;
mod unsafe_c;

/// Starts the event loop server
///
/// The server is listening for incoming connections.
/// When a stream is available it reads its content performs the mapping to the dir
/// and returns the response.
/// Waiting for the stream to be ready, reading data from it and then writing it back into the response is done nonblocking.
/// This does not mean that the whole server works nonblocking.
/// It is always actively looking for work, but when work arrives, but would block,
/// it continues on other work.
/// In a future update it will even wait passively, if no work is available.
pub fn start_server(ip: String, port: i32, dir: Files) {
    let address = format!("{}:{}", ip, port);

    let (mut incoming_q, mut reading_q, mut writing_q) = match create_qs(dir) {
        Ok(qs) => qs,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };


    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(err) => panic!(err)
    };

    match listener.set_nonblocking(true) {
        Err(err) => panic!(err),
        _ => {}
    };

    let listener_event = ListenerEvent::new(listener, [0; 2048]);
    let worked = incoming_q.add(listener_event);
    if let Err(err) = worked {
        println!("Could not accept connection.")
    }

    loop {
        if reading_q.events.len() > 0 {
            handle_reading(&mut reading_q, &mut writing_q)
        }
        if writing_q.events.len() > 0 {
            handle_writing(&mut writing_q)
        }
        if incoming_q.events.len() > 0 {
            handle_incoming(&mut incoming_q, &mut reading_q)
        }
    }
}

/// Creating the queues to handle the requests:
///
/// Creates the incoming, writing and reading queues.
/// When an error occurs, the server shuts down.
fn create_qs(dir: Files) -> Result<(Queue<ListenerEvent>, Queue<Event>, Queue<Event>), String> {
    let mut incoming_q = Queue::new(dir.clone())?;
    let mut reading_q = Queue::new(dir.clone())?;
    let mut writing_q = Queue::new(dir)?;

    return Ok((incoming_q, reading_q, writing_q));
}

/// Handle the writing into the socket nonblocking
fn handle_writing(writing_q: &mut Queue<Event>) {
    let ready_writing_events = match writing_q.poll() {
        Ok(events) => events,
        Err(error) => {
            println!("Could not poll writing event from queue.");
            return;
        }
    };
    for mut event in ready_writing_events {
        let bytes_written = match event.stream.write(&mut event.data) {
            Err(error) => 0,
            Ok(bytes_read) => bytes_read,
        };
        if bytes_written != event.data.len() {
            println!("Not all written: buf: {}, written: {}", event.data.len(), bytes_written);
            //todo back in queue with the rest of the work
        }
    }
}

/// Handle the reading from the socket nonblocking
fn handle_reading(mut reading_q: &mut Queue<Event>, writing_q: &mut Queue<Event>) {
    let ready_reading_events = match reading_q.poll() {
        Ok(events) => events,
        Err(error) => {
            println!("Could not poll reading event from queue.");
            return;
        }
    };
    for mut reading_event in ready_reading_events {
        if let Err(error) = reading_event.stream.read(&mut reading_event.data) {
            let response = Response::default_bad_request().make_sendable();
            let event = Event::new_write(reading_event.stream, from_slice(&response[..]));
            let worked = writing_q.add(event);
            if let Err(err) = worked {
                println!("Error while sending response.")
            }
            return;
        };
        let mut response = create_response(reading_event.data, reading_q.dir.clone());
        let event = Event::new_write(reading_event.stream, from_slice(&response[..]));
        let worked = writing_q.add(event);
        if let Err(err) = worked {
            println!("Error while sending response")
        }
    }
}

/// Handle the incoming connection nonblocking
fn handle_incoming(incoming_q: &mut Queue<ListenerEvent>, reading_q: &mut Queue<Event>) {
    let ready_listening_events = match incoming_q.poll() {
        Ok(events) => events,
        Err(error) => {
            println!("Could not poll incoming event from queue.");
            return;
        }
    };
    for listen_event in ready_listening_events {
        match listen_event.listener.accept() {
            Ok((stream, add)) => {
                let read_event = Event::new_read(stream, [0; 2048]);
                let worked = reading_q.add(read_event);
                if let Err(err) = worked {
                    println!("Could not accept connection.");
                    return;
                }
                let worked = incoming_q.add(listen_event);
                if let Err(err) = worked {
                    println!("Could not accept connection.");
                    return;
                }
            }
            Err(err) => { println!("Could not accept connection."); }
        };
    }
}


/// Converts a byte slice into a fixed length portion
fn from_slice(bytes: &[u8]) -> [u8; 2048] {
    let vec: Vec<u8> = bytes.to_vec();
    let mut result = [0; 2048];
    if bytes.len() > 2048 { println!("todo to small buffer"); }
    for i in 0..2048 {
        result[i] = match vec.get(i) {
            Some(val) => *val,
            None => 0,
        }
    }
    result
}


/// Creates a response according to the requested ressource
fn create_response(buffer: Buffer, files: Files) -> Vec<u8> {
    let mut response = Response::default_ok();

    let request = parse_request(buffer);
    let request = match request {
        Ok(request) => request,
        Err(error) => {
            println!("{}", error.description().to_string());
            return Response::default_bad_request().make_sendable();
        }
    };

    let req_file = request.request_identifiers.path;
    let file = match files.get(&req_file) {
        Some(file) => file,
        None => {
            println!("Requested source could not be found");
            return Response::default_not_found().make_sendable();
        }
    };
    response.add_content_type(req_file);
    response.body = file.clone();
    response.make_sendable()
}
