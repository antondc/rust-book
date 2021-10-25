use super::cacher::Cacher;
use super::workout_calculator;

pub fn generate_workout(intensity: u32, random_number: u32) {
  let mut expensive_result = Cacher::new(workout_calculator::workout_calculator);

  if intensity > 25 {
    println!("Today, do {} pushups", expensive_result.value(intensity));
    println!("Next, do {} situps!", expensive_result.value(intensity));
  } else {
    if random_number == 3 {
      println!("Take a break");
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_result.value(intensity)
      );
    }
  }
}
