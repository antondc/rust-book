mod create_list;
mod create_tree;
mod list;
mod node;

pub fn run() {
  println!("\n• cap_15_06_smart_pointers_leaks");

  create_list::create_list();
  create_tree::create_tree();
}
