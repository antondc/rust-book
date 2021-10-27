pub struct CustomSmartPointer {
  pub data: String,
}

// Implementing Drop trait for CustomSmartPointer. Will be called when value is dropped, both automatically or manually with `std::mem::drop`
impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}
