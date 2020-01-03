
use openssl::{sha};
use openssl::hash::{hash, MessageDigest};

// returns Vec so can be prepended
pub fn hash160(data: &[u8]) -> Vec<u8> {
  hash(MessageDigest::ripemd160(), &sha::sha256(data)).unwrap().to_vec()
}

pub fn hash256(data: &[u8]) -> Vec<u8> {
  sha::sha256(&sha::sha256(&data[..])).to_vec()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test1() {
    let bytes : [u8; 32] = [0x94, 0x19, 0x9c, 0x35, 0xc8, 0x84, 0x8e, 0x03, 0xe9, 0xcb, 0x43, 0x80, 0xef, 0x71, 0x2b, 0xc0, 
                            0x77, 0xa5, 0x99, 0x1f, 0xa0, 0xbb, 0xf2, 0xc4, 0xa4, 0x0b, 0x03, 0x53, 0xe3, 0xad, 0x6c, 0x27];

    let h256 = hash256(&bytes);
    assert_eq!(h256.len(), 32);
    let h160 = hash160(&bytes);
    assert_eq!(h160.len(), 20);
  }  
}
