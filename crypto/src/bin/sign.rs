use crypto::key::Key;
use crypto::hash::hash256;

use hex;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

  let args: Vec<String> = env::args().collect();

  if args.len() != 3 {
    println!("usage: sign <pem-key-file> <file>");
    std::process::exit(1);
  }

  let key = Key::from_pem_file(&args[1])?;

  let filename = &args[2];
  let mut file = File::open(filename)?;

  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;

  // NB not padding buffer to 4 bytes so will not always replicate CryptoJS javascript implementation
  println!("{} {} bytes", filename, buffer.len());

  let hash = hash256(&buffer);
  println!("hash: {}", hex::encode(&hash));

  let sig = key.sign(&hash)?;

  println!("sig: {}", hex::encode(&sig));

  let pubkey = key.to_pubkey()?;

  println!("check: {}", pubkey.verify(&hash, &sig)?);

  Ok(())
}
