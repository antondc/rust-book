use super::worker::Worker;
use super::Message;
pub use super::ThreadPoolError;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
  pub workers: Vec<Worker>,
  pub sender: mpsc::Sender<Message>,
}

/// Create a new thread pool
///
/// The size is the number of threads in the pool.
///
/// # Panics
///
/// The `new` function will panic if the size is zero.
impl ThreadPool {
  pub fn new(size: usize) -> Result<ThreadPool, ThreadPoolError> {
    if size <= 0 {
      let error = ThreadPoolError(String::from("ERROR: thread pool size not valid"));
      return Err(error);
    }
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      let cloned_receiver = Arc::clone(&receiver);
      workers.push(Worker::new(id, cloned_receiver));
    }

    Ok(ThreadPool { workers, sender })
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");

    for _ in &self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    println!("Shutting down all workers.");

    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);

      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}
