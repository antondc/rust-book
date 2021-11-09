use super::worker::Worker;
use super::Message;
use std::fmt;
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

    let (sender, receiver) = mpsc::channel(); // Create sender and received from channel
    let receiver = Arc::new(Mutex::new(receiver)); // Reference (Arc) that wont conflict between threads (Mutex)
    let mut workers = Vec::with_capacity(size); // Vector where we will store the workers

    // Iterate size to store workers in `workers` vector
    // Each worker instance will hold a reference to the receiver
    for id in 0..size {
      let cloned_receiver = Arc::clone(&receiver);
      let worker = Worker::new(id, cloned_receiver);

      workers.push(worker); // Set workers in place at `ThreadPool.workers` waiting for tasks
    }

    Ok(ThreadPool { workers, sender })
  }

  pub fn execute<F>(&self, incoming_task: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let task = Box::new(incoming_task);
    self.sender.send(Message::NewTask(task)).unwrap(); // Send a new task to the workers
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

pub struct ThreadPoolError(pub String);

impl fmt::Debug for ThreadPoolError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.0)
  }
}
