mod front_of_house;

use front_of_house::hosting;

pub fn run() {
  println!("\nā¢āā cap_07_restaurant");

  hosting::add_to_waitlist();
  front_of_house::hosting::add_to_waitlist();
}
