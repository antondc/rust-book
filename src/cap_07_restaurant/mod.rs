mod front_of_house;

use front_of_house::hosting;

pub fn run() {
  hosting::add_to_waitlist();
  front_of_house::hosting::add_to_waitlist();
}
