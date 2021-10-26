pub struct Cacher<T: Fn(u32) -> u32> {
  pub calculation: T,
  pub value: Option<u32>,
}

impl<T: Fn(u32) -> u32> Cacher<T> {
  pub fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }

  pub fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use ::std::thread;
  use ::std::time::Duration;
  #[test]
  fn call_with_different_values() {
    fn my_function(num: u32) -> u32 {
      println!("Calculating slowly...");
      thread::sleep(Duration::from_secs(2));

      num
    }

    let mut c = Cacher::new(my_function);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 1);
  }
}
