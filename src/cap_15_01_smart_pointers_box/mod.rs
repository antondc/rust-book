mod list;
use list::List::{Cons, Nil};

pub fn run() {
  println!("\n•   cap_15_01_smart_pointers_box");

  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("{:?}", list);
  let b = Box::new(5);
  println!("b = {}", b);
}
