use crate::nonblocking::ffi;
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::AsRawFd;
use core::ptr;
use crate::nonblocking::ffi::{Queue, Event, kevent, KeventInternal};
use futures::future::err;


pub(crate) fn main() {
    let mut incoming_queue = Queue::new().unwrap();


    let listener = match TcpListener::bind(format!("127.0.0.1:{}", 9005)) {
        Ok(listener) => listener,
        Err(err) => panic!(err)
    };

    loop {
        let stream = listener.accept().unwrap().0;
        stream.set_nonblocking(true).unwrap();

        let event = Event::new(stream, [0; 1024]);
        incoming_queue.add(event).unwrap();

        if incoming_queue.events.len() > 0 {
            incoming_queue.wait();
        }
    }
}

