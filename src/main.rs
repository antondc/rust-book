// # Rust Book exercises

#![allow(unused_variables, dead_code, unused_assignments)]

mod cap_07_restaurant;
mod cap_12_minigrep;
mod cap_13_01_closures;
mod cap_13_02_iterators;
mod cap_13_03_improving_io;
mod cap_14_01_cargo_crates;
mod cap_15_01_smart_pointers_box;
mod cap_15_02_smart_pointers_deref;
mod cap_15_03_smart_pointers_drop;
mod cap_15_04_smart_pointers_reference_counted;

use std::env;

enum Module {
  NoModuleArg,
  Minigrep,
  Minigrep2,
}

fn get_module(args: &Vec<String>) -> Module {
  if args.len() == 1 {
    return Module::NoModuleArg;
  }

  match args[1].as_str() {
    "minigrep" => Module::Minigrep,
    "minigrep2" => Module::Minigrep2,
    _ => Module::NoModuleArg,
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let module_arg = get_module(&args);

  match module_arg {
    Module::Minigrep2 => cap_12_minigrep::run(),
    Module::Minigrep => cap_13_03_improving_io::run(),
    Module::NoModuleArg => {
      cap_07_restaurant::run();
      cap_13_01_closures::run();
      cap_13_02_iterators::run();
      cap_14_01_cargo_crates::run();
      cap_15_01_smart_pointers_box::run();
      cap_15_02_smart_pointers_deref::run();
      cap_15_03_smart_pointers_drop::run();
      cap_15_04_smart_pointers_reference_counted::run();
    }
  }
}
