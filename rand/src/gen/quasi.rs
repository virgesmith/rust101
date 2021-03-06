//! gen::quasi: quasirandom Generator module

// mod rand is implicit from project name in Cargo.toml
// mod gen  is implicit from the path
// mod quasi is implicit from the filename
use crate::gen::*;

/// untyped pointer to C struct. rust doesnt need to know the type as it doesnt directly access the object
type SobolImpl = *const std::ffi::c_void;
pub struct Sobol {
  dim: u32,
  cache: Vec<u32>,
  pimpl: SobolImpl
}

#[link(name = "sobol", kind = "static")]
extern {
  // SobolData* nlopt_sobol_create(uint32_t sdim)
  fn nlopt_sobol_create(dim: u32) -> SobolImpl;
  // int nlopt_sobol_next(SobolData* s, uint32_t *x)
  fn nlopt_sobol_next(pimpl: SobolImpl, dest: &u32) -> i32;
  // void nlopt_sobol_skip(SobolData* s, uint32_t n, uint32_t *x)
  fn nlopt_sobol_skip(pimpl: SobolImpl, skips: u32, dest: &u32);
  // void nlopt_sobol_destroy(SobolData* s)
  fn nlopt_sobol_destroy(pimpl: SobolImpl);
  // inline uint32_t sobol_maxdim() { return MAXDIM; }
  fn sobol_maxdim() -> u32;
}

impl Drop for Sobol {
  fn drop(&mut self) {
    unsafe { nlopt_sobol_destroy(self.pimpl); }
  }
}

impl Sobol {
  pub fn new(dim: u32) -> Sobol {
    assert!(dim > 0 && dim <= unsafe {sobol_maxdim()});
    let this = Sobol{dim, cache: vec![0; dim as usize], pimpl: unsafe { nlopt_sobol_create(dim) } };
    // initialise cache
    unsafe { nlopt_sobol_next(this.pimpl, &this.cache[0]); }
    this
  }
}


impl Dimensioned for Sobol {
  fn dim(&self) -> u32 {
    self.dim
  }
}

impl RandomStream for Sobol {
  fn next_n(&mut self, n: usize) -> Vec<u32> {
    // only a multiple of dim is allowed
    assert_eq!(n % self.cache.len(), 0);
    // clone the cache
    let mut result = Vec::new();
    for _ in 0..n/self.cache.len() {
      result.append(&mut self.cache.clone());
      unsafe { nlopt_sobol_next(self.pimpl, &self.cache[0]); }
    }
    result
  }

  fn uniforms01(&mut self, n: usize) -> Vec<f64> {
    const SCALE: f64 = 1.0 / 4294967296.0;
    self.next_n(n).iter().map(|&r| r as f64 * SCALE).collect()
  }

}

impl Resettable for Sobol {
  fn reset(&mut self) -> &mut Self {
    unsafe {
      nlopt_sobol_destroy(self.pimpl);
      self.pimpl = nlopt_sobol_create(self.dim);
      // refresh cache
      nlopt_sobol_next(self.pimpl, &self.cache[0]);
    }
    self
  }

  fn skip(&mut self, n: u32) -> &mut Self {
    //println!("{:?}", self.cache);
    unsafe { nlopt_sobol_skip(self.pimpl, n * self.dim, &self.cache[0]); }
    //println!("{:?}", self.cache);
    self
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_sobol() {
    let mut gen = Sobol::new(8);
    assert_eq!(gen.dim, 8);
    assert_eq!(gen.uniforms01(8), vec![0.5; 8]);

    let mut gen = Sobol::new(2);
    gen.next_n(2);
    assert_eq!(gen.uniforms01(2), vec![0.75, 0.25]);
    assert_eq!(gen.uniforms01(2), vec![0.25, 0.75]);
    // reset and skip forward
    assert_eq!(gen.reset().skip(2).uniforms01(2), vec![0.25, 0.75]);

    let mut gen = Sobol::new(1111);
    assert_eq!(gen.uniforms01(1111), vec![0.5; 1111]);
    assert_eq!(gen.reset().uniforms01(1111), vec![0.5; 1111]);
  }

  #[test]
  #[should_panic]
  fn test_sobol_failures() {
    Sobol::new(0);
  }

  #[test]
  #[should_panic]
  fn test_sobol_failures2() {
    Sobol::new(unsafe { sobol_maxdim() } + 1);
  }
}
