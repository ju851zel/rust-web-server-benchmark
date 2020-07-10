// use crate::Directory;
// use core::ptr;
//
//
// /// from https://gist.github.com/ytomino/03f2f687483674feff1446e64c3fdac9:~:text=EVFILT_READ
// /// use man kevent on mac
// // adds the event readding will modify and not re add event,
// // automatically enabled it when not calling EV_DISABLE
// // must be added to not return the event when it
// // automatically gets activated when adding to q
//
// const EVFILT_WRITE: i16 = -2;
// const EVFILT_READ: i16 = -1;
// const EV_ADD: u16 = 1;
// const EV_ENABLE: u16 = 4;
// const EV_ONESHOT: u16 = 16;
//
// #[link(name = "c")]
// extern "C" {
//     // call of the C function, returning the fd
//     fn kqueue() -> i32;
//
//     fn kevent(
//         kq: i32, // the file descriptor of the q
//         changelist: *const KeventInternal, // pointer to array of KeventInternal, put in the ones that should be added
//         nchanges: i32, // size of the changelist array
//         eventlist: *const KeventInternal, // pointer to array of the finished KeventInternal
//         nevents: i32, // size of eventlist array
//         timeout: *const Timespec,
//     ) -> i32;
//
//     fn close(d: i32) -> i32;
// }
//
// pub fn create_kqueue() -> Result<i32, String> {
//     let file_desc = unsafe { kqueue() };
//     if file_desc < 0 {
//         return Err(String::from("Error creating new k queue"));
//     }
//     return Ok(file_desc);
// }
//
//
// fn create_k_event(filter: i16, listener_fd: u64) -> KeventInternal {
//     KeventInternal {
//         ident: listener_fd,
//         filter,
//         flags: EV_ADD | EV_ENABLE | EV_ONESHOT,
//         fflags: 0,
//         data: 0,
//         udata: 0,
//     }
// }
//
// pub fn create_k_read_event(fd: u64) -> KeventInternal {
//     create_k_event(EVFILT_READ, fd)
// }
//
// pub fn create_k_write_event(fd: u64) -> KeventInternal {
//     create_k_event(EVFILT_WRITE, fd)
// }
//
//
// pub fn put_kevent_in_kqueue(fd: i32, event: &KeventInternal, time_spec: & Timespec) -> Result<(), String> {
//     let worked = unsafe {
//         kevent(
//             fd,
//             event,
//             1,
//             ptr::null_mut(),
//             0,
//             time_spec,
//         )
//     };
//     if worked < 0 {
//         return Err(String::from("Could not insert event into q"));
//     }
//     Ok(())
// }
//
// pub fn poll_kevents_from_q(fd: i32, timeout: &Timespec) -> Result<Vec<KeventInternal>, String> {
//     let mut finished_events: Vec<KeventInternal> = Vec::with_capacity(256);// todo change to proper size/
//     let res = unsafe {
//         kevent(
//             fd,
//             ptr::null(),
//             0,
//             finished_events.as_mut_ptr(),
//             finished_events.capacity() as i32,
//             timeout,
//         )
//     };
//     if res < 0 {
//         return Err(String::from("Could not wait for event"));
//     };
//
//     unsafe { finished_events.set_len(res as usize) };
//     return Ok(finished_events);
// }
//
//
// //identified by ident,filter and udata
// #[derive(Debug, PartialEq)]
// #[repr(C)]
// pub struct KeventInternal {
//     pub ident: uintptr_t,
//     pub filter: int16_t,
//     pub flags: uint16_t,
//     pub fflags: uint32_t,
//     pub data: intptr_t,
//     pub udata: void_ptr,
// }
//
//
// type uintptr_t = u64;
// type int16_t = i16;
// type uint16_t = u16;
// type uint32_t = u32;
// type intptr_t = i64;
// type void_ptr = u64;
//
//
// #[derive(Debug)]
// #[repr(C)]
// pub struct Timespec {
//     tv_sec: isize,
//     v_nsec: usize,
// }
//
//
// impl Timespec {
//     pub fn of(msec: i32) -> Self {
//         Timespec {
//             tv_sec: (msec / 1000) as isize,
//             v_nsec: ((msec % 1000) * 1000000) as usize,
//         }
//     }
//     pub fn zero() -> Self {
//         Timespec {
//             tv_sec: 0 as isize,
//             v_nsec: 0 as usize,
//         }
//     }
// }