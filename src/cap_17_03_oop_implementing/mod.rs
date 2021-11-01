mod post;
use post::Post;

pub fn run() {
  println!("\n• cap_17_03_oop_implementing");

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

  println!("------------------------------");
  let mut post_2 = Post::new();

  post_2.add_text("Lorem ipsum dolor.");
  assert_eq!("", post_2.get_content());

  post_2.request_review();
  assert_eq!("", post_2.get_content());

  post_2.reject();
  assert_eq!("", post_2.get_content());

  let content_2 = post_2.get_content();
  println!("{}", content_2);
}
