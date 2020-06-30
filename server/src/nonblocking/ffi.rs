use std::ptr;
use std::net::TcpStream;
use std::os::unix::io::AsRawFd;
use std::borrow::Borrow;

///from https://gist.github.com/ytomino/03f2f687483674feff1446e64c3fdac9:~:text=EVFILT_READ
/// use man kevent on mac
//
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
        timeout: *const KeventInternal,//todo will always be null pointer
    ) -> i32;

    pub fn close(d: i32) -> i32;
}

#[derive(Debug)]
pub struct Queue {
    // unique identifier for this event
    pub reading: Vec<Event>,
    pub incoming: Vec<Event>,
    pub(crate) fd: i32,
}

impl Queue {
    pub fn new() -> Result<Queue, String> {
        let fd = unsafe { kqueue() };
        if fd < 0 {
            return Err(String::from("Error creating new event queue"));
        }
        Ok(Queue { reading: vec![], incoming: vec![], fd })
    }

    pub fn add(&mut self, event: Event) -> Result<(), String> {
        self.incoming.push(event);
        let worked = unsafe {
            kevent(
                self.fd,
                self.incoming.last().unwrap().kevent.as_ptr(),
                1,
                ptr::null_mut(),
                0,
                ptr::null(),
            )
        };
        if worked < 0 {
            return Err(String::from("Could not insert event into q"));
        }
        return Ok(());
    }
    pub fn wait(&mut self) -> Result<(), String> {
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

        println!("{:#?}", finished_events);
        let index = self.incoming.iter()
            .position(|ev| ev.kevent[0].ident == finished_events[0].ident);

        if index.is_some() {
            let removed = self.incoming.remove(index.unwrap());
            let event = Event {
                data: [u8;1024],
                stream: removed.stream,
                kevent: removed.kevent,
            };
            self.reading.push(event);
        }

        // println!("Event came in: {:#?}", events.first());
        return Ok(());
    }
}

//identified by ident,filter and udata
#[derive(Debug)]
pub struct Event {
    //todo change to request or sth like that
    pub data: [u8;1024],
    pub stream: TcpStream,
    // the internal C representation of the Event
    pub kevent: [KeventInternal; 1],
}


impl Event {
    pub(crate) fn new(stream: TcpStream, data: [u8;1024]) -> Event {
        Event {
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