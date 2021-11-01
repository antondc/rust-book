mod post;
use post::Post;

pub fn run() {
  println!("\n• cap_17_03_02_oop_states_as_types");

  let mut post = Post::new();

  post.add_text("Lorem ipsum dolor.");

  let post = post.request_review();

  let post = post.approve();

  assert_eq!("Lorem ipsum dolor.", post.get_content());

  let content = post.get_content();
  println!("{}", content);
}
