// As in OOP languages, a struct can hold both data and the logic that mutates this data enabling encapsulation
pub struct AveragedCollection {
  list: Vec<i32>,
  average: f64,
}

impl AveragedCollection {
  pub fn new() -> AveragedCollection {
    AveragedCollection {
      list: vec![],
      average: 0.0,
    }
  }

  pub fn add(&mut self, item: i32) {
    self.list.push(item);
    self.update_average();
  }

  pub fn remove(&mut self) -> Option<i32> {
    let result = self.list.pop();
    match result {
      Some(value) => {
        self.update_average();
        Some(value)
      }
      None => None,
    }
  }

  pub fn average(&self) -> f64 {
    self.average
  }

  pub fn update_average(&mut self) {
    let total: i32 = self.list.iter().sum();
    self.average = total as f64 / self.list.len() as f64
  }
}
