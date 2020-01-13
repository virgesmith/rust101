
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crypto::hash;
use hex;

use std::fs::File;
use std::io::Read;


#[pyfunction]
fn hash160(filename: String) -> PyResult<String> {
  hash_impl(filename, hash::hash160)
} 

#[pyfunction]
fn hash256(filename: String) -> PyResult<String> {
  hash_impl(filename, hash::hash256)
} 

fn hash_impl(filename: String, func: impl Fn(&[u8]) -> Vec<u8>) -> PyResult<String> {

  let mut file = File::open(filename)?;
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;

  Ok(hex::encode(func(&buffer)))
}

#[pymodule]
fn pycrypto(_: Python, m: &PyModule) -> PyResult<()> {
  m.add_wrapped(wrap_pyfunction!(hash160))?;
  m.add_wrapped(wrap_pyfunction!(hash256))?;
  Ok(())
}


// #[cfg(test)]
// mod tests {
//   #[test]
//   fn it_works() {
//     assert_eq!(2 + 2, 4);
//   }
// }
