use ::std::thread;
use ::std::time::Duration;

pub fn workout_calculator(num: u32) -> u32 {
  println!("Calculating slowly...");
  thread::sleep(Duration::from_secs(2));
  num
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn call_with_different_values() {
    let result_1 = workout_calculator(1);
    let result_2 = workout_calculator(2);

    assert_eq!(result_1, 1);
    assert_eq!(result_2, 2);
  }
}
