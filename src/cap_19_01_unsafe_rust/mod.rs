// Five possible unsafe actions:

pub fn run() {
  println!("\n• cap_19_01_unsafe_rust");
  // · Dereference a raw pointer
  let mut number = 42;

  let r1 = &number as *const i32;
  let r2 = &mut number as *mut i32;

  // We can get references
  // We can get get references, dereference and reference them again
  println!("&number:           {:p}", &number);
  println!("&*&number:         {:p}", &*&number);
  println!("&*&*&number:       {:p}", &*&*&number);
  println!("r1:                {:p}", &number as *const i32);
  println!("r2:                {:p}", r2);

  // We can not dereference directly
  unsafe {
    println!("dereferencing r2:  {}", *r2);
  }
  // · Call an unsafe function or method
  // · Access or modify a mutable static variable
  // · Implement an unsafe trait
  // · Access fields of unions
}
