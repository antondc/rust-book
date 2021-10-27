mod limit_tracker;
mod list;
use list::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
  println!("\n• cap_15_05_smart_pointers_refcell-------");

  let value = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
  let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

  *value.borrow_mut() += 10;

  println!("a after = {:?}", a);
  println!("b after = {:?}", b);
  println!("c after = {:?}", c);
}
