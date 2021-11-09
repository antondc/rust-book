use super::Job;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let job = receiver
        .lock()
        .expect("ERROR: something failed while processing the request")
        .recv()
        .unwrap();
      println!("{}", id);

      job();
    });

    Worker { id, thread }
  }
}
