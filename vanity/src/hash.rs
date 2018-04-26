
extern crate openssl;
use openssl::sha;
use openssl::hash::{hash, MessageDigest};

// returns Vec so can be prepended
pub fn hash160(data: &[u8]) -> Vec<u8> {
  let data1 = sha::sha256(data);
  let res = hash(MessageDigest::ripemd160(), &data1).unwrap().to_vec(); //to_vec().clone_into_array()
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

