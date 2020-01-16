
use crypto::base58;
use crypto::address;
use crypto::vanity::search;

use std::env;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {

  let args: Vec<String> = env::args().collect();

  if args.len() < 2 || args.len() > 3 {
    println!("usage: vanity <pattern> <threads>");
    std::process::exit(1);
  } 

  // assumed P2PKH prefix 1 (for consistency with C++ impl)
  let search_word = &args[1];

  // Use u8 to ensure threads <= 256, defaulting to 1
  let threads = if args.len() == 3 { 
    match args[2].parse::<u8>() {
      Ok(0) => { println!("zero threads requested, actually using 1 thread"); 1 },
      Ok(n) => n,
      Err(e) => { println!("invalid threads arg: {}", e); std::process::exit(1); }
    }
  } else {
    1 
  } as usize;

  println!("finding key for BTC P2PKH address starting with 1{} using {} threads...", search_word, threads);

  let start = std::time::SystemTime::now();

  // TODO sort out lifetime of search_word
  let (k, n) = search(search_word.to_string(), threads)?;
  println!("Found key {}", hex::encode(&k.private_key()?)); 
  println!("ADDR: {}", address::p2pkh(&k.compressed_public_key()?));
  println!("WIF: {}", address::wif(&k.private_key()?));

  let elapsed = start.elapsed().unwrap().as_millis() as f64 / 1000.0;
  println!("{} attempts in {} seconds", n, elapsed);

  println!("Rate {}/thread/sec", n as f64 / threads as f64 / elapsed);

  Ok(())
}

