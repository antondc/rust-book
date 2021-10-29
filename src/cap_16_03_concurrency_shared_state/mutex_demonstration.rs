use std::sync::Mutex;

// Demonstration that we can create a `Mutex`, lock it to modify within a scope, and use it after the scope ended
// which means that mutex was unlocked when variable holding it was dropped
pub fn run() {
  let mutex = Mutex::new(5);

  {
    // Lock mutex and save it into a mutable `num` variable
    let mut num = mutex.lock().unwrap();
    // Set `num` value to `6`
    *num = 6;
  }

  // Will print `6`
  println!("m = {:?}", mutex);
}
