// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
  //let out_dir = env::var("OUT_DIR").unwrap();

  Command::new("g++").args(&["src/SobolImpl.cpp", "-c", "-std=c++14", "-g", "-O2", "-fPIC"])
                      //.arg(&format!("{}/hello.o", out_dir))
                      .status().unwrap();
  Command::new("ar").args(&["crus", "../lib/libsobol.a", "src/SobolImpl.o"])
                    //.current_dir("./")
                    .status().unwrap();

  println!("cargo:rustc-link-search=native={}", "./lib");
  println!("cargo:rustc-link-lib=static=sobol");
}