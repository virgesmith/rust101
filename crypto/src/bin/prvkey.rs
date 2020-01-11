use crypto::key::Key;
use crypto::address;

use hex;
use base64;

use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {

  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    println!("usage: prvKey <pem-file>");
    std::process::exit(1);
  }

  let filename = &args[1];

  let prvkey = Key::from_pem_file(filename)?.private_key()?;

  println!("Private key data");
  println!("hex: {}", hex::encode(&prvkey));
  println!("base64: {}", base64::encode(&prvkey));
  println!("BTC WIF: {}", address::wif(&prvkey));
  println!("rust: [u8; 32] = {:?}", &prvkey);

  Ok(())
}
