mod post;
use post::Post;

pub fn run() {
  println!("\n• cap_17_03_01_oop_state_pattern");

  let mut post = Post::new();

  post.add_text("Lorem ipsum dolor.");
  assert_eq!("", post.get_content());

  post.request_review();
  assert_eq!("", post.get_content());

  post.approve();
  post.approve();
  assert_eq!("Lorem ipsum dolor.", post.get_content());

  let content = post.get_content();
  println!("{}", content);
}
