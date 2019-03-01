//! gen: Generator module

// mod rand is implicit from project name in Cargo.toml
// mod gen  is implicit from this filename

// Pseudo Generator interface
// Initialise using clock as seed
pub trait PRNG {
  fn new() -> Self;
  fn seed(s: u32) -> Self;
  fn next_1(&mut self) -> u32;
  fn next_n(&mut self, n: usize) -> Vec<u32>;
  fn uniform01(&mut self) -> f64;
  fn uniforms01(&mut self, n: usize) -> Vec<f64>;
  fn reset(&mut self) -> &mut Self;
}

// Quasi Generator interface
// Initialise using dimension
pub trait QRNG {
  fn new(dim: u32) -> Self;
  fn next_d(&self) -> Vec<u32>;
  fn uniforms01(&self) -> Vec<f64>;
  fn skip(&self, n: u32) -> &Self;
  fn reset(&mut self) -> &mut Self;
}

pub struct LCG {
  s: u32,
  r: u32
}

pub struct Xorshift64 {
  s: u64,
  r: u64
}

// untyped pointer to C++ object
type MT19937Impl = *const std::ffi::c_void;
pub struct MT19937 {
  seed: u32,
  pimpl: MT19937Impl
}


// untyped pointer to C struct
type SobolImpl = *const std::ffi::c_void;
pub struct Sobol {
  dim: u32,
  cache: Vec<u32>,
  pimpl: SobolImpl
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

impl MT19937 {
  fn drop(&self) {
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

#[link(name = "sobol", kind = "static")]
extern {
  // SobolData* nlopt_sobol_create(uint32_t sdim)
  fn nlopt_sobol_create(dim: u32) -> SobolImpl;
  // int nlopt_sobol_next(SobolData* s, uint32_t *x)
  fn nlopt_sobol_next(pimpl: SobolImpl, dest: &u32) -> i32;
  // void nlopt_sobol_skip(SobolData* s, uint32_t n, uint32_t *x)
  fn nlopt_sobol_skip(pimpl: SobolImpl, skips: u32, dest: &u32) -> ();
  // void nlopt_sobol_destroy(SobolData* s)
  fn nlopt_sobol_destroy(pimpl: SobolImpl) -> ();
}

impl Sobol {
  fn drop(&mut self) {
    unsafe { nlopt_sobol_destroy(self.pimpl); }
  }
}



impl QRNG for Sobol {
  fn new(dim: u32) -> Sobol {
    let this = Sobol{dim: dim, cache: vec![0; dim as usize], pimpl: unsafe { nlopt_sobol_create(dim) } };
    // initialise cache
    unsafe { nlopt_sobol_next(this.pimpl, &this.cache[0]); }
    this
  }

  fn next_d(&self) -> Vec<u32> {
    // clone the cache
    let result = self.cache.clone();
    // update
    unsafe { nlopt_sobol_next(self.pimpl, &self.cache[0]); }
    // return the cloned cache
    result
  }

  fn uniforms01(&self) -> Vec<f64> {
    // calc uniforms
    let result = self.cache.iter().map(|&x| x as f64 / 2.0f64.powi(32)).collect();
    // update cache before returning
    unsafe { nlopt_sobol_next(self.pimpl, &self.cache[0]); };
    result
  }

  fn skip(&self, n: u32) -> &Self {
    println!("{:?}", self.cache);
    unsafe { nlopt_sobol_skip(self.pimpl, n * self.dim, &self.cache[0]); }
    println!("{:?}", self.cache);
    self
  }

  fn reset(&mut self) -> &mut Self {
    unsafe { 
      nlopt_sobol_destroy(self.pimpl); 
      self.pimpl = nlopt_sobol_create(self.dim);
      // refresh cache
      nlopt_sobol_next(self.pimpl, &self.cache[0]);
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
    assert_eq!(r, gen.reset().next_n(10));
  }

  #[test]
  fn test_sobol() {
    let gen = Sobol::new(8);
    assert_eq!(gen.dim, 8);
    assert_eq!(gen.uniforms01(), vec![0.5; 8]);

    let mut gen = Sobol::new(2);
    gen.next_d();
    assert_eq!(gen.uniforms01(), vec![0.75, 0.25]);
    assert_eq!(gen.uniforms01(), vec![0.25, 0.75]);
    // reset and skip forward
    assert_eq!(gen.reset().skip(2).uniforms01(), vec![0.25, 0.75])
  }
}
