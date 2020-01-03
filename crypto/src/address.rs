// address module
use crate::hash::{hash160, hash256};
use crate::base58;

pub fn p2pkh(pubkey: &[u8]) -> String {

  let mut data = hash160(&pubkey);
  
  // prepend network byte
  data.insert(0, 0);
  let check = hash256(&data);
  // append checksum...
  let addr = [&data, &check[0..4]].concat();
  base58::from_bytes(addr)
}

pub fn wif(prvkey: &Vec<u8>) -> String {

  let mut data = prvkey.clone();
  // prepend network byte (wif)
  data.insert(0, 128);
  // append 1 (not sure why)
  data.push(1);
  // append checksum
  let check = hash256(&data);
  let wif = [&data, &check[0..4]].concat();
  base58::from_bytes(wif)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test1() {
    // addr PKH: 1GGZnReKybChriBrvxEDWsQqQJBLQHvRzW

    let prvkey : [u8; 32] = [/*0x00,*/ 0x94, 0x19, 0x9c, 0x35, 0xc8, 0x84, 0x8e, 0x03, 0xe9, 0xcb, 0x43, 0x80, 0xef, 0x71, 0x2b, 0xc0, 
                                       0x77, 0xa5, 0x99, 0x1f, 0xa0, 0xbb, 0xf2, 0xc4, 0xa4, 0x0b, 0x03, 0x53, 0xe3, 0xad, 0x6c, 0x27];
    let pubkey : [u8; 33] = [0x02, 0xf6, 0x75, 0x5a, 0xfd, 0x57, 0xb6, 0xda, 0x43, 0xe8, 0xee, 0xc8, 0x14, 0x4b, 0x5e, 0xfe, 0x63, 
                                   0xf9, 0x02, 0xcc, 0xc1, 0x98, 0x04, 0x61, 0xfc, 0x66, 0x43, 0x56, 0x71, 0xf5, 0x4b, 0xea, 0x02];

    let a = p2pkh(&pubkey);
    assert_eq!(a, "1GGZnReKybChriBrvxEDWsQqQJBLQHvRzW");
    let w = wif(prvkey.to_vec());
    assert_eq!(w, "L2Bbdwmcs188qfBWjhGi95P6sxVeGbvS1zQsnvpcAc4h1864jJXD");    
  }
}
