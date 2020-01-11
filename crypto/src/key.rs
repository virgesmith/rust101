
use openssl::ec::*;
use openssl::ecdsa::*;
use openssl::pkey::{Private, Public};
use openssl::nid::Nid;
use openssl::bn::*;
use pem;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;


pub struct Key
{
  key_impl: EcKey<Private>
}

lazy_static!{
  static ref EC_GRP: EcGroup = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
}

impl Key {

  pub fn new() -> Result<Key, Box<dyn Error>> {
    Ok(Key{ key_impl: EcKey::generate(&EC_GRP)? }) 
  }

  pub fn from_pem_file(filename: &str) -> Result<Key, Box<dyn Error>> {

    let mut file = File::open(filename)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
  
    Ok(Key{ key_impl: EcKey::private_key_from_pem(&buffer)? })
  }

  pub fn from_private_bytes(bytes: &[u8;32]) -> Result<Key, Box<dyn Error>> {
    let ctx = BigNumContext::new()?;
    let prv = BigNum::from_slice(bytes)?;
    let mut pbl = EcPoint::new(&EC_GRP)?;
    //pbl.mul(&group, &EcPoint::new(&group).unwrap(), &prv, &ctx).unwrap();
    pbl.mul_generator(&EC_GRP, &prv, &ctx).unwrap();

    Ok(Key{ key_impl: EcKey::from_private_components(&EC_GRP, &prv, &pbl)? })
  }

  fn public_key_impl(&self, form: PointConversionForm) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut ctx = BigNumContext::new()?;
    Ok(self.key_impl.public_key().to_bytes(&EC_GRP, form, &mut ctx)?)
  }

  pub fn public_key(&self)  -> Result<Vec<u8>, Box<dyn Error>> {
    self.public_key_impl(PointConversionForm::UNCOMPRESSED)
  }

  pub fn compressed_public_key(&self)  -> Result<Vec<u8>, Box<dyn Error>> {
    self.public_key_impl(PointConversionForm::COMPRESSED)
  }

  pub fn private_key(&self) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(self.key_impl.private_key().to_vec())
  }

  pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(EcdsaSig::sign(data, &self.key_impl)?.to_der()?)
  }

  pub fn to_pubkey(&self) -> Result<PubKey, Box<dyn Error>> {
    Ok(PubKey::from_bytes(&self.public_key()?)?)
  }
}

pub struct PubKey {
  key_impl: EcKey<Public>
}

impl PubKey {

  pub fn from_pem_file(filename: &str) -> Result<PubKey, Box<dyn Error>> {

    let mut file = File::open(filename)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let pem = pem::parse(buffer)?;    
  
    PubKey::from_bytes(&pem.contents)
  }

  pub fn from_bytes(bytes: &[u8]) -> Result<PubKey, Box<dyn Error>> {
    let mut ctx = BigNumContext::new()?;
    let pbl = EcPoint::from_bytes(&EC_GRP, &bytes, &mut ctx)?;

    Ok(PubKey{ key_impl: EcKey::from_public_key(&EC_GRP, &pbl)? })
  }

  // pub fn from_signature(bytes: &[u8]) -> Result<PubKey, Box<dyn Error>> {

  // }

  fn public_key_impl(&self, form: PointConversionForm) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut ctx = BigNumContext::new()?;
    Ok(self.key_impl.public_key().to_bytes(&EC_GRP, form, &mut ctx)?)
  }

  pub fn public_key(&self) -> Result<Vec<u8>, Box<dyn Error>> {
    self.public_key_impl(PointConversionForm::UNCOMPRESSED)
  }

  pub fn compressed_public_key(&self) -> Result<Vec<u8>, Box<dyn Error>> {
    self.public_key_impl(PointConversionForm::COMPRESSED)
  }

  pub fn verify(&self, msg: &[u8], sig: &Vec<u8>) -> Result<bool, Box<dyn Error>>  {
    Ok(EcdsaSig::from_der(&sig)?.verify(&msg, &self.key_impl)?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::hash::hash256;

  #[test]
  fn test_construction() {
    let prvbytes: [u8; 32] = [0x94, 0x19, 0x9c, 0x35, 0xc8, 0x84, 0x8e, 0x03, 0xe9, 0xcb, 0x43, 0x80, 0xef, 0x71, 0x2b, 0xc0, 
                            0x77, 0xa5, 0x99, 0x1f, 0xa0, 0xbb, 0xf2, 0xc4, 0xa4, 0x0b, 0x03, 0x53, 0xe3, 0xad, 0x6c, 0x27];

    let prvkey = Key::from_private_bytes(&prvbytes).unwrap();

    assert_eq!(prvkey.private_key().unwrap(), prvbytes);

    let c_pubbytes: [u8; 33] = [2, 246, 117, 90, 253, 87, 182, 218, 67, 232, 238, 200, 20, 75, 94, 254, 99, 249, 2, 204, 193, 152, 
                              4, 97, 252, 102, 67, 86, 113, 245, 75, 234, 2];

    let pubkey = PubKey::from_bytes(&c_pubbytes).unwrap();
    assert_eq!(&pubkey.compressed_public_key().unwrap(), &c_pubbytes.to_vec());

    let u_pubbytes: [u8; 65] = [4, 246, 117, 90, 253, 87, 182, 218, 67, 232, 238, 200, 20, 75, 94, 254, 99, 249, 2, 204, 193, 152, 
                              4, 97, 252, 102, 67, 86, 113, 245, 75, 234, 2, 20, 124, 143, 146, 74, 30, 124, 190, 102, 230, 205, 
                              240, 101, 50, 19, 99, 81, 216, 134, 70, 128, 148, 169, 63, 137, 233, 148, 250, 142, 187, 208, 128];
    let pubkey = PubKey::from_bytes(&u_pubbytes).unwrap();
    assert_eq!(&pubkey.public_key().unwrap(), &u_pubbytes.to_vec());                 
  }
  #[test]
  fn test_consistency() {
    let bytes : [u8; 32] = [0x94, 0x19, 0x9c, 0x35, 0xc8, 0x84, 0x8e, 0x03, 0xe9, 0xcb, 0x43, 0x80, 0xef, 0x71, 0x2b, 0xc0, 
                            0x77, 0xa5, 0x99, 0x1f, 0xa0, 0xbb, 0xf2, 0xc4, 0xa4, 0x0b, 0x03, 0x53, 0xe3, 0xad, 0x6c, 0x27];

    let key = Key::from_private_bytes(&bytes).unwrap();
    let chex = "02f6755afd57b6da43e8eec8144b5efe63f902ccc1980461fc66435671f54bea02";
    let uhex = "04f6755afd57b6da43e8eec8144b5efe63f902ccc1980461fc66435671f54bea02147c8f924a1e7cbe66e6cdf06532136351d886468094a93f89e994fa8ebbd080";

    // directly get public key from private key
    let c_pubbytes = key.compressed_public_key().unwrap();
    assert_eq!(c_pubbytes.len(), 33 as usize);
    assert_eq!(hex::encode(c_pubbytes), chex);

    let u_pubbytes = key.public_key().unwrap();
    assert_eq!(u_pubbytes.len(), 65 as usize);
    assert_eq!(hex::encode(u_pubbytes), uhex);

    // construct a PubKey object and extract
    let pubkey = key.to_pubkey().unwrap();
    let c_pubbytes = pubkey.compressed_public_key().unwrap();
    assert_eq!(c_pubbytes.len(), 33 as usize);
    assert_eq!(hex::encode(c_pubbytes), chex);

    let u_pubbytes = pubkey.public_key().unwrap();
    assert_eq!(u_pubbytes.len(), 65 as usize);
    assert_eq!(hex::encode(u_pubbytes), uhex);
  }

  #[test]
  fn test_signing() {

    let bytes : [u8; 32] = [0x94, 0x19, 0x9c, 0x35, 0xc8, 0x84, 0x8e, 0x03, 0xe9, 0xcb, 0x43, 0x80, 0xef, 0x71, 0x2b, 0xc0, 
                            0x77, 0xa5, 0x99, 0x1f, 0xa0, 0xbb, 0xf2, 0xc4, 0xa4, 0x0b, 0x03, 0x53, 0xe3, 0xad, 0x6c, 0x27];

    let key = Key::from_private_bytes(&bytes).unwrap();

    let msg = hash256(b"Top secret plans");

    let sig = key.sign(&msg).unwrap();

    let pubkey = key.to_pubkey().unwrap();
    // should verify against public key and original digest
    assert_eq!(pubkey.verify(&msg, &sig).unwrap(), true);

    let false_msg = hash256(b"Decoy plans");

    // should NOT verify against public key and a different digest
    assert_eq!(pubkey.verify(&false_msg, &sig).unwrap(), false);

    let other_bytes : [u8; 32] = [0x95, 0x19, 0x9c, 0x35, 0xc8, 0x84, 0x8e, 0x03, 0xe9, 0xcb, 0x43, 0x80, 0xef, 0x71, 0x2b, 0xc0, 
                            0x77, 0xa5, 0x99, 0x1f, 0xa0, 0xbb, 0xf2, 0xc4, 0xa4, 0x0b, 0x03, 0x53, 0xe3, 0xad, 0x6c, 0x27];

    let other_pubkey = Key::from_private_bytes(&other_bytes).unwrap().to_pubkey().unwrap();
    other_pubkey.compressed_public_key().unwrap();

    // should NOT verify against different public key and original digest
    assert_eq!(other_pubkey.verify(&msg, &sig).unwrap(), false);

  }

}
