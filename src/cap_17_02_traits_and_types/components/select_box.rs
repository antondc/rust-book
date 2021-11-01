use super::super::types::Draw;

pub struct SelectBox {
  pub width: u32,
  pub height: u32,
  pub options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    println!(
      "Drawing a box with height {}, width {} and label {:?}",
      self.height, self.width, self.options
    );
  }
}
