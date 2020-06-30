use crate::nonblocking::ffi;
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::AsRawFd;
use core::ptr;
use crate::nonblocking::ffi::{Queue, Event, kevent, KeventInternal, ListenerEvent, ListenerQueue};


pub(crate) fn main() {
    let mut incoming_q = ListenerQueue::new().unwrap();
    let mut reading_q = Queue::new().unwrap();
    let mut writing_q = Queue::new().unwrap();


    let listener = match TcpListener::bind(format!("127.0.0.1:{}", 9005)) {
        Ok(listener) => listener,
        Err(err) => panic!(err)
    };

    listener.set_nonblocking(true).unwrap();

    let listener_event = ListenerEvent::new(listener, [0; 1024]);
    incoming_q.add(listener_event);

    loop {
        if reading_q.events.len() > 0 {
            let (event, result) = reading_q.wait_for_read_data().unwrap();
            let event = Event::new_write(event.stream, from_slice(&result[..]));
            writing_q.add(event).unwrap();
        }
        if writing_q.events.len() > 0 {
            let x = writing_q.wait_for_write_data().unwrap();
        }

        if incoming_q.events.len() > 0 {
            let (listener, stream) = incoming_q.wait().unwrap();
            let event = Event::new_read(stream, [0; 1024]);
            reading_q.add(event).unwrap();
            incoming_q.add(listener);
        }
    }


    // stream.set_nonblocking(true).unwrap();
}

fn from_slice(bytes: &[u8]) -> [u8; 1024] {
    let vec: Vec<u8> = bytes.to_vec();
    let mut result = [0; 1024];
    if bytes.len() > 1024 { println!("todo to small buffer"); }
    for i in 0..1024 {
        result[i] = match vec.get(i) {
            Some(val) => *val,
            None => 0,
        }
    }
    result
}
