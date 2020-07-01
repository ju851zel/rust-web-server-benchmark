use crate::event_loop::{ffi, Files};
use std::net::{TcpListener};
use core::ptr;
use crate::event_loop::ffi::{Queue, Event, ListenerEvent};
use std::io::{Read, ErrorKind, Write};
use crate::request::Request;


pub fn start_server(ip: String, port: i32, dir: Files) {
    let address = format!("{}:{}", ip, port);

    let mut incoming_q = Queue::new(dir.clone()).unwrap();
    let mut reading_q = Queue::new(dir.clone()).unwrap();
    let mut writing_q = Queue::new(dir).unwrap();


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
        if incoming_q.events.len() > 0 {
            let ready_listening_events = incoming_q.poll().unwrap();
            for listen_event in ready_listening_events {
                match listen_event.listener.accept() {
                    Ok((stream, add)) => {
                        let read_event = Event::new_read(stream, [0; 2048]);
                        let worked = reading_q.add(read_event);
                        if let Err(err) = worked {
                            println!("Could not accept connection.");
                        }
                        let worked = incoming_q.add(listen_event);
                        if let Err(err) = worked {
                            panic!("Could not accept any more connections.")
                        }
                    }
                    Err(err) => { println!("Could not accept connection."); }
                };
            }
        }
        if reading_q.events.len() > 0 {
            let ready_reading_events = reading_q.poll().unwrap();
            for mut reading_event in ready_reading_events {
                match reading_event.stream.read(&mut reading_event.data) {
                    Err(error) => {
                        println!("{}", error);
                    }
                    Ok(bytes_read) => {
                        println!("I read {} bytes", bytes_read);
                    }
                };
                let mut response = Request::create_response(reading_event.data, reading_q.dir.clone()).make_sendable();
                let event = Event::new_write(reading_event.stream, from_slice(&response[..]));
                let worked = writing_q.add(event);
                if let Err(err) = worked {
                    println!("Error while sending response")
                }
            }
        }
        if writing_q.events.len() > 0 {
            let ready_writing_events = writing_q.poll().unwrap();
            for mut event in ready_writing_events {
                let bytes_written = match event.stream.write(&mut event.data) {
                    Err(error) if error.kind() == ErrorKind::WouldBlock => {
                        println!("Would block");
                        0
                    }
                    Err(error) => {
                        println!("{}", error);
                        0
                    }
                    Ok(bytes_read) => {
                        println!("I wrote {} bytes", bytes_read);
                        bytes_read
                    }
                };
                if bytes_written != event.data.len() {
                    println!("Not all written: buf: {}, written: {}", event.data.len(), bytes_written);
                }
            }
        }
    }
}

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
