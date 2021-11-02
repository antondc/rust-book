use std::fmt;

pub fn run() {
  trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
      let output = self.to_string();
      let len = output.len();
      println!("{}", "*".repeat(len + 4));
      println!("*{}*", " ".repeat(len + 2));
      println!("* {} *", output);
      println!("*{}*", " ".repeat(len + 2));
      println!("{}", "*".repeat(len + 4));
    }
  }

  struct Point {
    x: i32,
    y: i32,
  }

  impl OutlinePrint for Point {}

  impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}, {})", self.x, self.y)
    }
  }
  let point = Point { x: 12, y: 54 };
  point.outline_print();
}
