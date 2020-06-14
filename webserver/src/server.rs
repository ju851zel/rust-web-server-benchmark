use std::sync::{mpsc::Sender, mpsc::channel, mpsc::Receiver, Arc, Mutex};
use std::thread;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use crate::requests::Request;


/// Starts the server listening on the address,
/// with the amount of threads provided by thread_pool_size.
pub fn start_server(address: &str, thread_pool_size: usize) {
    let pool = match ThreadPool::new(thread_pool_size) {
        Ok(pool) => pool,
        Err(err) => panic!(err)
    };

    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(err) => panic!(err)
    };

    println!("Listening for incoming requests on {}", address);
    for stream in listener.incoming() {
        let connection = stream.unwrap();
        pool.execute(|| {
            handle_connection(connection);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    let response = "HTTP/1.1 200 OK\r\n\r\nhello";

    stream.read(&mut buffer).unwrap();
    let buffer = String::from_utf8(buffer.to_vec());
    println!("Request came in, request: {:#?}", &buffer);
    let request = Request::read_request(&buffer.unwrap());
    println!("Request came in, request: {:#?}", request);
    // println!("Request came in, body: {:#?}", first_line);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


struct ThreadPool {
    workers: Vec<Worker>,
    transmitter: Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, String> {
        if size < 2 { return Err("A thread pool size of 2 or more is required.".to_string()); }

        let (tx, rx) = channel();

        let receiver = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Ok(ThreadPool { workers, transmitter: tx })
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.transmitter.send(job).unwrap();
    }
}


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} starts executing a job", id);
                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}


trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;
