mod move_thread;
mod use_thread;

pub fn run() {
  println!("\n• cap_16_01_concurrency_threads_simultaneously");

  use_thread::run();
  move_thread::run();
}
