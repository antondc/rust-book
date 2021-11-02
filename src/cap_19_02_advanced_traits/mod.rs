mod generic_type_and_overloading;
mod multiple_traits_same_methods;
mod newtype;
mod supertraits;

pub fn run() {
  println!("\n• cap_19_02_advanced_traits");

  generic_type_and_overloading::run();
  println!("------------------------------");
  multiple_traits_same_methods::run();
  println!("------------------------------");
  supertraits::run();
  println!("------------------------------");
  newtype::run();
}
