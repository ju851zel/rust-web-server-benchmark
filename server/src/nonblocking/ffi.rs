use std::ptr;
use std::net::{TcpStream, TcpListener};
use std::os::unix::io::AsRawFd;
use std::borrow::Borrow;
use std::io::{Read, Write};
use futures::io::ErrorKind;
use crate::response::Response;
use crate::request::Request;

///from https://gist.github.com/ytomino/03f2f687483674feff1446e64c3fdac9:~:text=EVFILT_READ
/// use man kevent on mac
//
pub const EVFILT_WRITE: i16 = -2;
pub const EVFILT_READ: i16 = -1;
//adds the event readding will modify and not re add event,
// automatically enabled it when not calling EV_DISABLE
pub const EV_ADD: u16 = 1;
// must be added to not return the event when it
// automatically gets activated when adding to q
pub const EV_ENABLE: u16 = 4;
pub const EV_ONESHOT: u16 = 16;

#[link(name = "c")]
extern "C" {
    // returns a file descriptor to a new kernel event q / -1 if error
    // notify user when kernel event (kevent) happens
    pub fn kqueue() -> i32; //file descriptor

    //return the number of events placed in eventlist up to number of nevents
    pub fn kevent(
        kq: i32, //the file descriptor
        changelist: *const KeventInternal, //pointer to array of Kevent
        nchanges: i32, // size of the changelist array
        eventlist: *const KeventInternal, // pointer to array of out Kevent structs
        nevents: i32, // size of eventlist
        timeout: *const Timespec,//todo will always be null pointer
    ) -> i32;

    pub fn close(d: i32) -> i32;
}

// #[derive(Debug)]
pub struct Queue {
    // unique identifier for this event
    pub events: Vec<Event>,
    pub wait_timeout: Timespec,
    pub(crate) fd: i32,
}

pub struct ListenerQueue {
    // unique identifier for this event
    pub events: Vec<ListenerEvent>,
    pub wait_timeout: Timespec,
    pub(crate) fd: i32,
}

impl Queue {
    pub fn new() -> Result<Queue, String> {
        let fd = unsafe { kqueue() };
        if fd < 0 {
            return Err(String::from("Error creating new event queue"));
        }
        Ok(Queue { events: vec![], wait_timeout: Timespec::zero(), fd })
    }

    pub fn add(&mut self, event: Event) -> Result<(), String> {
        self.events.push(event);
        let worked = unsafe {
            kevent(
                self.fd,
                self.events.last().unwrap().kevent.as_ptr(),
                1,
                ptr::null_mut(),
                0,
                &self.wait_timeout,
            )
        };
        if worked < 0 {
            return Err(String::from("Could not insert event into q"));
        }
        return Ok(());
    }
    pub fn wait_for_read_data(&mut self) -> Result<(Event, Vec<u8>), String> {
        let mut finished_events = Vec::with_capacity(16);//todo change to proper size/
        loop {
            let res = unsafe {
                kevent(
                    self.fd,
                    ptr::null(),
                    0,
                    finished_events.as_mut_ptr(),
                    finished_events.capacity() as i32,
                    ptr::null(),
                )
            };
            // This result will return the number of events which occurred
            // (if any) or a negative number if it's an error.
            if res < 0 {
                return Err(String::from("Could not wait for event"));
            };
            if res > 0 {
                unsafe { finished_events.set_len(res as usize) };
                println!("res: {:#?}, finished_events: {:#?}", res, finished_events);
                break;
            }
        }

        let index = self.events.iter()
            .position(|ev| ev.kevent[0].ident == finished_events[0].ident);

        if index.is_some() {
            let removed = self.events.remove(index.unwrap());
            let mut event = Event {
                data: [0; 1024],
                stream: removed.stream,
                kevent: removed.kevent,
            };
            let bytes_read = match event.stream.read(&mut event.data) {
                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                    println!("Would block");
                    0
                }
                Err(error) => {
                    println!("{}", error);
                    0
                }
                Ok(bytes_read) => {
                    println!("I read {} bytes", bytes_read);
                    bytes_read
                }
            };

            let mut response = Response::default_ok();

            let utf8_buffer = String::from_utf8(event.data.to_vec());
            if utf8_buffer.is_err() {
                println!("Request could not be interpreted as utf-8");
            };

            let request = Request::read_request(&utf8_buffer.unwrap());
            if request.is_err() {
                println!("Request is invalid");
            } else {
                println!("Request: {:#?}", request.unwrap());
            };
            response.body = "<h1>test</h1>".to_string().into_bytes();
            return Ok((event, response.make_sendable()));
        }

        // println!("Event came in: {:#?}", events.first());
        return Err("Error in .....".to_string());
    }

    pub fn wait_for_write_data(&mut self) -> Result<(), String> {
        let mut finished_events = Vec::with_capacity(16);//todo change to proper size/
        loop {
            let res = unsafe {
                kevent(
                    self.fd,
                    ptr::null(),
                    0,
                    finished_events.as_mut_ptr(),
                    finished_events.capacity() as i32,
                    ptr::null(),
                )
            };
            // This result will return the number of events which occurred
            // (if any) or a negative number if it's an error.
            if res < 0 {
                return Err(String::from("Could not wait for event"));
            };
            if res > 0 {
                unsafe { finished_events.set_len(res as usize) };
                println!("res: {:#?}, finished_events: {:#?}", res, finished_events);
                break;
            }
        }

        let index = self.events.iter()
            .position(|ev| ev.kevent[0].ident == finished_events[0].ident);

        if index.is_some() {
            let mut event = self.events.remove(index.unwrap());
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
                    println!("I read {} bytes", bytes_read);
                    bytes_read
                }
            };

            if bytes_written == event.data.len(){
                return Ok(())
            } else {
                println!("Not all written: buf: {}, written: {}", event.data.len(), bytes_written);
                return Ok(())
            }
        }

        return Err("Error in .....".to_string());
    }
}

impl ListenerQueue {
    pub fn new() -> Result<Self, String> {
        let fd = unsafe { kqueue() };
        if fd < 0 {
            return Err(String::from("Error creating new event queue"));
        }
        Ok(Self { events: vec![], wait_timeout: Timespec::zero(), fd })
    }

    pub fn add(&mut self, event: ListenerEvent) -> Result<(), String> {
        self.events.push(event);
        let worked = unsafe {
            kevent(
                self.fd,
                self.events.last().unwrap().kevent.as_ptr(),
                1,
                ptr::null_mut(),
                0,
                &self.wait_timeout,
            )
        };
        if worked < 0 {
            return Err(String::from("Could not insert event into q"));
        }
        return Ok(());
    }
    pub fn wait(&mut self) -> Result<(ListenerEvent, TcpStream), String> {
        let mut finished_events = Vec::with_capacity(16);//todo change to proper size/
        loop {
            let res = unsafe {
                kevent(
                    self.fd,
                    ptr::null(),
                    0,
                    finished_events.as_mut_ptr(),
                    finished_events.capacity() as i32,
                    ptr::null(),
                )
            };
            // This result will return the number of events which occurred
            // (if any) or a negative number if it's an error.
            if res < 0 {
                return Err(String::from("Could not wait for event"));
            };
            if res > 0 {
                unsafe { finished_events.set_len(res as usize) };
                println!("res: {:#?}, finished_events: {:#?}", res, finished_events);
                break;
            }
        }

        let index = self.events.iter()
            .position(|ev| ev.kevent[0].ident == finished_events[0].ident);

        if index.is_some() {
            let removed = self.events.remove(index.unwrap());
            let mut event = ListenerEvent {
                data: [0; 1024],
                listener: removed.listener,
                kevent: removed.kevent,
            };
            let tcp_stream = event.listener.accept().unwrap().0;
            return Ok((event, tcp_stream));
        }

        // println!("Event came in: {:#?}", events.first());
        return Err("Error accepting".to_string());
    }
}

//identified by ident,filter and udata
// #[derive(Debug)]
pub struct Event {
    //todo change to request or sth like that
    pub data: [u8; 1024],
    pub stream: TcpStream,
    // the internal C representation of the Event
    pub kevent: [KeventInternal; 1],
}

pub struct ListenerEvent {
    //todo change to request or sth like that
    pub data: [u8; 1024],
    pub listener: TcpListener,
    // the internal C representation of the Event
    pub kevent: [KeventInternal; 1],
}


impl Event {
    pub(crate) fn new_read(stream: TcpStream, data: [u8; 1024]) -> Self {
        Self {
            data,
            kevent: [KeventInternal {
                ident: stream.as_raw_fd() as u64,
                filter: EVFILT_READ,
                flags: EV_ADD | EV_ENABLE | EV_ONESHOT,
                fflags: 0,
                data: 0,
                udata: 0,
            }],
            stream,
        }
    }
    pub(crate) fn new_write(stream: TcpStream, data: [u8; 1024]) -> Self {
        Self {
            data,
            kevent: [KeventInternal {
                ident: stream.as_raw_fd() as u64,
                filter: EVFILT_WRITE,
                flags: EV_ADD | EV_ENABLE | EV_ONESHOT,
                fflags: 0,
                data: 0,
                udata: 0,
            }],
            stream,
        }
    }
}

impl ListenerEvent {
    pub(crate) fn new(stream: TcpListener, data: [u8; 1024]) -> Self {
        Self {
            data,
            kevent: [KeventInternal {
                ident: stream.as_raw_fd() as u64,
                filter: EVFILT_READ,
                flags: EV_ADD | EV_ENABLE | EV_ONESHOT,
                fflags: 0,
                data: 0,
                udata: 0,
            }],
            listener: stream,
        }
    }
}

//identified by ident,filter and udata
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct KeventInternal {
    /* identifier for this event */
    pub ident: uintptr_t,
    /* filter for event */
    pub filter: int16_t,
    /* general flags */
    pub flags: uint16_t,
    /* filter-specific flags */
    pub fflags: uint32_t,
    /* filter-specific data */
    pub data: intptr_t,
    /* opaque user data identifier */
    pub udata: void_ptr,
}


type uintptr_t = u64;
type int16_t = i16;
type uint16_t = u16;
type uint32_t = u32;
type intptr_t = i64;
type void_ptr = u64;


#[derive(Debug)]
#[repr(C)]
pub struct Timespec {
    tv_sec: isize,
    v_nsec: usize,
}


impl Timespec {
    pub fn of(msec: i32) -> Self {
        Timespec {
            tv_sec: (msec / 1000) as isize,
            v_nsec: ((msec % 1000) * 1000000) as usize,
        }
    }
    pub fn zero() -> Self {
        Timespec {
            tv_sec: 0 as isize,
            v_nsec: 0 as usize,
        }
    }
}