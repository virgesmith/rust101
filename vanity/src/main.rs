extern crate openssl;
extern crate time;

use std::env;
use openssl::ec::{EcKey,EcGroup,PointConversionForm};
use openssl::nid::Nid;
use openssl::bn::BigNumContext;

mod hash;
mod base58;
mod address;

fn main() {

  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  openssl::init();

  if args.len() < 2 {
    println!("usage: vanity <pattern>");
    return;
  } 

  let vanity = &args[1];
  println!("finding key for BTC address starting with {}...", vanity);

  let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap(); 
  let mut ctx = BigNumContext::new().unwrap();

  let mut i = 0;
  let start = time::get_time();
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
      let elapsed = time::get_time() - start;
      println!("Found {}\n{}/sec ", addr, i * 1000 / (1+elapsed.num_milliseconds()));
      println!("prv key = {:?}", key.private_key().to_vec());
      println!("wif = {}", address::wif(key.private_key().to_vec()));
      break;
    }
  }
  let a = "1GGZnReKybChriBrvxEDWsQqQJBLQHvRzW";
  //println!("{:?}", base58::to_bytes(a));
  println!("{} => \n{:?}", a, base58::from_bytes(base58::to_bytes(a)));
}


// zero PKH: 11111111111111111111111111114oLvT2
// addr PKH: 1GGZnReKybChriBrvxEDWsQqQJBLQHvRzW
// priv [0x00, 0x94, 0x19, 0x9c, 0x35, 0xc8, 0x84, 0x8e, 0x03, 0xe9, 0xcb, 0x43, 0x80, 0xef, 0x71, 0x2b, 0xc0, 0x77, 0xa5, 0x99, 0x1f, 0xa0, 0xbb, 0xf2, 0xc4, 0xa4, 0x0b, 0x03, 0x53, 0xe3, 0xad, 0x6c, 0x27]
// pub [0x04, 0xf6, 0x75, 0x5a, 0xfd, 0x57, 0xb6, 0xda, 0x43, 0xe8, 0xee, 0xc8, 0x14, 0x4b, 0x5e, 0xfe, 0x63, 0xf9, 0x02, 0xcc, 0xc1, 0x98, 0x04, 0x61, 0xfc, 0x66, 0x43, 0x56, 0x71, 0xf5, 0x4b, 0xea, 0x02, 0x14, 0x7c, 0x8f, 0x92, 0x4a, 0x1e, 0x7c, 0xbe, 0x66, 0xe6, 0xcd, 0xf0, 0x65, 0x32, 0x13, 0x63, 0x51, 0xd8, 0x86, 0x46, 0x80, 0x94, 0xa9, 0x3f, 0x89, 0xe9, 0x94, 0xfa, 0x8e, 0xbb, 0xd0, 0x80]
// pub [0x02, 0xf6, 0x75, 0x5a, 0xfd, 0x57, 0xb6, 0xda, 0x43, 0xe8, 0xee, 0xc8, 0x14, 0x4b, 0x5e, 0xfe, 0x63, 0xf9, 0x02, 0xcc, 0xc1, 0x98, 0x04, 0x61, 0xfc, 0x66, 0x43, 0x56, 0x71, 0xf5, 0x4b, 0xea, 0x02]
