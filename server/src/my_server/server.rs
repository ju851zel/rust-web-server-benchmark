use std::sync::{mpsc::Sender, mpsc::channel, mpsc::Receiver, Arc, Mutex};
use std::thread;



pub struct ThreadPool {
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
