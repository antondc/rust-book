pub fn run() {
  println!("\n• cap_18_02_patterns_refutability");

  // Will return irrefutable pattern warning
  // if let x = 5 {
  //   println!("{}", x);
  // };

  let some_option_value = Some(1);
  // Following line will return error, as we are not considering None
  // let Some(x) = some_option_value;
  if let Some(x) = some_option_value {
    println!("{}", x);
  }
}
