use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);
        for id in 1..size {
            workers.push(Worker::new(size));
        }

        ThreadPool{
            workers
        }
    }

    pub fn execute<F>(&self, f: F)
        where F : FnOnce() + Send + 'static {

        }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| { });

        Worker {
            id,
            thread,
        }
    }
}