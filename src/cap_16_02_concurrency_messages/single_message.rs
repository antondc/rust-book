use std::sync::mpsc;
use std::thread;

pub fn run() {
  // «multiple producer, single consumer» channel
  let (tx, rx) = mpsc::channel();

  // Create a thread, and send a message from the transmitter
  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    // Val already sent, we don't own it: following line will return compile error
    // println!("val is {}", val);
  });

  let received_message = rx.recv().unwrap();
  println!("Got {}", received_message)
}
