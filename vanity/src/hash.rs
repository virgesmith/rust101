
extern crate openssl;
use openssl::sha;
use openssl::hash::{hash, MessageDigest};

// returns Vec so can be prepended
pub fn hash160(data: &[u8]) -> Vec<u8> {
  let res = hash(MessageDigest::ripemd160(), &sha::sha256(data)).unwrap().to_vec(); 
  // let mut a = [0; 20];
  // for i in 0..a.len() {
  //     // Panics if not enough input
  //     a[i] = res[i];
  // }
  // a
  res
}

pub fn hash256(data: &[u8]) -> [u8; 32] {
  sha::sha256(&sha::sha256(&data[..]))
}

