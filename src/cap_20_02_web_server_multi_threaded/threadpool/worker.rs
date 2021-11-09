use super::Message;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/// Handles tasks
///
pub struct Worker {
  pub id: usize,
  pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      // Listen for tasks, idles here
      let message = receiver.lock().unwrap().recv().unwrap();

      // Task received, check if its a task or terminate command
      match message {
        Message::NewTask(task) => {
          println!("Worker {} got a task; executing.", id);

          task(); // Execute actual task
        }
        Message::Terminate => {
          println!("Worker {} was told to terminate.", id);

          break;
        }
      }
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}
