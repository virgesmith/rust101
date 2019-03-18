
//use crate::dist::*;
use crate::gen::{RandomStream, Dimensionless, Rejectable};
use crate::dist::normal::{Polar, InverseCumulative};
use crate::dist::Dist;

#[derive(Debug)]
pub struct Uniform<R> {
  l: f64,
  s: f64,
  rng: R 
}

#[derive(Debug)]
pub struct Normal<T> {
  mu: f64,
  sigma: f64, 
  transform: T
}

#[derive(Debug)]
pub struct Exponential<R> {
  lambda: f64,
  rng: R
}

impl<R: RandomStream> Uniform<R> {
  pub fn new(l: f64, h: f64, rng: R) -> Uniform<R> {
    assert!(h > l);
    Uniform{l: l, s: h-l, rng: rng}
  }
}

impl<R: RandomStream> Dist<f64> for Uniform<R> {
  // fn sample_1(&mut self) -> f64 {
  //   self.rng.uniform01() * self.s + self.l 
  // } 

  fn sample_n(&mut self, n: usize) -> Vec<f64> {
    self.rng.uniforms01(n).iter().map(|&x| self.l + self.s * x).collect()
  } 
}

impl<R: RandomStream> Normal<InverseCumulative<R>> {
  pub fn new(mean: f64, variance: f64, rng: R) -> Normal<InverseCumulative<R>> {
    assert!(variance > 0.0);
    Normal{mu: mean, sigma: variance.sqrt(), transform: InverseCumulative::new(rng) }
  }

  pub fn sample_1(&mut self) -> f64 {
    self.mu + self.sigma * self.transform.get_n(1)[0]
  } 
}

impl<R: RandomStream> Dist<f64> for Normal<InverseCumulative<R>> {
  /// Returns a vector of n normal variates
  ///
  /// # Arguments
  ///
  /// * `n` - The number of variates to return
  ///
  /// # Example
  /// ```
  /// // Sample 100 normal variates with zero mean and unit variance 
  /// // using Mersenne Twister as the underlying random number generator
  /// // with Marsaglia's polar transformation to convert to normal
  /// use rand::gen::{*, pseudo::*};
  /// use rand::dist::{Dist, continuous::*, normal::*};
  /// // init Mersenne Twister using system clock
  /// let mut normdist = Normal::<InverseCumulative<MT19937>>::new(0.0, 1.0, MT19937::new(None));
  /// let v = normdist.sample_n(100);
  /// ```
  fn sample_n(&mut self, n: usize) -> Vec<f64> {
    self.transform.get_n(n).iter().map(|&r| self.mu + self.sigma * r).collect()
  } 
}

impl<R: RandomStream + Dimensionless + Rejectable> Normal<Polar<R>> {
  pub fn new(mean: f64, variance: f64, rng: R) -> Normal<Polar<R>> {
    assert!(variance > 0.0);
    Normal{mu: mean, sigma: variance.sqrt(), transform: Polar::new(rng) }
  }

  // pub fn sample_1(&mut self) -> f64 {
  //   self.mu + self.sigma * self.transform.get_n(1)[0]
  // } 
}

impl<R: RandomStream + Dimensionless + Rejectable> Dist<f64> for Normal<Polar<R>> {
  /// Returns a vector of n normal variates
  ///
  /// # Arguments
  ///
  /// * `n` - The number of variates to return
  ///
  /// # Example
  /// ```
  /// // Sample 100 normal variates with zero mean and unit variance 
  /// // using Mersenne Twister as the underlying random number generator
  /// // with Marsaglia's polar transformation to convert to normal
  /// use rand::gen::{*, pseudo::*};
  /// use rand::dist::{Dist, continuous::*, normal::*};
  /// // init Mersenne Twister using system clock
  /// let mut normdist = Normal::<Polar<MT19937>>::new(0.0, 1.0, MT19937::new(None));
  /// let v = normdist.sample_n(100);
  /// ```
  fn sample_n(&mut self, n: usize) -> Vec<f64> {
    self.transform.get_n(n).iter().map(|&r| self.mu + self.sigma * r).collect()
  } 
}

// TODO implement a transform layer (so can add e.g. Ziggurat)
// currently hard-coded to inverse
impl<R: RandomStream> Exponential<R> {
  pub fn new(lambda: f64, rng: R) -> Exponential<R> {
    assert!(lambda > 0.0);
    Exponential{lambda, rng}
  }
}

impl<R: RandomStream> Dist<f64> for Exponential<R> {
  // fn sample_1<R: RandomStream + Dimensionless>(&mut self, rng: &mut R) -> f64 {
  //   -rng.uniform01().ln() / self.lambda 
  // } 

  fn sample_n(&mut self, n: usize) -> Vec<f64> {
    self.rng.uniforms01(n).iter().map(|&r| -r.ln() / self.lambda).collect()
  } 
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::gen::pseudo::*;
  use crate::gen::quasi::*;
  use crate::gen::entropy::*;

  const TRIALS: usize = 60000;

  #[test]
  fn test_uniform_lcg() {
    let mut u = Uniform::new(-1.0, 1.0, LCG::new(Some(19937)));
    let mu: f64 = u.sample_n(TRIALS).iter().sum::<f64>() / (TRIALS as f64);
    assert!(mu.abs() < (TRIALS as f64).sqrt());
  }

  #[test]
  fn test_uniform_entropy() {
    let mut u = Uniform::new(-1.0, 1.0, EntropySource::new());
    let mu: f64 = u.sample_n(TRIALS).iter().sum::<f64>() / (TRIALS as f64);
    assert!(mu.abs() < (TRIALS as f64).sqrt());
  }

  #[test]
  fn test_uniform_xorshift() {
    let mut u = Uniform::new(-1.0, 1.0, Xorshift64::new(Some(19937)));
    let mu: f64 = u.sample_n(TRIALS).iter().sum::<f64>() / (TRIALS as f64);
    assert!(mu.abs() < (TRIALS as f64).sqrt());
  }


  #[test]
  #[should_panic]
  fn test_uniform_invalid() {
    Uniform::new(1.0, 1.0, LCG::new(None));
  }

  #[test]
  fn test_exponential_xorshift() {
    // test k from 1e-5 to 1e+5
    for i in -5..6 { 
      let k = 10.0f64.powi(i);
      let mut e = Exponential::new(k, Xorshift64::new(Some(19937)));
      let mu: f64 = e.sample_n(TRIALS).iter().sum::<f64>() / (TRIALS as f64);
      println!("{} {}", mu, 1.0/k);
      // mean should be 1/k
      assert!((mu * k - 1.0).abs() < 1.0 / (TRIALS as f64).sqrt());
    }
  }

  #[test]
  #[should_panic]
  fn test_exponential_invalid() {
    Exponential::new(0.0, LCG::new(None));
  }

  #[test]
  fn test_normal_inversecumulative_xorshift() {
    // test variance from 1e-5 to 1e+5
    for i in -5..=5 { 
      let var = 10.0f64.powi(i);
      let mut e = Normal::<InverseCumulative<Xorshift64>>::new(0.0, var, Xorshift64::new(Some(19937)));
      let mu: f64 = e.sample_n(TRIALS).iter().sum::<f64>() / (TRIALS as f64);
      // mean should be 0.0 +/- 
      assert!(mu.abs() < (var / (TRIALS as f64)).sqrt());
    }
  }

  #[test]
  fn test_normal_polar_xorshift() {
    // test variance from 1e-5 to 1e+5
    for i in -5..=5 { 
      let var = 10.0f64.powi(i);
      let mut e = Normal::<Polar<Xorshift64>>::new(0.0, var, Xorshift64::new(Some(19937)));
      let mu: f64 = e.sample_n(TRIALS).iter().sum::<f64>() / (TRIALS as f64);
      // mean should be 0.0 +/- 
      assert!(mu.abs() < (var / (TRIALS as f64)).sqrt());
    }
  }

  #[test]
  #[should_panic]
  fn test_normal_invalid() {
    Normal::<Polar<LCG>>::new(0.0, 0.0, LCG::new(None));
  }

  #[test]
  fn test_normal_quasi() {
    // can't use rejection sampling with quasirandom generator
    // let mut dist = Normal::<Polar<Sobol>>::new(0.0, 1.0, Sobol::new(1));
    let mut dist = Normal::<InverseCumulative<Sobol>>::new(0.0, 1.0, Sobol::new(1));
    let _ = dist.sample_n(1);
  }
}
