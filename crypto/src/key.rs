
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

  pub fn public_key(&self) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut ctx = BigNumContext::new()?;
    Ok(self.key_impl.public_key().to_bytes(&EC_GRP, PointConversionForm::COMPRESSED, &mut ctx)?)
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

  pub fn public_key(&self) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut ctx = BigNumContext::new()?;
    Ok(self.key_impl.public_key().to_bytes(&EC_GRP, PointConversionForm::COMPRESSED, &mut ctx)?)
  }

  pub fn verify(&self, msg: &[u8], sig: &Vec<u8>) -> Result<bool, Box<dyn Error>>  {
    Ok(EcdsaSig::from_der(&sig)?.verify(&msg, &self.key_impl)?)
  }
}


// extract key