use crypto::hash::hash160;

use hex;
use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> Result<(), io::Error> {

  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    println!("usage: hash160 <file>");
    std::process::exit(1);
  }

  let filename = &args[1];

  let mut file = File::open(filename)?;

  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;

  // NB not padding buffer to 4 bytes so will not always replicate CryptoJS javascript implementation
  println!("{} {} bytes", filename, buffer.len());

  let h160 = hash160(&buffer);

  println!("{}", hex::encode(h160));

  Ok(())
}
