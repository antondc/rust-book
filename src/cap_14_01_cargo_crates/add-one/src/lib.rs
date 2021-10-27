pub fn add_one(x: i32) -> i32 {
  x + 1
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn adds_one_plus_one() {
    assert_eq!(add_one(1), 2);
  }
}
