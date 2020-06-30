use std::sync::{mpsc::Sender, mpsc::channel, mpsc::Receiver, Arc, Mutex};
use std::thread;


#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    transmitter: Sender<Job>,
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

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}


impl Worker {
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


trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;
