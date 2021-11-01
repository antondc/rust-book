pub fn run() {
  println!("\n• cap_18_03_pattern_syntax");

  // Matching Literals
  let x = 1;
  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
  }

  println!("------------------------------");

  let x: Option<i32> = Some(1);
  match x {
    Some(1) => println!("one"),
    Some(2) => println!("two"),
    Some(3) => println!("three"),
    _ => println!("anything"),
  }

  println!("------------------------------");
  // Matching Named Variables
  let x = Some(5);
  let z = 10;
  match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {:?}", x, z);

  println!("------------------------------");
  // Multiple Patterns
  let x = 1;
  match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
  }

  let x: Option<i8> = Some(1);
  match x {
    Some(1) | Some(2) => println!("one or two"),
    Some(3) => println!("three"),
    _ => println!("anything"),
  }

  println!("------------------------------");
  // Matching Ranges of Values with ..=
  let x = 5;
  match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
  }

  let x: Option<i8> = Some(5);
  match x {
    Some(1..=5) => println!("one through five"),
    _ => println!("something else"),
  }

  let x = 'c';
  match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
  }

  println!("------------------------------");
  // Destructuring Structs
  struct Point {
    x: i32,
    y: i32,
  }

  let point = Point { x: 0, y: 7 };
  let Point { x, y } = point;
  println!("a: {}", x);
  println!("b: {}", y);
  let Point { x: foo, y: bar } = point;
  println!("foo: {}", foo);
  println!("bar: {}", bar);

  match point {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }

  println!("------------------------------");
  // Destructuring Enums
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  let msg = Message::ChangeColor(0, 160, 255);
  match msg {
    Message::Quit => {
      println!("The Quit variant has no data to destructure.")
    }
    Message::Move { x, y } => {
      println!("Move in the x direction {} and in the y direction {}", x, y);
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    }
  }

  println!("------------------------------");
  // Nested destructuring
  enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
  }

  enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
  }

  let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
  match msg {
    Message2::ChangeColor(Color::Rgb(r, g, b)) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    }
    Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
      "Change the color to hue {}, saturation {}, and value {}",
      h, s, v
    ),
    _ => (),
  }

  println!("------------------------------");
  let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

  println!("feet: {}, inches: {}", feet, inches);
  println!("x: {}, y: {}", x, y);

  println!("------------------------------");
  // Ignoring values
  fn my_function(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
  }

  my_function(3, 4);

  println!("------------------------------");
  // Ignoring Parts of a Value with a Nested _

  let mut setting_value = Some(5);
  let new_setting_value = Some(10);

  match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
      println!("Can't overwrite an existing customized value");
    }
    _ => {
      setting_value = new_setting_value;
    }
  }

  println!("setting is {:?}", setting_value);

  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, _, third, _, fifth) => {
      println!("Some numbers: {}, {}, {}", first, third, fifth)
    }
  }

  println!("------------------------------");
  // Ignoring an Unused Variable by Starting Its Name with _

  let _x = 5;
  let y = 10;

  let string = Some(String::from("Hello!"));

  if let Some(_) = string {
    println!("found a string");
  }

  println!("{:?}", string);

  println!("------------------------------");
  // Ignoring Remaining Parts of a Value with ..

  struct Point2 {
    foo: i32,
    bar: i32,
    baz: i32,
  }

  let origin = Point2 {
    foo: 0,
    bar: 0,
    baz: 0,
  };

  match origin {
    Point2 { foo, .. } => println!("foo is {}", foo),
  }

  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, .., last) => {
      println!("first: {}, last: {}", first, last);
    }
  }

  println!("------------------------------");
  // Extra Conditionals with Match Guards
  let num = Some(4);

  match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
  }

  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {}", n),
    _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {}", x, y);

  let x = 4;
  let y = false;

  match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
  }

  println!("------------------------------");
  // @ Bindings
  enum Message3 {
    Hello { id: i32 },
  }

  let msg = Message3::Hello { id: 5 };

  match msg {
    Message3::Hello {
      id: id_variable @ 3..=7,
    } => println!("Found an id in range: {}", id_variable),
    Message3::Hello { id: 10..=12 } => {
      println!("Found an id in another range")
    }
    Message3::Hello { id } => println!("Found some other id: {}", id),
  }
}
