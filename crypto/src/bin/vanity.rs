
use crypto::base58;
use crypto::address;
use crypto::key::Key;

//use rand::gen::*;

use std::env;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {

  let args: Vec<String> = env::args().collect();

  if args.len() < 2 || args.len() > 3 {
    println!("usage: vanity <pattern> <threads>");
    std::process::exit(1);
  } 

  // assumed P2PKH prefix 1 (for consistency with C++ impl)
  let vanity = &args[1];

  if !base58::is_valid(vanity) {
    println!("{} is not a valid base 58 number (using the BTC alphabet)", vanity);
    std::process::exit(1);
  }

  // be realistic: 58^8 > ~1e14
  if vanity.len() > 7 {
    println!("{} is too long to realistically find a matching address", vanity);
    std::process::exit(1);
  }

  // Use u8 to ensure threads <= 256, defaulting to 1
  let threads: usize = if args.len() == 3 { 
    match args[2].parse::<u8>() {
      Ok(0) => { println!("zero threads requested, actually using 1 thread"); 1 },
      Ok(n) => n,
      Err(e) => { println!("invalid threads arg: {}", e); std::process::exit(1); }
    }
  } else {
    1 
  } as usize;

  openssl::init();

  println!("finding key for BTC P2PKH address starting with 1{} using {} threads...", vanity, threads);

  let start = std::time::SystemTime::now();

  let pair = Arc::new((Mutex::new(false), Condvar::new()));

  // spawn threads here
  let mut handles = vec![];

  for _ in 0..threads {
    let p = pair.clone();
    let v = vanity.clone();
    handles.push( thread::spawn(move || { worker(v, p) }));
  }

  let mut total_tries = 0;
  for (i, e) in handles.into_iter().enumerate() {
    let result = e.join().unwrap();
    total_tries += result.1;
    match result.0 {
      Some(r) => { 
        println!("thread {} found key {}", i, hex::encode(&r.private_key()?)); 
        println!("ADDR: {}", address::p2pkh(&r.public_key()?));
        println!("WIF: {}", address::wif(&r.private_key()?));
      },
      // The thread didnt find the address 
      None => continue,
    }
  }
  let elapsed = start.elapsed().unwrap().as_millis() as f64 / 1000.0;
  println!("{} attempts in {} seconds", total_tries, elapsed);

  println!("Rate {}/thread/sec", total_tries as f64 / threads as f64 / elapsed);

  Ok(())
}

fn worker(vanity: String, pair: Arc<(Mutex<bool>, Condvar)>) -> (Option<Key>, usize) {

  let &(ref lock, ref cvar) = &*pair;

  let mut i = 0;
  let mut rng = pseudo::LCG::new(None);

  loop {

    // //let prv = rng.next_n(32/4).into_iter().fold(Vec::<u8>::new(), |acc, v32| acc.append(v32.to_be_bytes()) );
    // let prv32 = rng.next_n(32/4);
    // let mut prv8 = [0u8;32];
    // for i in 0..prv32.len() {
    //   let block = prv32[i].to_be_bytes();
    //   prv8[i*4] = block[0];
    //   prv8[i*4+1] = block[1];
    //   prv8[i*4+2] = block[2];
    //   prv8[i*4+3] = block[3];      
    //   //prv8.append(&mut val32.to_be_bytes().to_vec());
    // }
    // let key = Key::from_private_bytes(&prv8).unwrap();

    // this is no slower than using an external RNG to generate the private key
    let key = Key::new().unwrap();
    let bytes = key.public_key().unwrap();

    let addr = address::p2pkh(&bytes);
    let cmp = &addr[1..vanity.len()+1];
    i += 1;
    if vanity == cmp {
      *lock.lock().unwrap() = true;
      cvar.notify_all();
      return (Some(key), i);
    }
    if *lock.lock().unwrap() {
      return (None, i);
    }
  }
}
