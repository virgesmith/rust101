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
  println!("{:?}", args);

  openssl::init();

  if args.len() < 3 {
    println!("usage: vanity <pattern> <threads>");
    return;
  } 

  let vanity = &args[1];
  let threads = args[2].parse::<usize>().unwrap();
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
    //println!("pubkey {:?}", bytes);
    // let bytes = [0x02, 0xf6, 0x75, 0x5a, 0xfd, 0x57, 0xb6, 0xda, 0x43, 0xe8, 0xee, 0xc8, 0x14, 
    //   0x4b, 0x5e, 0xfe, 0x63, 0xf9, 0x02, 0xcc, 0xc1, 0x98, 0x04, 0x61, 0xfc, 0x66, 0x43, 0x56, 
    //   0x71, 0xf5, 0x4b, 0xea, 0x02];

    let addr = address::p2pkh(&bytes);
    let cmp = &addr[..vanity.len()];
    //println!("11111111111111111111111111114oLvT2\n{}", a);
    //println!("{}", a);
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


// zero PKH: 11111111111111111111111111114oLvT2
// addr PKH: 1GGZnReKybChriBrvxEDWsQqQJBLQHvRzW
// priv [0x00, 0x94, 0x19, 0x9c, 0x35, 0xc8, 0x84, 0x8e, 0x03, 0xe9, 0xcb, 0x43, 0x80, 0xef, 0x71, 0x2b, 0xc0, 0x77, 0xa5, 0x99, 0x1f, 0xa0, 0xbb, 0xf2, 0xc4, 0xa4, 0x0b, 0x03, 0x53, 0xe3, 0xad, 0x6c, 0x27]
// pub [0x04, 0xf6, 0x75, 0x5a, 0xfd, 0x57, 0xb6, 0xda, 0x43, 0xe8, 0xee, 0xc8, 0x14, 0x4b, 0x5e, 0xfe, 0x63, 0xf9, 0x02, 0xcc, 0xc1, 0x98, 0x04, 0x61, 0xfc, 0x66, 0x43, 0x56, 0x71, 0xf5, 0x4b, 0xea, 0x02, 0x14, 0x7c, 0x8f, 0x92, 0x4a, 0x1e, 0x7c, 0xbe, 0x66, 0xe6, 0xcd, 0xf0, 0x65, 0x32, 0x13, 0x63, 0x51, 0xd8, 0x86, 0x46, 0x80, 0x94, 0xa9, 0x3f, 0x89, 0xe9, 0x94, 0xfa, 0x8e, 0xbb, 0xd0, 0x80]
// pub [0x02, 0xf6, 0x75, 0x5a, 0xfd, 0x57, 0xb6, 0xda, 0x43, 0xe8, 0xee, 0xc8, 0x14, 0x4b, 0x5e, 0xfe, 0x63, 0xf9, 0x02, 0xcc, 0xc1, 0x98, 0x04, 0x61, 0xfc, 0x66, 0x43, 0x56, 0x71, 0xf5, 0x4b, 0xea, 0x02]
