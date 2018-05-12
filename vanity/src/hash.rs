
extern crate openssl;
use openssl::sha;
use openssl::hash::{hash, MessageDigest};

// returns Vec so can be prepended
pub fn hash160(data: &[u8]) -> Vec<u8> {
  hash(MessageDigest::ripemd160(), &sha::sha256(data)).unwrap().to_vec()
}

pub fn hash256(data: &[u8]) -> [u8; 32] {
  sha::sha256(&sha::sha256(&data[..]))
}

