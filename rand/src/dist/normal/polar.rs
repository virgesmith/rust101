// Algorithms to transform uniform variates to normal 
use crate::gen::RandomStream;
use crate::gen::Rejectable;
use crate::gen::Dimensionless;

// Marsaglia's polar method of sampling normals
#[derive(Debug)]
pub struct Polar<R> {
  rng: R,
  is_cached: bool,
  cached_val: f64
}

impl<R: RandomStream + Dimensionless + Rejectable> Polar<R> {
  pub fn new(rng: R) -> Polar<R> {
    Polar{rng: rng, is_cached: false, cached_val: std::f64::NAN}
  }

  fn get_impl(&mut self) -> f64 {
    if self.is_cached {
      self.is_cached = false;
      return self.cached_val;
    }
    loop {
      let (x,y) = (self.rng.uniform01() * 2.0 - 1.0, self.rng.uniform01() * 2.0 - 1.0);
      let s = x*x + y*y;
      if s > 0.0 && s < 1.0 {
        let m = (-2.0 * s.ln() / s).sqrt();
        self.is_cached = true;
        self.cached_val = y * m;
        return x * m;
      }
    }
  }

  pub fn get_n(&mut self, n: usize) -> Vec<f64> {
    (0..n).map(|_| self.get_impl()).collect()
  }
}


#[cfg(test)]
mod test {
  use super::*;
  use crate::gen::pseudo::*;
  //use crate::gen::entropy::*;

  #[test]
  fn test_polar() {
    const N: usize = 10000;
  //   let f = (0..N).map(|i| (i as f64)/(N as f64)).collect::<Vec<f64>>();
  //   let x = f.iter().map(|&fi| standard_inv_cdf(fi)).collect::<Vec<f64>>();
  //   for i in 0..N {
  //     assert!((f[i] - standard_cdf(x[i])).abs() < std::f64::EPSILON);
  //   }

    let mut polar = Polar::new(MT19937::new(Some(77027465)));
    let v = polar.get_n(N);
    println!("{} {}", N, v.iter().sum::<f64>());
    // mean should be < 1/sqrt(N) so sum should be < sqrt(N)
    assert!(v.iter().sum::<f64>() < (N as f64).sqrt());
  }
}
