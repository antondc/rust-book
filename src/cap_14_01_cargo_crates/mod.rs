pub mod art;
pub mod cargo_crates;
use add_one;
use art::mix;
use art::PrimaryColor;

pub fn run() {
  println!("•   cap_14_01_cargo_crates-------");

  let result = cargo_crates::add_one(1);
  println!("{}", result);

  let result = mix(PrimaryColor::Blue, PrimaryColor::Red);
  println!("{:?}", result);

  let result = add_one::add_one(1);
  println!("{:?}", result);
}
