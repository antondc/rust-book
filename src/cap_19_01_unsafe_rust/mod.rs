// Five possible unsafe actions:

pub fn run() {
  println!("\n• cap_19_01_unsafe_rust");

  // · Dereference a raw pointer
  let mut number = 42;

  // We can get references as well as dereference and reference them again
  let ref_1 = &number as *const i32;
  let ref_2 = &mut number as *mut i32;

  println!("&number:              {:p}", &number); //                0x7ffee953dcfc
  println!("&*&number:            {:p}", &*&number); //              0x7ffee953dcfc
  println!("&*&*&number:          {:p}", &*&*&number); //            0x7ffee953dcfc
  println!("ref_1:                {:p}", &number as *const i32); //  0x7ffee953dcfc
  println!("ref_2:                {:p}", ref_2); //                     0x7ffee953dcfc

  let address = 0x012345usize;
  let ref_3 = address as *const i32;

  println!("address:  {:#x}", address);
  println!("ref_3:    {:p}", ref_3);

  // We can not dereference directly
  let mut number = 5;

  let ref_4 = &number as *const i32;
  let ref_5 = &mut number as *mut i32;

  // We need `unsafe` to be able to dereference
  // Otherwise we will get «dereference of raw pointer» error
  unsafe {
    println!("ref_4 is: {}", *ref_4);
    println!("ref_5 is: {}", *ref_5);
  }

  println!("------------------------------");

  // · Call an unsafe function or method
  // This function doesn't do anything
  unsafe fn dangerous() {}

  // We need `unsafe` to be able to run this function
  // Otherwise we will get call to unsafe function» error
  unsafe {
    dangerous();
  }

  // ·· Creating a Safe Abstraction over Unsafe Code
  // Lets try to implement a function that takes a reference to a vector and splits it
  let mut my_vector = vec![1, 2, 3, 4, 5, 6];
  let my_vector_ref = &mut my_vector[..];
  let (first_half, second_half) = my_vector_ref.split_at_mut(3); // <== implement this

  println!("first_half:   {:?}", first_half); //  [1, 2, 3]
  println!("second_half:  {:?}", second_half); // [4, 5, 6]

  fn split_at_mut(list: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    use std::slice;
    let len = list.len();
    let ptr = list.as_mut_ptr();

    assert!(mid <= len);

    // Returning next line would result in «cannot borrow `*list` as mutable more than once at a time» error
    // (&mut list[..mid], &mut list[mid..])
    unsafe {
      (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid),
      )
    }
  }

  // ·· Using extern Functions to Call External Code
  // We can call C programs using `extern "C"` statement
  extern "C" {
    fn abs(input: i32) -> i32;
  }

  unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
  }

  println!("------------------------------");

  // · Access or modify a mutable static variable
  static mut COUNTER: u32 = 0;

  fn add_to_count(inc: u32) {
    unsafe {
      COUNTER += inc; // Static variable, with `unsafe` we can modify it
    }
  }

  add_to_count(3);

  unsafe {
    println!("COUNTER: {}", COUNTER);
  }

  // · Implement an unsafe trait
  unsafe trait Foo {
    // methods go here
  }

  unsafe impl Foo for i32 {
    // method implementations go here
  }

  // · Access fields of unions
  // No code
}
