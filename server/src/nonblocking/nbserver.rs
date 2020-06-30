use crate::nonblocking::ffi;
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::AsRawFd;
use core::ptr;
use crate::nonblocking::ffi::{Queue, Event, kevent, KeventInternal, ListenerEvent, ListenerQueue};


pub(crate) fn main() {
    let mut incoming_q = ListenerQueue::new().unwrap();
    let mut reading_q = Queue::new().unwrap();


    let listener = match TcpListener::bind(format!("127.0.0.1:{}", 9005)) {
        Ok(listener) => listener,
        Err(err) => panic!(err)
    };

    listener.set_nonblocking(true).unwrap();

    let listener_event = ListenerEvent::new(listener, [0; 1024]);
    incoming_q.add(listener_event);

    loop {
        if reading_q.events.len() > 0 {
            reading_q.wait();
        }
        if incoming_q.events.len() > 0 {
            let (listener, stream) = incoming_q.wait().unwrap();
            let event = Event::new(stream, [0; 1024]);
            reading_q.add(event).unwrap();
            incoming_q.add(listener);
        }

    }


    // stream.set_nonblocking(true).unwrap();
}

