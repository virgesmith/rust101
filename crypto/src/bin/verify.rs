use crypto::key::PubKey;
use crypto::hash::hash256;

//use core::ops::Add;

use hex;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
//use std::borrow::Borrow;


fn main() -> Result<(), Box<dyn Error>> {

  let args: Vec<String> = env::args().collect();

  if args.len() != 4 {
    println!("usage: verify <file> <pubkey> <sig>");
    std::process::exit(1);
  }

  let filename = &args[1];
  let mut file = File::open(filename)?;

  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;

  // NB not padding buffer to 4 bytes so will not always replicate CryptoJS javascript implementation
  println!("{} {} bytes", filename, buffer.len());

  let hash = hash256(&buffer);
  println!("hash: {}", hex::encode(&hash));

  //let pubkey = PubKey::from_pem_file(&args[1])?;
  let pubkey = PubKey::from_bytes(&hex::decode(&args[2])?)?;

  //  let sig = key.sign(&hash)?;
  let sig = hex::decode(&args[3])?;

//  println!("sig: {}", hex::encode(&sig));
//  let pubkey = key.to_pubkey()?;

  println!("check: {}", pubkey.verify(&hash, &sig)?);

  Ok(())
}
