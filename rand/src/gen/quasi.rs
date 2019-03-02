//! gen::quasi: quasirandom Generator module

// mod rand is implicit from project name in Cargo.toml
// mod gen  is implicit from the path
// mod quasi is implicit from the filename


/// Quasirandom generator interface
pub trait QRNG {
  // Initialise using dimension d
  fn new(dim: u32) -> Self;
  /// Return next integers in the sequence (one per dimension)
  fn next_d(&self) -> Vec<u32>;
  /// Return next integers in the sequence (one per dimension)
  fn uniforms01(&self) -> Vec<f64>;
  /// Skip n values in the sequence (n * dimension values)
  fn skip(&self, n: u32) -> &Self;
  /// Reset the generator to its initial state
  fn reset(&mut self) -> &mut Self;
}

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
  fn nlopt_sobol_skip(pimpl: SobolImpl, skips: u32, dest: &u32) -> ();
  // void nlopt_sobol_destroy(SobolData* s)
  fn nlopt_sobol_destroy(pimpl: SobolImpl) -> ();
  // inline uint32_t sobol_maxdim() { return MAXDIM; }
  fn sobol_maxdim() -> u32;
}

impl Drop for Sobol {
  fn drop(&mut self) {
    unsafe { nlopt_sobol_destroy(self.pimpl); }
  }
}

impl QRNG for Sobol {
  fn new(dim: u32) -> Sobol {
    assert!(dim > 0 && dim <= unsafe {sobol_maxdim()});
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
    assert_eq!(gen.reset().skip(2).uniforms01(), vec![0.25, 0.75]);

    let mut gen = Sobol::new(1111);
    assert_eq!(gen.uniforms01(), vec![0.5; 1111]);
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
