
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::{PyException, PyValueError};

use crypto::hash;
use crypto::key::{Key, PubKey};
use crypto::address;
use crypto::vanity;
use crypto::CryptoResult;
use hex;
use base64;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;


#[pyfunction]
fn hash160(filename: String) -> PyResult<String> {
  hash_impl(filename, hash::hash160)
} 

#[pyfunction]
fn hash256(filename: String) -> PyResult<String> {
  hash_impl(filename, hash::hash256)
} 

// workaround for Error not converting to PyResult::Err
fn wrap_result<T>(res: CryptoResult<T>) -> PyResult<T> {
  match res {
    Ok(r) => Ok(r),
    // TODO to map errors to python exception types?
    Err(e) => Err(PyException::new_err(format!("{}", e))) 
  }
}

#[pyfunction]
fn pubkey(filename: String) -> PyResult<HashMap<String, String>> {
  wrap_result(pubkey_impl(filename))
}

#[pyfunction]
fn prvkey(filename: String) -> PyResult<HashMap<String, String>> {
  wrap_result(prvkey_impl(filename))
}

fn pubkey_impl(filename: String) -> CryptoResult<HashMap<String, String>> {

  let key = Key::from_pem_file(&filename)?.to_pubkey()?;

  let mut m = HashMap::new();
  m.insert("uncompressed hex".to_string(), hex::encode(&key.public_key()?));
  m.insert("uncompressed base64".to_string(), base64::encode(&key.public_key()?));
  m.insert("uncompressed raw".to_string(), format!("{:?}", &key.public_key()?));
  m.insert("compressed hex".to_string(), hex::encode(&key.compressed_public_key()?));
  m.insert("compressed base64".to_string(), base64::encode(&key.compressed_public_key()?));
  m.insert("compressed raw".to_string(), format!("{:?}", &key.compressed_public_key()?));
  m.insert("BTC p2pkh".to_string(), address::p2pkh(&key.compressed_public_key()?));

  Ok(m)
}

fn prvkey_impl(filename: String) -> CryptoResult<HashMap<String, String>> {

  let key = Key::from_pem_file(&filename)?;

  let mut m = HashMap::new();
  m.insert("hex".to_string(), hex::encode(&key.private_key()?));
  m.insert("base64".to_string(), base64::encode(&key.private_key()?));
  m.insert("raw".to_string(), format!("{:?}", &key.private_key()?));
  m.insert("BTC wif".to_string(), address::wif(&key.private_key()?));

  Ok(m)
}

fn hash_impl(filename: String, func: impl Fn(&[u8]) -> Vec<u8>) -> PyResult<String> {

  let mut file = File::open(filename)?;
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;

  Ok(hex::encode(func(&buffer)))
}

#[pyfunction]
fn sign(key_filename: String, msg_filename: String) -> PyResult<HashMap<String, String>> {
  wrap_result(sign_impl(key_filename, msg_filename))
}

fn sign_impl(key_filename: String, msg_filename: String) -> CryptoResult<HashMap<String, String>> {

  let key = Key::from_pem_file(&key_filename)?;

  let mut file = File::open(&msg_filename)?;
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;

  let hash = hash::hash256(&buffer);
  let sig = key.sign(&hash)?;

  let mut m = HashMap::new();
  m.insert("file".to_string(), msg_filename);
  m.insert("hash".to_string(), hex::encode(&hash));
  m.insert("signature".to_string(), hex::encode(&sig));

  Ok(m)
}

#[pyfunction]
fn verify(msg_filename: String, pubkey_hex: String, sig_hex: String) -> PyResult<bool> {
  wrap_result(verify_impl(msg_filename, pubkey_hex, sig_hex))
}

fn verify_impl(msg_filename: String, pubkey_hex: String, sig_hex: String) -> CryptoResult<bool> {
  let mut file = File::open(msg_filename)?;

  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;

  let hash = hash::hash256(&buffer);

  let pubkey = PubKey::from_bytes(&hex::decode(pubkey_hex)?)?;

  let sig = hex::decode(&sig_hex)?;

  Ok(pubkey.verify(&hash, &sig)?)
}

#[pyfunction]
fn vanity(s: String, nth: usize) -> PyResult<HashMap<String, String>> {
  // Use u8 to ensure threads <= 256, defaulting to 1
  if nth < 1 || nth > 256 {
    return Err(PyValueError::new_err(format!("invalid number of threads requested {} (must be 1-256)", nth)));
  }

  wrap_result(vanity_impl(s, nth))
}

fn vanity_impl(s: String, nth: usize) -> CryptoResult<HashMap<String, String>> {

  //println!("finding key for BTC P2PKH address starting with 1{} using {} threads...", vanity, threads);

  let start = std::time::SystemTime::now();
  let (k, tries) = vanity::search(s, nth)?;

  let elapsed = start.elapsed().unwrap().as_millis() as f64 / 1000.0;
  //println!("{} attempts in {} seconds", total_tries, elapsed);

  let mut m = HashMap::new();
  m.insert("hex".to_string(), hex::encode(&k.private_key()?)); 
  m.insert("p2pkh".to_string(), address::p2pkh(&k.compressed_public_key()?));
  m.insert("wif".to_string(), address::wif(&k.private_key()?));
  m.insert("tries".to_string(), tries.to_string());
  m.insert("time(s)".to_string(), elapsed.to_string());

  Ok(m)
}


#[pymodule]
fn pycrypto(_: Python, m: &PyModule) -> PyResult<()> {
  m.add_wrapped(wrap_pyfunction!(hash160))?;
  m.add_wrapped(wrap_pyfunction!(hash256))?;
  m.add_wrapped(wrap_pyfunction!(pubkey))?;
  m.add_wrapped(wrap_pyfunction!(prvkey))?;
  m.add_wrapped(wrap_pyfunction!(sign))?;
  m.add_wrapped(wrap_pyfunction!(verify))?;
  m.add_wrapped(wrap_pyfunction!(vanity))?;

  Ok(())
}

// tests use pytest

