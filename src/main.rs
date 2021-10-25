// # Rust Book exercises

#![allow(unused_variables, dead_code, unused_assignments)]

mod cap_07_restaurant;
mod cap_12_minigrep;
mod cap_13_01_closures;
mod cap_13_02_iterators;
mod cap_13_03_improving_io;

use std::env;

enum Module {
  NoModuleArg,
  Minigrep,
}

fn get_module(args: &Vec<String>) -> Module {
  if args.len() == 1 {
    return Module::NoModuleArg;
  }

  match args[1].as_str() {
    "minigrep" => Module::Minigrep,
    _ => Module::NoModuleArg,
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let module_arg = get_module(&args);

  match module_arg {
    Module::Minigrep => cap_12_minigrep::run(),
    Module::NoModuleArg => {
      cap_07_restaurant::run();
      cap_13_01_closures::run();
      cap_13_02_iterators::iterators::run();
      cap_13_03_improving_io::run();
    }
  }
}
