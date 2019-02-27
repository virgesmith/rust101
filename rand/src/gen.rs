//! gen: Generator module

// mod rand is implicit from project name in Cargo.toml
// mod gen  is implicit from this filename

//! Generator interface
//! Initialise using clock as seed
//! 
pub trait Gen {
  fn new() -> Self;
  fn seed(s:u32) -> Self;
  fn next(&mut self) -> u32;
  fn next01(&mut self) -> f64;
  fn reset(&mut self);
}

pub struct LCG {
  s: u32,
  r: u32
}

pub struct Xorshift64 {
  s: u64,
  r: u64
}

use std::time::{SystemTime, UNIX_EPOCH};

// private 
impl LCG {
  const A: u64 = 48271;
  const M: u64 = std::i32::MAX as u64;
}

// public
impl Gen for LCG {
  fn new() -> LCG {
    LCG::seed(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos())   
  }

  fn seed(seed: u32) -> LCG {
    assert_ne!(seed, 0);
    LCG{s: seed, r: seed}
  }

  fn next(&mut self) -> u32 {
    self.r = ((self.r as u64 * LCG::A) % LCG::M) as u32;
    self.r
  }

  fn next01(&mut self) -> f64 {
    self.next() as f64 / LCG::M as f64
  }

  fn reset(&mut self) {
    self.r = self.s;  
  }
}

impl Xorshift64 { }

impl Gen for Xorshift64 {
  fn new() -> Xorshift64 {
    Xorshift64::seed(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos())   
  }

  fn seed(seed: u32) -> Xorshift64 {
    let seed = ((seed as u64) << 32) | (seed as u64);
    assert_ne!(seed, 0);
    Xorshift64{s: seed, r: seed}
  }

  fn next(&mut self) -> u32 {
    let mut x = self.r;
    x ^= x << 13; 
    x ^= x >> 7; 
    x ^= x << 17; 
    self.r = x;
    (self.r & 0x00000000FFFFFFFF) as u32
  }

  fn next01(&mut self) -> f64 {
    self.next() as f64 / std::u32::MAX as f64
  }

  fn reset(&mut self) {
    self.r = self.s;  
  }
}

mod test {
  use super::*;
  #[test]
  fn test_lcg() {
    let mut gen = LCG::seed(1);
    assert_eq!(gen.next(), LCG::A as u32);
  }

  #[test]
  fn test_xorshift64() {
    let mut gen = Xorshift64::seed(1);
    assert_eq!(gen.next(), 1115824193);
  }
}
