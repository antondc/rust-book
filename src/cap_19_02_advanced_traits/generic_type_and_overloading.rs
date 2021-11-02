use std::ops::Add;

pub fn run() {
  // Customizing Add::Add trait
  #[derive(Debug, Copy, Clone, PartialEq)]
  pub struct Point {
    pub x: i32,
    pub y: i32,
  }

  impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
      Point {
        x: self.x + other.x,
        y: self.y + other.y,
      }
    }
  }

  println!("{:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });
  println!("{:?}", Point { x: 3, y: 3 });

  println!("------------------------------");

  pub struct Meters(pub u32);

  #[derive(Debug)]
  pub struct Millimeters(u32);

  // We can pass meters to avoid default generic type at `pub trait Add<Rhs = Self> `
  impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
      Millimeters(self.0 + (other.0 * 1000))
    }
  }

  let meters = Meters(2);
  let millimeters = Millimeters(2000);
  let result = millimeters.add(meters).0;

  println!("Millimeters: {}", result);
}
