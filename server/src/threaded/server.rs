use std::sync::{mpsc::Sender, mpsc::channel, mpsc::Receiver, Arc, Mutex};
use std::thread;
use chrono::{NaiveDateTime};


/// The threadpool struct that manages the threads
#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    transmitter: Sender<Job>,
}

/// The struct that manages Stats for the server
#[derive(Debug)]
pub struct ServerStats {
    pub request_results: Mutex<Vec<RequestResult>>
}

/// The struct that manages a single stat about a specific request
#[derive(Debug)]
pub struct RequestResult {
    pub response_code: u32,
    pub requested_resource: String,
    pub time: NaiveDateTime,
    pub response_time: u128,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (tx, rx) = channel();

        let receiver = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, transmitter: tx }
    }

    pub fn execute<F>(&self, function: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(function);
        self.transmitter.send(job).unwrap();
    }
}

/// The worker thread
#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}


impl Worker {
    /// Creates a new worker thread
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        Worker {
            id,
            thread: thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                job.call_box();
            }),
        }
    }
}

/// A trait that makes the storing of the function that should be run when the thread runs possible
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;
