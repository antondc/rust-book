use std::fmt;
use std::io::{Error, Result as IoResult};

pub fn run() {
  println!("\n• cap_19_03_advanced_types");

  // · Creating Type Synonyms with Type Aliases
  // We can create a simple type alias
  type Kilometers = i32;
  let x: i32 = 5;
  let y: Kilometers = 5;
  println!("x + y = {}", x + y);

  // We can create complex types
  type MyTuple = (String, String);
  let my_tuple: MyTuple = (String::from("my"), String::from("tuple"));
  println!("{:?}", my_tuple);

  type Thunk = Box<dyn Fn() + Send + 'static>;

  fn takes_and_returns_long_type(thunk: Thunk) -> Thunk {
    thunk
  }

  // Common use case is with Result<> type
  pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
  }

  // As type Result<T> = std::result::Result<T, std::io::Error> we can also do:
  // Usually we wont rename it
  pub trait WriteWithIoResult {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize>;
    fn flush(&mut self) -> IoResult<()>;

    fn write_all(&mut self, buf: &[u8]) -> IoResult<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> IoResult<()>;
  }

  // · The Never Type that Never Returns
  // …

  // Dynamically Sized Types and the Sized Trait
  // Every generic
  fn generic_a<T>(t: T) {}
  // is actually treated as though we had written this:
  fn generic_b<T: Sized>(t: T) {}
  // We can place unknown size types with `?Sized`, and placing the type behind a reference
  fn generic_c<T: ?Sized>(t: &T) {}
}
