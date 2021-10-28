use super::list::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_list() {
  println!("\n{}", "- create_list");

  // Instantiate our list with `{ 1, Nil }`
  let a = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));

  println!("a = {:?}", a);
  // Count refs, only one
  println!("a initial rc count = {}", Rc::strong_count(&a));
  // Get last item — `{ Nil }`
  println!("a last item = {:?}", a.tail());
  println!("------------------------------");

  // Create a new list with one item, and `a` list
  let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
  println!("b = {:?}", b);
  // Count refs to `a`: 2
  println!("a rc count after b creation = {}", Rc::strong_count(&a));
  // Count refs to `b`: 1
  println!("b initial rc count = {}", Rc::strong_count(&b));
  // Get `b` last item, which will be `a`: `{ 1, Nil }`
  println!("b next item = {:?}", b.tail());
  println!("------------------------------");

  // From `a` tail, get last item; if some, borrow it and modify pointing it to `b`.
  // This creates a cycle: an infinite loop of two references pointing to each other
  // overflowing the stack
  if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b);
  }

  // Now `a` last item is a reference to `b`,
  // If following line is uncommented this will be log to console
  // println!("a = {:?}", a.tail());
  println!("a rc count after changing a = {}", Rc::strong_count(&a));
  println!("b rc count after changing a = {}", Rc::strong_count(&b));
  println!("------------------------------");
}
