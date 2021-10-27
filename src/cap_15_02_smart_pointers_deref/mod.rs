mod cap_15_02_smart_pointers_deref;
use cap_15_02_smart_pointers_deref::MyBox;
pub mod hello;

pub fn run() {
  println!("\n• cap_15_02_smart_pointers_deref-------");
  let x = 5;
  let y = MyBox::new(x);

  // We can dereference boxed x as it implements defer method, which returns `&self.0`
  assert_eq!(5, x);
  assert_eq!(5, *y);

  // Hello will work with &str, &String, and their boxed variants
  let string_str = "Rust";
  let string_str_boxed = MyBox::new("Rust");
  let string_string = String::from("Rust");
  let string_string_boxed = MyBox::new(String::from("Rust"));

  hello::hello(&string_str);
  hello::hello(&string_str_boxed);
  hello::hello(&string_string);
  hello::hello(&string_string_boxed);
}
