pub fn run() {
  println!("\n• cap_18_01_patterns_places");

  let favorite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favorite_color {
    println!("Using your favorite color, {}, as the background", color);
  } else if is_tuesday {
    println!("Tuesday is green day!");
  } else if let Ok(age) = age {
    if age > 30 {
      println!("Using purple as the background color");
    } else {
      println!("Using orange as the background color");
    }
  } else {
    println!("Using blue as the background color");
  }

  println!("------------------------------");
  let mut stack: Vec<&str> = Vec::new();

  stack.push("a");
  stack.push("b");
  stack.push("c");

  while let Some(top) = stack.pop() {
    println!("{}", top);
  }

  println!("------------------------------");
  let v = vec!['a', 'b', 'c'];

  for (index, item) in v.iter().enumerate() {
    println!("{:?}: {:?}", index, item);
  }

  println!("------------------------------");
  let (x, y, z) = (1, 2, 3);

  println!("{}, {}, {}", x, y, z);

  println!("------------------------------");
  // Destructuring in parameter
  fn print_coordinates((x, y): (i32, i32)) {
    println!("Current location: ({}, {})", x, y);
  }
  let point = (3, 5);
  print_coordinates(point);
}
