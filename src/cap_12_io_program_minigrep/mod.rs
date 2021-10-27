use std::env;
use std::process;

mod minigrep;

use minigrep::Config;

pub fn run() {
  println!("\n•   cap_12_minigrep");

  let args: Vec<_> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);

    process::exit(1);
  });

  if let Err(e) = minigrep::run(config) {
    eprintln!("Application error: {}", e);

    process::exit(1);
  };
}
