use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_millis(10));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}
