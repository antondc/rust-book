pub mod iterators;

pub fn run() {
  println!("\nā¢āā cap_13_02_iterators");
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();

  for val in v1_iter {
    println!("Got: {}", val);
  }

  let v1 = vec![1, 2, 3];
  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

  assert_eq!(v2, vec![2, 3, 4]);
}
