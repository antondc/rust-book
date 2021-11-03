// # Rust Book exercises

#![allow(unused_variables, dead_code, unused_assignments)]

mod cap_07_packages_crates_modules;
mod cap_12_io_program_minigrep;
mod cap_13_01_closures;
mod cap_13_02_iterators;
mod cap_13_03_improving_io;
mod cap_14_01_cargo_crates;
mod cap_15_01_smart_pointers_box;
mod cap_15_02_smart_pointers_deref;
mod cap_15_03_smart_pointers_drop;
mod cap_15_04_smart_pointers_reference_counted;
mod cap_15_05_smart_pointers_refcell;
mod cap_15_06_smart_pointers_leaks;
mod cap_16_01_concurrency_threads_simultaneously;
mod cap_16_02_concurrency_messages;
mod cap_16_03_concurrency_shared_state;
mod cap_17_01_oop_encapsulation;
mod cap_17_02_oop_traits_and_types;
mod cap_17_03_01_oop_state_pattern;
mod cap_17_03_02_oop_states_as_types;
mod cap_18_01_patterns_places;
mod cap_18_02_patterns_refutability;
mod cap_18_03_pattern_syntax;
mod cap_19_01_unsafe_rust;
mod cap_19_02_advanced_traits;
mod cap_19_03_advanced_types;
mod cap_19_04_advanced_functions;
mod cap_19_05_macros;
mod cap_20_01_web_server_single_threaded;

use std::env;

enum Module {
  Minigrep,
  Minigrep2,
  WebServerSingle,
  NoModuleArg,
}

fn get_module(args: &Vec<String>) -> Module {
  if args.len() == 1 {
    return Module::NoModuleArg;
  }

  match args[1].as_str() {
    "minigrep" => Module::Minigrep,
    "minigrep2" => Module::Minigrep2,
    "web_server_single" => Module::WebServerSingle,
    _ => Module::NoModuleArg,
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let module_arg = get_module(&args);

  match module_arg {
    Module::Minigrep2 => cap_12_io_program_minigrep::run(),
    Module::Minigrep => cap_13_03_improving_io::run(),
    Module::WebServerSingle => cap_20_01_web_server_single_threaded::run(),
    Module::NoModuleArg => {
      cap_07_packages_crates_modules::run();
      cap_13_01_closures::run();
      cap_13_02_iterators::run();
      cap_14_01_cargo_crates::run();
      cap_15_01_smart_pointers_box::run();
      cap_15_02_smart_pointers_deref::run();
      cap_15_03_smart_pointers_drop::run();
      cap_15_04_smart_pointers_reference_counted::run();
      cap_15_05_smart_pointers_refcell::run();
      cap_15_06_smart_pointers_leaks::run();
      cap_16_01_concurrency_threads_simultaneously::run();
      cap_16_02_concurrency_messages::run();
      cap_16_03_concurrency_shared_state::run();
      cap_17_01_oop_encapsulation::run();
      cap_17_02_oop_traits_and_types::run();
      cap_17_03_01_oop_state_pattern::run();
      cap_17_03_02_oop_states_as_types::run();
      cap_18_01_patterns_places::run();
      cap_18_02_patterns_refutability::run();
      cap_18_03_pattern_syntax::run();
      cap_19_01_unsafe_rust::run();
      cap_19_02_advanced_traits::run();
      cap_19_03_advanced_types::run();
      cap_19_04_advanced_functions::run();
      cap_19_05_macros::run();
    }
  }
}
