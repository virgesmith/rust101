//! gen: Generator module

// mod rand is implicit from project name in Cargo.toml
// mod gen  is implicit from this filename

//! Generator interface
//! Initialise using clock as seed
//! 
pub trait Gen {
  fn new() -> Self;
  fn seed(s: u32) -> Self;
  fn next_1(&mut self) -> u32;
  fn next_n(&mut self, n: usize) -> Vec<u32>;
  fn uniform01(&mut self) -> f64;
  fn uniforms01(&mut self, n: usize) -> Vec<f64>;
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

type MT19937Impl = u64;
pub struct MT19937 {
  seed: u32,
  //cache: Vec<u32>,
  // pointer...
  pimpl: MT19937Impl
}


// hack a pointer to C struct
type SobolData = u64;
pub struct Sobol {
  dim: usize,
  cache: Vec<u32>,
  // pointer...
  pimpl: SobolData
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

  fn reset(&mut self) {
    self.r = self.s;  
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

impl MT19937 {
  fn drop(&self) {
    unsafe { mt19937_destroy(self.pimpl); }
  }
}

impl Gen for MT19937 {
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

  fn reset(&mut self) {
    unsafe { 
      mt19937_destroy(self.pimpl); 
      self.pimpl = mt19937_create(self.seed);
    }
  }

}

#[link(name = "sobol", kind = "static")]
extern {
  // SobolData* nlopt_sobol_create(uint32_t sdim)
  fn nlopt_sobol_create(dim: u32) -> SobolData;
  // int nlopt_sobol_next(SobolData* s, uint32_t *x)
  fn nlopt_sobol_next(pimpl: SobolData, dest: &u32) -> i32;
  // void nlopt_sobol_skip(SobolData* s, uint32_t n, uint32_t *x)
  fn nlopt_sobol_skip(pimpl: SobolData, skips: u32, dest: &u32) -> ();
  // void nlopt_sobol_destroy(SobolData* s)
  fn nlopt_sobol_destroy(pimpl: SobolData) -> ();
}

impl Sobol {
  pub fn new(dim: u32) -> Sobol {
    Sobol{dim: dim as usize, cache: vec![0; dim as usize], pimpl: unsafe { nlopt_sobol_create(dim) } }
  }

  pub fn drop(&self) {
    unsafe { nlopt_sobol_destroy(self.pimpl); }
  }

  pub fn next_d(&self) -> Vec<f64> {
    unsafe { nlopt_sobol_next(self.pimpl, &self.cache[0]); }
    self.cache.iter().map(|&x| x as f64 / 2.0f64.powi(32)).collect()
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
  fn test_xorshift64() {
    let mut gen = Xorshift64::seed(1);
    assert_eq!(gen.next_1(), 1115824193);

    let mean: f64 = gen.uniforms01(TRIALS).iter().sum::<f64>() / (TRIALS as f64);
    assert!(mean > 0.49 && mean < 0.51);
  }

  #[test]
  fn test_mt19937() {
    let mut gen = MT19937::new();
    let r = gen.next_n(10);
    gen.reset();
    assert_eq!(r, gen.next_n(10));
  }

  #[test]
  fn test_sobol() {
    let gen = Sobol::new(8);
    assert_eq!(gen.dim, 8);
    assert_eq!(gen.next_d(), vec![0.5; 8]);

    let gen = Sobol::new(2);
    gen.next_d();
    assert_eq!(gen.next_d(), vec![0.75, 0.25]);
    assert_eq!(gen.next_d(), vec![0.25, 0.75]);
  }
}
