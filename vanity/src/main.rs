extern crate openssl;
extern crate time;

use std::env;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use openssl::ec::{EcKey,EcGroup,PointConversionForm};
use openssl::pkey::Private;
use openssl::nid::Nid;
use openssl::bn::BigNumContext;

mod hash;
mod base58;
mod address;

// fn f(j: usize, pair: Arc<(Mutex<bool>, Condvar)>) -> usize {
//   println!("started thread {}", j);
//   let &(ref lock, ref cvar) = &*pair;
//   if j == 1 {
//     *lock.lock().unwrap() = true;
//     cvar.notify_all();
//   }
//   loop {
//     thread::sleep(std::time::Duration::from_millis(1000 as u64));
//     if *lock.lock().unwrap() {
//       break;
//     }
//   }
//   println!("stopped thread {}", j);
//   j+1
// }

//type AddressResult = (usize, Option<EcKey<Private>>);

fn main() {

  let args: Vec<String> = env::args().collect();
  //println!("{:?}", args);

  openssl::init();

  if args.len() < 3 {
    println!("usage: vanity <pattern> <threads>");
    return;
  } 

  let vanity = &args[1];

  let threads = match args[2].parse::<usize>() {
    Ok(0) => { println!("zero threads requested, actually using 1 thread"); 1 },
    Ok(n) => n,
    Err(e) => { println!("invalid threads arg: {}", e); return; }
  };

  println!("finding key for BTC address starting with {} using {} threads...", vanity, threads);

  let start = time::get_time();

  let pair = Arc::new((Mutex::new(false), Condvar::new()));

  // spawn threads here
  let mut handles = vec![];
  for _ in 0..threads {
    let p = pair.clone();
    let v = vanity.clone();
    handles.push( thread::spawn(move || { worker(v, p) }));
  }

  let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap(); 
  let mut ctx = BigNumContext::new().unwrap();

  let mut total_tries = 0;
  for (i, e) in handles.into_iter().enumerate() {
    let result = e.join().unwrap();
    total_tries += result.1;
    match result.0 {
      Some(r) => { 
        println!("thread {} found ADDR: {}", i, address::p2pkh(&r.public_key().to_bytes(&group, 
          PointConversionForm::COMPRESSED, &mut ctx).unwrap()));
        println!("WIF: {}", address::wif(r.private_key().to_vec()));
      },
      // The thread didnt find the address 
      None    => continue,
    }
  }
  println!("{:?}", total_tries);

  let elapsed = (time::get_time() - start).num_milliseconds() as f64 / 1000.0;
  println!("Rate {}/thread/sec", total_tries as f64 / threads as f64 / elapsed);
}

fn worker(vanity: String, pair: Arc<(Mutex<bool>, Condvar)>) -> (Option<EcKey<Private>>, usize) {

  // TODO pass in...
  let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap(); 
  let mut ctx = BigNumContext::new().unwrap();
  let &(ref lock, ref cvar) = &*pair;

  let mut i = 0;

  loop {

    let key = EcKey::generate(&group).unwrap();
    assert!(key.check_key().unwrap() == ()); // returns Result<(), ErrorStack>
    let bytes = key.public_key().to_bytes(&group, PointConversionForm::COMPRESSED, &mut ctx).unwrap();

    let addr = address::p2pkh(&bytes);
    let cmp = &addr[..vanity.len()];
    i += 1;
    if vanity == cmp {
      *lock.lock().unwrap() = true;
      cvar.notify_all();
      return (Some(key.clone()), i);
    }
    if *lock.lock().unwrap() {
      return (None, i);
    }
  }
}


