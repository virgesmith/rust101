// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
  Command::new("g++").args(&["../src/SobolImpl.cpp", "-c", "-std=c++14", "-g", "-O2", "-fPIC"])
                     .current_dir("./lib").status().unwrap();
  Command::new("ar").args(&["crus", "./libsobol.a", "./SobolImpl.o"])
                    .current_dir("./lib").status().unwrap();
  Command::new("g++").args(&["../src/MT19937.cpp", "-c", "-std=c++14", "-g", "-O2", "-fPIC"])
                     .current_dir("./lib").status().unwrap();
  Command::new("ar").args(&["crus", "./libmt19937.a", "./MT19937.o"])
                    .current_dir("./lib").status().unwrap();

  println!("cargo:rustc-link-search=native={}", "./lib");
  println!("cargo:rustc-link-lib=static=mt19937");
  println!("cargo:rustc-link-lib=static=sobol");
  // needed for operator new & delete in MT19937.cpp
  println!("cargo:rustc-link-lib=stdc++");
}