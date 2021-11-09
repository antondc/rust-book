use std::fmt;

pub struct ThreadPoolError(pub String);

impl fmt::Debug for ThreadPoolError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.0)
  }
}
