use std::fmt;

pub fn run() {
  struct MyListWrapper<'a>(Vec<String>, &'a str);

  // We can overwrite fmt formatter to print Self the way we consider
  impl fmt::Display for MyListWrapper<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "custom print: {}, {}", self.0.join(", "), self.1)
    }
  }
  let my_list = vec![String::from("foo"), String::from("bar")];
  let my_list_wrapper = MyListWrapper(my_list, "baz");

  // my_list_wrapper — custom print: foo, bar, baz
  println!("my_list_wrapper — {}", my_list_wrapper);
}
