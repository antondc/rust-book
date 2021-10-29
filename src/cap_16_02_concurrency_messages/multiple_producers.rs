use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
  let (tx, rx) = mpsc::channel();

  // Clone transmitter, creating a new sender
  let tx1 = tx.clone();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];
    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("from"),
      String::from("you"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for message in rx {
    println!("received: {}", message)
  }
}
