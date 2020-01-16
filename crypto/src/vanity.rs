
use crate::key::Key;
use crate::address;
use crate::base58;

use rand::gen::*;

use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
enum VanityError {
  InvalidDigits(String),
  TooLong(String)
}

impl fmt::Display for VanityError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      VanityError::InvalidDigits(ref s) => write!(f, "invalid search string: {}", s),
      VanityError::TooLong(ref s) => write!(f, "search string is too long: {}", s),
    }
  }
}

impl Error for VanityError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

pub fn search(pattern: String, threads: usize) -> Result<(Key, usize), Box<dyn Error>>  {

  if !base58::is_valid(&pattern) {
    return Err(Box::new(VanityError::InvalidDigits(pattern)));
  }

  // be realistic: 58^8 > ~1e14
  if pattern.len() > 7 {
    return Err(Box::new(VanityError::TooLong(pattern)));
  }
  
  openssl::init();

  let pair = Arc::new((Mutex::new(false), Condvar::new()));

  let mut handles = vec![];

  for _ in 0..threads {
    let p = pair.clone();
    let v = pattern.clone();
    handles.push( thread::spawn(move || { worker(v, p) }));
  }

  let mut k = Key::new()?; 
  let mut total_tries = 0;
  for (_, e) in handles.into_iter().enumerate() {
    let result = e.join().unwrap();
    total_tries += result.1;
    match result.0 {
      Some(r) => k = r,
      None => continue
    }
  }
  Ok((k, total_tries))
}

fn worker(vanity: String, pair: Arc<(Mutex<bool>, Condvar)>) -> (Option<Key>, usize) {

  let &(ref lock, ref cvar) = &*pair;

  let mut i = 0;
  let mut rng = pseudo::LCG::new(None);

  loop {

    //let prv = rng.next_n(32/4).into_iter().fold(Vec::<u8>::new(), |acc, v32| acc.append(v32.to_be_bytes()) );
    let prv32 = rng.next_n(32/4);
    let mut prv8 = [0u8;32];
    for i in 0..prv32.len() {
      let block = prv32[i].to_be_bytes();
      prv8[i*4] = block[0];
      prv8[i*4+1] = block[1];
      prv8[i*4+2] = block[2];
      prv8[i*4+3] = block[3];      
      //prv8.append(&mut val32.to_be_bytes().to_vec());
    }
    let key = Key::from_private_bytes(&prv8).unwrap();

    // this is no slower than using an external RNG to generate the private key
    //let key = Key::new().unwrap();

    let bytes = key.compressed_public_key().unwrap();

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

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test1() {
    let (k, _) = search("A".to_string(), 1).unwrap();
    let a = address::p2pkh(&k.compressed_public_key().unwrap());
    assert_eq!(&a[..2], "1A");
    let (k, _) = search("BB".to_string(), 4).unwrap();
    let a = address::p2pkh(&k.compressed_public_key().unwrap());
    assert_eq!(&a[..3], "1BB");

  }

  #[test]
  fn test_failures() {
    // invalid
    match search("0".to_string(), 1) {
      Ok(_) => assert!(false, "invalid base 58 digit should fail"),
      Err(e) => assert_eq!(e.to_string(), "invalid search string: 0")
    }
    match search("AAAAAAAAAAAA".to_string(), 1) {
      Ok(_) => assert!(false, "invalid base 58 digit should fail"),
      Err(e) => assert_eq!(e.to_string(), "search string is too long: AAAAAAAAAAAA")
    }
  }
}
