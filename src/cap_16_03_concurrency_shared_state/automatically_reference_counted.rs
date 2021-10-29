use std::sync::{Arc, Mutex};
use std::thread;

// Goal is to communicate both threads sharing the same memory position
pub fn run() {
  // Use `Arc` —Atomically Reference Counted—, which exposes same API as `Rc`
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    // For each iteration, clone counter in order to update it
    let counter = Arc::clone(&counter);

    let handle = thread::spawn(move || {
      // Create new thread and lock `counter` to mutate it; after drop, `counter` will be unlocked.
      let mut num = counter.lock().unwrap();

      *num += 1;
    });

    handles.push(handle);
  }

  for handle in handles {
    // Ensure that all threads are executed
    handle.join().unwrap();
  }

  // Get `counter` value, lock and use it
  println!("Result: {}", *counter.lock().unwrap());
}
