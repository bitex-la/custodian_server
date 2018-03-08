extern crate regex;
use std::fs;
use regex::Regex;

fn main(){
  let libs = "./vendor/bitprim_btc";
  println!(r"cargo:rustc-link-search={}", libs);
  println!(r"cargo:rustc-link-lib=static=stdc++");

  let re = Regex::new(r"lib([0-9A-Za-z_-]+)\.a").unwrap();
  let paths = fs::read_dir(libs).expect("Path not found");

  for entry in paths {
    let path = entry.unwrap().path();
    let filename = path.to_str().unwrap();
    let captures = re.captures(filename).expect("A non-library found");
    println!(r"cargo:rustc-link-lib=static={}", &captures[1]);
  }
}
