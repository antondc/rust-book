use super::node::Node;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn create_tree() {
  println!("{}", "- Create tree");

  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  // No reference to `leaf` parent, as we didnt create `branch` yet
  println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());

  let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
  });

  // Use `borrow_mut` on the RefCell<Weak<Node>> in the parent field of `leaf`
  // then use `Rc::downgrade` to create a `Weak<Node>` reference to `branch` from the `Rc<Node>` in `branch`.
  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

  // Get reference for `leaf` parent —`branch`—
  let leaf_parent = leaf.parent.borrow().upgrade();
  // Now we can print `leaf` parent without a cycle
  println!("leaf parent = {:#?}", leaf_parent);

  println!("------------------------------");
  println!("{}", "- Create new tree with scope");

  // Create new leaf and branch to see how scope drops them

  let new_leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  // Print how many strong and weak refs we currently have at new_leaf
  // strong = 1, weak = 0
  println!(
    "new_leaf: strong = {}, weak = {}",
    Rc::strong_count(&new_leaf),
    Rc::weak_count(&new_leaf),
  );

  {
    let new_branch = Rc::new(Node {
      value: 5,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![Rc::clone(&new_leaf)]),
    });

    // Link new_leaf and new_branch
    *new_leaf.parent.borrow_mut() = Rc::downgrade(&new_branch);

    // Print how many strong and weak refs we currently have at new_leaf
    // strong = 2, weak = 0
    println!(
      "new_leaf: strong = {}, weak = {}",
      Rc::strong_count(&new_leaf),
      Rc::weak_count(&new_leaf),
    );

    // Print how many strong and weak refs we currently have at new_branch
    // strong = 1, weak = 1
    println!(
      "new_branch: strong = {}, weak = {}",
      Rc::strong_count(&new_branch),
      Rc::weak_count(&new_branch),
    );
  }

  // Out of scope, new_branch dropped
  // Print new_leaf parent: None
  println!("new_leaf parent = {:?}", new_leaf.parent.borrow().upgrade());
  // Print new_leaf strong and weak refs: strong = 1, weak = 0
  println!(
    "new_leaf strong: = {}, weak = {}",
    Rc::strong_count(&new_leaf),
    Rc::weak_count(&new_leaf),
  );
}
