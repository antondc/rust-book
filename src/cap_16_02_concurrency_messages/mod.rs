mod multiple_messages;
mod multiple_producers;
mod single_message;

pub fn run() {
  println!("\n• cap_16_01_concurrency_threads_simultaneously");

  single_message::run();
  println!("------------------------------");
  multiple_producers::run();
  println!("------------------------------");
  multiple_messages::run();
  println!("------------------------------");
}
