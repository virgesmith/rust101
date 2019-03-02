//! gen: Generator module

// mod rand is implicit from project name in Cargo.toml
// mod gen  is implicit from this filename

/// Pseudorandom generator interface
pub trait PRNG {
  /// Initialise using clock as seed
  fn new() -> Self;
  /// Initialise using explicit value
  fn seed(s: u32) -> Self;
  /// Return next integer in the sequence
  fn next_1(&mut self) -> u32;
  /// Return next n integers in the sequence
  fn next_n(&mut self, n: usize) -> Vec<u32>;
  /// Return the next value as a normalised float
  fn uniform01(&mut self) -> f64;
  /// Return the next n values as normalised floats
  fn uniforms01(&mut self, n: usize) -> Vec<f64>;
  /// Reset the generator to its initial state
  fn reset(&mut self) -> &mut Self;
}

/// Linear congruential generator equivalent to the C++11 minstd_rand 
pub struct LCG {
  /// The seed
  s: u32,
  /// The current value
  r: u32
}

pub struct Xorshift64 {
  s: u64,
  r: u64
}

/// untyped pointer to C++ object. rust doesnt need to know the type as it doesnt directly access the object
type MT19937Impl = *const std::ffi::c_void;
pub struct MT19937 {
  seed: u32,
  pimpl: MT19937Impl
}

use std::time::{SystemTime, UNIX_EPOCH};

// private 
impl LCG {
  const A: u64 = 48271;
  const M: u64 = std::i32::MAX as u64;
}

// public
impl PRNG for LCG {
  fn new() -> LCG {
    LCG::seed(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos())   
  }

  fn seed(seed: u32) -> LCG {
    assert_ne!(seed, 0);
    LCG{s: seed, r: seed}
  }

  fn next_1(&mut self) -> u32 {
    self.r = ((self.r as u64 * LCG::A) % LCG::M) as u32;
    self.r
  }

  fn next_n(&mut self, n: usize) -> Vec<u32> {
    (0..n).map(|_| self.next_1()).collect()
  }

  fn uniform01(&mut self) -> f64 {
    self.next_1() as f64 / LCG::M as f64
  }

  fn uniforms01(&mut self, n: usize) -> Vec<f64> {
    (0..n).map(|_| self.uniform01()).collect()
  }

  fn reset(&mut self) -> &mut Self {
    self.r = self.s;  
    self
  }
}

impl Xorshift64 { }

impl PRNG for Xorshift64 {
  fn new() -> Xorshift64 {
    Xorshift64::seed(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos())   
  }

  fn seed(seed: u32) -> Xorshift64 {
    let seed = ((seed as u64) << 32) | (seed as u64);
    assert_ne!(seed, 0);
    Xorshift64{s: seed, r: seed}
  }

  fn next_1(&mut self) -> u32 {
    let mut x = self.r;
    x ^= x << 13; 
    x ^= x >> 7; 
    x ^= x << 17; 
    self.r = x;
    (self.r & 0x00000000FFFFFFFF) as u32
  }

  fn next_n(&mut self, n: usize) -> Vec<u32> {
    (0..n).map(|_| self.next_1()).collect()
  }

  fn uniform01(&mut self) -> f64 {
    self.next_1() as f64 / 2.0f64.powi(32)
  }

  fn uniforms01(&mut self, n: usize) -> Vec<f64> {
    (0..n).map(|_| self.uniform01()).collect()
  }

  fn reset(&mut self) -> &mut Self {
    self.r = self.s; 
    self 
  }
}


#[link(name = "mt19937", kind = "static")]
extern {
  // std::mt19937* mt19937_create(uint32_t seed)
  fn mt19937_create(seed: u32) -> MT19937Impl;
  // uint32_t mt_19937_next(std::mt19937* pimpl) 
  fn mt19937_next(pimpl: MT19937Impl) -> u32;
  // void mt19937_destroy(std::mt19937* pimpl)
  fn mt19937_destroy(pimpl: MT19937Impl) -> ();
}

impl Drop for MT19937 {
  fn drop(&mut self) {
    unsafe { mt19937_destroy(self.pimpl); }
  }
}

impl PRNG for MT19937 {
  fn new() -> MT19937 {
    MT19937::seed(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos())   
  }

  fn seed(seed: u32) -> MT19937 {
    unsafe { MT19937{seed:seed, pimpl: mt19937_create(seed)} }
  }

  fn next_1(&mut self) -> u32 {
    unsafe { mt19937_next(self.pimpl) }
  }

  fn next_n(&mut self, n: usize) -> Vec<u32> {
    (0..n).map(|_| self.next_1()).collect()
  }

  fn uniform01(&mut self) -> f64 {
    self.next_1() as f64 / 2.0f64.powi(32)
  }

  fn uniforms01(&mut self, n: usize) -> Vec<f64> {
    (0..n).map(|_| self.uniform01()).collect()
  }

  fn reset(&mut self) -> &mut Self {
    unsafe { 
      mt19937_destroy(self.pimpl); 
      self.pimpl = mt19937_create(self.seed);
    }
    self
  }

}

mod test {
  use super::*;

  const TRIALS: usize = 10000;
  #[test]
  fn test_lcg() {
    let mut gen = LCG::seed(1);
    assert_eq!(gen.next_1(), LCG::A as u32);

    let mean: f64 = gen.uniforms01(TRIALS).iter().sum::<f64>() / (TRIALS as f64);
    assert!(mean > 0.49 && mean < 0.51);
  }

  #[test]
  #[should_panic]
  fn test_lcg_failures() {
    LCG::seed(0);
  }

  #[test]
  fn test_xorshift64() {
    let mut gen = Xorshift64::seed(1);
    assert_eq!(gen.next_1(), 1115824193);

    let mean: f64 = gen.uniforms01(TRIALS).iter().sum::<f64>() / (TRIALS as f64);
    assert!(mean > 0.49 && mean < 0.51);
  }

  #[test]
  #[should_panic]
  fn test_xorshift64_failures() {
    Xorshift64::seed(0);
  }  

  #[test]
  fn test_mt19937() {
    let mut gen = MT19937::new();
    let r = gen.next_n(10);
    assert_eq!(r, gen.reset().next_n(10));
  }
}
