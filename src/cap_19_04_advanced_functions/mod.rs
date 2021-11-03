pub fn run() {
  println!("\n• cap_19_04_advanced_functions");

  // Passing a function as a parameter
  fn add_one(x: i32) -> i32 {
    x + 1
  }

  fn do_twice(my_function: fn(i32) -> i32, arg: i32) -> i32 {
    my_function(arg) + my_function(arg)
  }

  let answer = do_twice(add_one, 5);

  println!("The answer is: {}", answer);

  // In a case where we are using a `.map()`
  let list_of_numbers = vec![1, 2, 3];
  let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

  // We may also use a custom function
  let list_of_numbers = vec![1, 2, 3];
  fn stringify_and_prepend(item: &i32) -> String {
    String::from("prepended-") + &(item).to_string()
  }
  let list_of_strings: Vec<String> = list_of_numbers.iter().map(stringify_and_prepend).collect();

  println!("{:?}", list_of_strings); // ["prepended-1", "prepended-2", "prepended-3"]

  // We can iterate over tuples
  #[derive(Debug)]
  enum Status {
    Value(u32),
    Stop,
  }

  let list_of_statuses: Vec<Status> = (1u32..=5).map(Status::Value).collect();

  println!("{:?}", list_of_statuses); // [Value(1), Value(2), Value(3), Value(4), Value(5)]

  // Returning closures
  // This wont compile
  //
  //    fn returns_closure() -> dyn Fn(i32) -> i32 {
  //      |x| x + 1
  //    }
  //
  // We need to Box both the return type as well as the return value
  fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
  }
}
