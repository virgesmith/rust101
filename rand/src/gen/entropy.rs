use crate::gen::*;
use std::fs::File;
use std::io::BufReader;
extern crate byteorder;

pub struct EntropySource 
{ 
  buf: BufReader<File>
}

impl EntropySource {
  pub fn new() -> EntropySource {
    EntropySource{ buf: BufReader::new(File::open("/dev/urandom").unwrap()) }
  }
}

/// General traits of random 
impl RandomStream for EntropySource {
  fn next_n(&mut self, n: usize) -> Vec<u32> {
    use byteorder::{ReadBytesExt, NativeEndian};
    let mut res: Vec<u32> = Vec::with_capacity(n);
    unsafe { res.set_len(n); }
    self.buf.read_u32_into::<NativeEndian>(&mut res).unwrap();
    res
  }
  fn uniforms01(&mut self, n: usize) -> Vec<f64> {
    self.next_n(n).iter().map(|&r| r as f64 / (2.0f64.powi(32))).collect()
  }
}

impl Rejectable for EntropySource { } 

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_basic() {
    let n: usize = 10_000;
    let mut rng = EntropySource::new();
    assert!((rng.uniforms01(n).iter().sum::<f64>() - (n as f64/2.0)).abs() < (n as f64).sqrt());
  }
}
