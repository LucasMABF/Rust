use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};

struct Worker{
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker{
    fn new(id: usize, rx: Arc<Mutex<Receiver<Job>>>) -> Worker{
        Worker{
            id,
            thread: Some(thread::spawn(move ||{
                loop{
                    println!("Before waiting");
                    let message = rx.lock().unwrap().recv(); // lock dropped before end of this line and waits for a job

                    match message{
                        Ok(job) => {
                            println!("Worker {id} got a job; executing.");

                            job();
                            println!("Worker {id} fineshed!");
                        },
                        Err(a) => {
                            println!("Worker {id} disconnected; shutting down. Err: {a}");
                            break;
                        }
                    }
                }
            })),
        }
    }
}

pub struct ThreadPool{
    workers : Vec<Worker>,
    sender: Option<Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
       }

        ThreadPool{
            workers,
            sender: Some(sender),
        }
    }

    pub fn spawn(&self, f: impl FnOnce() + Send + 'static){
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self){
        drop(self.sender.take());
        for worker in &mut self.workers{
            println!("shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}
