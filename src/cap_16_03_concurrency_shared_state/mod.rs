mod automatically_reference_counted;
mod mutex_demonstration;

pub fn run() {
  println!("\n• cap_16_03_concurrency_shared_state");

  mutex_demonstration::run();
  println!("------------------------------");
  automatically_reference_counted::run();
  println!("------------------------------");
}
