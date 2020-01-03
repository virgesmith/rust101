use crypto::key::Key;
use crypto::address;

use hex;
use base64;

use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {

  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    println!("usage: pubKey <pem-file>");
    std::process::exit(1);
  }

  let filename = &args[1];

  let pubkey = Key::from_pem_file(filename)?.public_key()?;

  println!("Compressed form public key data");
  println!("hex: {}", hex::encode(&pubkey));
  println!("base64: {}", base64::encode(&pubkey));
  println!("BTC p2pkh: {}", address::p2pkh(&pubkey));
  println!("rust: [u8; 33] = {:?}", &pubkey);

  Ok(())
}
