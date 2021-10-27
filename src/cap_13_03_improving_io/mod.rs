use std::env;
use std::process;

mod improving_io;

use improving_io::Config;

pub fn run() {
  println!("•   cap_13_03_improving_io-------");
  let args: env::Args = env::args();

  let config = Config::new(args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);

    process::exit(1);
  });

  if let Err(e) = improving_io::run(config) {
    eprintln!("Application error: {}", e);

    process::exit(1);
  };
}
