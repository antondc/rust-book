mod cacher;
mod generate_workout;
mod workout_calculator;

pub fn run() {
  let simulated_user_specified_value = 30;
  let simulated_random_number = 7;

  generate_workout::generate_workout(simulated_user_specified_value, simulated_random_number);

  println!("Hello, world!");
}
