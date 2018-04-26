// address module

use hash::{hash160, hash256};
use base58;

pub fn p2pkh(pkey: &[u8]) -> String {

  let mut data = hash160(&pkey);
  
  // prepend network byte
  data.insert(0, 0);
  let check = hash256(&data);
  // append checksum...
  let addr = [&data[..], &check[0..4]].concat();
  base58::from_bytes(addr)
}

// TODO wif