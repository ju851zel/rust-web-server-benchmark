// use std::net::{TcpStream, TcpListener};
// use std::os::unix::io::AsRawFd;
// use crate::{Directory, Buffer};
// use crate::event_loop::unsafe_c::{Timespec, create_kqueue, put_kevent_in_kqueue, poll_kevents_from_q, create_k_read_event, create_k_write_event, KeventInternal};
//
// /// The Queue holding events and a reference to the kqueue
// pub struct Queue<T> where T: GeneralEvent {
//     pub events: Vec<T>,
//     pub wait_timeout: Timespec,
//     pub dir: Directory,
//     pub fd: i32,
// }
//
// impl<T> Queue<T> where T: GeneralEvent {
//     /// Creates a new k queue
//     pub fn new(dir: Directory) -> Result<Queue<T>, String> {
//         Ok(Self {
//             events: vec![],
//             wait_timeout: Timespec::zero(),
//             fd: create_kqueue()?,
//             dir,
//         })
//     }
//
//     /// Adds a given element into the kqueue
//     pub fn add(&mut self, event: T) -> Result<(), T> {
//         self.events.push(event);
//         let kevent = self.events.last().unwrap().get_kevent();
//         let worked = put_kevent_in_kqueue(self.fd, &kevent, &self.wait_timeout);
//         if let Err(err) = worked {
//             println!("{}", err);
//             return Err(self.events.remove(self.events.len() - 1));
//         }
//         Ok(())
//     }
//
//     /// Polls the q and retrieves the ready events
//     pub fn poll(&mut self) -> Result<Vec<T>, String> {
//         let finished_events = loop {
//             let results = poll_kevents_from_q(self.fd, &self.wait_timeout)?;
//             if results.len() > 0 {
//                 break results;
//             }
//         };
//
//         let mut indexes = Vec::with_capacity(8);
//         for event in finished_events {
//             let index =
//                 self.events
//                     .iter()
//                     .position(|ev| ev.get_ident() == event.ident);
//             if let Some(idx) = index {
//                 indexes.push(idx)
//             }
//         }
//
//         let mut events = Vec::with_capacity(8);
//         for index in indexes {
//             events.push(self.events.remove(index))
//         }
//         Ok(events)
//     }
// }
//
// /// The Event containing the request data, stream object and the kevent
// pub struct Event {
//     //todo change to request or sth like that
//     pub data: Buffer,
//     pub stream: TcpStream,
//     // the internal C representation of the Event
//     pub kevent: KeventInternal,
// }
//
// /// The Event containing the request data, connection object and the kevent
// pub struct ListenerEvent {
//     //todo change to request or sth like that
//     pub data: Buffer,
//     pub listener: TcpListener,
//     // the internal C representation of the Event
//     pub kevent: KeventInternal,
// }
//
// /// Trait which defines general Event functions used in Event and ListenerEvent
// pub trait GeneralEvent {
//     fn get_ident(&self) -> u64;
//     fn get_kevent(&self) -> &KeventInternal;
// }
//
// impl Event {
//     pub(crate) fn new_read(stream: TcpStream, data: Buffer) -> Self {
//         Self {
//             data,
//             kevent: create_k_read_event(stream.as_raw_fd() as u64),
//             stream,
//         }
//     }
//     pub(crate) fn new_write(stream: TcpStream, data: Buffer) -> Self {
//         Self {
//             data,
//             kevent: create_k_write_event(stream.as_raw_fd() as u64),
//             stream,
//         }
//     }
// }
//
// impl GeneralEvent for Event {
//     fn get_ident(&self) -> u64 {
//         self.kevent.ident
//     }
//     fn get_kevent(&self) -> &KeventInternal {
//         &self.kevent
//     }
// }
//
// impl GeneralEvent for ListenerEvent {
//     fn get_ident(&self) -> u64 {
//         self.kevent.ident
//     }
//
//     fn get_kevent(&self) -> &KeventInternal {
//         &self.kevent
//     }
// }
//
//
// impl ListenerEvent {
//     pub(crate) fn new(listener: TcpListener, data: Buffer) -> Self {
//         Self {
//             data,
//             kevent: create_k_read_event(listener.as_raw_fd() as u64),
//             listener,
//         }
//     }
// }
