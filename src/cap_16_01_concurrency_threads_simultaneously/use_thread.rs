use std::thread;
use std::time::Duration;

pub fn run() {
  // New thread. Saving it into a variable and calling `handle.join` will ensure that the whole thread is consumed
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(100));
    }
  });

  // If we called hadle.join here, the handled thread will run entirely before main thread
  // handle.join().unwrap();

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(100));
  }

  handle.join().unwrap();
}
