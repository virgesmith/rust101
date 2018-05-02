use openssl::bn::{BigNum,BigNumContext};

pub static DIGITS_BTC: &'static str = 
  "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";


pub fn from_bytes(b: Vec<u8>) -> String {
  if b.len() == 0 {
    () //String::from("")
  }
  let mut ctx = BigNumContext::new().unwrap();
  let base = BigNum::from_u32(58).unwrap(); 
  let zero = BigNum::from_u32(0).unwrap(); 
  let mut r = BigNum::new().unwrap();
  let mut x = BigNum::from_slice(&b).unwrap();

  let addr_len = b.len() * 138 / 100;

  let mut buf : Vec<char> = Vec::with_capacity(64);
  let mut i = 0;
  while x != zero {
    let mut tmpx = BigNum::new().unwrap();
    let _ = tmpx.div_rem(&mut r, &x, &base, &mut ctx);
    x = tmpx;
    let j = r.mod_word(58).unwrap() as usize;
    buf.push(DIGITS_BTC.as_bytes()[j] as char);
    i += 1;
  }
  // add leading zeros
  while i < addr_len {
    buf.push(DIGITS_BTC.as_bytes()[0] as char);
    i += 1;
  }
  let str: String = buf.into_iter().rev().collect();   
  str
}

fn base58_lookup(digits: &str) -> [u8; 256] {
  let invalid: u8 = 255;
  let mut lookup: [u8;256] = [invalid; 256];

  for (i, c) in digits.chars().enumerate() {
    lookup[c as usize] = i as u8;
  }
  lookup
}


pub fn to_bytes(s: &str) -> Vec<u8> {
  let base = 58; 
  let mut n = BigNum::from_u32(0).unwrap(); 

  // TODO make static?
  let lookup = base58_lookup(DIGITS_BTC);
  // loop over str
  for c in s.chars() {
    let v = lookup[c as usize];
    let _ = n.mul_word(base);
    let _ = n.add_word(v as u32); 
    //println!("{}", n/*, s[i]*/);
  }
  n.to_vec()
}
