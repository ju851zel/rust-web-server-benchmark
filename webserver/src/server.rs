use std::sync::{mpsc::Sender, mpsc::channel, mpsc::Receiver, Arc, Mutex};
use std::thread;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use crate::requests::Request;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;


/// Starts the server listening on the address,
/// with the amount of threads provided by thread_pool_size.
pub fn start_server(address: &str, thread_pool_size: usize, dir: Arc<HashMap<String, String>>) {
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
        let dir = dir.clone();
        pool.execute(|| {
            handle_connection(connection, dir);
        });
    }
}


fn handle_connection(mut stream: TcpStream, dir: Arc<HashMap<String, String>>) {
    let mut buffer = [0; 2048];
    let response200 = "HTTP/1.1 200 OK\r\n\r\n";
    let response404 = "HTTP/1.1 404 Not found\r\n\r\n";

    stream.read(&mut buffer).unwrap();
    let buffer = String::from_utf8(buffer.to_vec());
    let request = Request::read_request(&buffer.unwrap());
    // println!("Request came in, request: {:#?}", request);
    let path = match request {
        Ok(request) => request.start_line.path,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let response = if dir.contains_key(&path) {
        println!("return content to path: {}", path);
        format!("{}{}", response200, dir.get(&path).unwrap())
    } else {
        println!("Could not find path {} in {:#?}", path, dir);
        response404.to_string()
    };

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

    pub fn execute<F>(&self, function: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(function);
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
