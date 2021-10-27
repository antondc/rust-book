mod cap_15_01_smart_pointers;
use cap_15_01_smart_pointers::List::{Cons, Nil};

pub fn run() {
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("{:?}", list);
  let b = Box::new(5);
  println!("b = {}", b);
}
