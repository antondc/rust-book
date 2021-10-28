mod list;
use list::List::{Cons, Nil};
use std::rc::Rc;

pub fn run() {
  println!("\n• cap_15_04_smart_pointers_reference_counted");

  // One reference, Rc::strong_count of a will be 1
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  // Two references, Rc::strong_count of a will be 2
  let b = Cons(3, Rc::clone(&a));
  println!("count after creating b = {}", Rc::strong_count(&a));
  // Three references, Rc::strong_count of a will be 3
  // By creating a new scope, c will be deferenced after the scope finishes, and count will be decreased by one
  {
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  // Two references again
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
