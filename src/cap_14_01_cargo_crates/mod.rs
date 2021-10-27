pub mod art;
pub mod cargo_crates;
use art::mix;
use art::PrimaryColor;

pub fn run() {
  let result = cargo_crates::add_one(1);
  println!("{}", result);

  let result = mix(PrimaryColor::Blue, PrimaryColor::Red);
  println!("{:?}", result);
}
