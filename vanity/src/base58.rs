use openssl::bn::{BigNum,BigNumContext};

pub const DIGITS_BTC: &'static str = 
  "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";


pub fn is_valid(digits: &str) -> bool {
  // TODO make static?
  let lookup = base58_lookup(DIGITS_BTC);

  for c in digits.chars() {
    let v = lookup[c as usize];
    // check for invalid chars
    if v == 255u8 {
      //return Err("invalid character in base58".to_string());
      return false;
    }
  }
  true
}

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
  let mut alookup: [u8;256] = [invalid; 256];

  for (i, c) in digits.chars().enumerate() {
    alookup[c as usize] = i as u8;
  }
  alookup
}

pub fn to_bytes(s: &str) -> Result<Vec<u8>, String> {

  // check for invalid chars
  if !is_valid(s) {
    return Err("invalid character in base58".to_string());
  }

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
  Ok(n.to_vec())
}

#[cfg(test)]
mod tests {
  use base58;
  #[test]
  fn test1() {

    let bytes : [u8; 32] = [0x94, 0x19, 0x9c, 0x35, 0xc8, 0x84, 0x8e, 0x03, 0xe9, 0xcb, 0x43, 0x80, 0xef, 0x71, 0x2b, 0xc0, 
                            0x77, 0xa5, 0x99, 0x1f, 0xa0, 0xbb, 0xf2, 0xc4, 0xa4, 0x0b, 0x03, 0x53, 0xe3, 0xad, 0x6c, 0x27];

    let encoded = base58::from_bytes(bytes.to_vec());
    assert_eq!(encoded, "Ay7zNBc5FhxKVaEUvcestTchSzbJtie96iwEUi5Hb32N");
    let decoded = base58::to_bytes(&encoded).unwrap();
    assert_eq!(decoded[0], bytes[0]);
    
  }

  #[test]
  fn test2() {
    let valid = "Va11d";
    let invalid = "Invalid";

    assert!(base58::is_valid(valid));
    assert!(!base58::is_valid(invalid));

    match base58::to_bytes(invalid) {
      Ok(_) => assert!(false, "error expected"),
      Err(_) => assert!(true),
    }
  }
  
}
