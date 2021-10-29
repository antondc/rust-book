mod averaged_collection;

pub fn run() {
  println!("\n• cap_17_01_oop_encapsulation");

  let mut my_averaged_collection = averaged_collection::AveragedCollection::new();
  my_averaged_collection.add(1);
  my_averaged_collection.add(10);
  let average = my_averaged_collection.average();

  println!("{}", average);
}
