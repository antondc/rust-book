pub fn run() {
  println!("\n• cap_18_02_patterns_refutability");

  // Irrefutable pattern warning
  if let x = 5 {
    println!("{}", x);
  };

  let some_option_value = Some(1);
  // let Some(x) = some_option_value; // Will return error, as we are not considering None
  if let Some(x) = some_option_value {
    println!("{}", x);
  }
}
