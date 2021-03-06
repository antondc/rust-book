use ::std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}
