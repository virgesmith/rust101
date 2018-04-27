// address module

use hash::{hash160, hash256};
use base58;

pub fn p2pkh(pubkey: &[u8]) -> String {

  let mut data = hash160(&pubkey);
  
  // prepend network byte
  data.insert(0, 0);
  let check = hash256(&data);
  // append checksum...
  let addr = [&data[..], &check[0..4]].concat();
  base58::from_bytes(addr)
}

pub fn wif(prvkey: Vec<u8>) -> String {

  let mut data = prvkey;
  // prepend network byte (wif)
  data.insert(0, 128);
  // append 1 (not sure why)
  data.push(1);
  // append checksum
  let check = hash256(&data);
  let wif = [&data[..], &check[0..4]].concat();
  base58::from_bytes(wif)
}
