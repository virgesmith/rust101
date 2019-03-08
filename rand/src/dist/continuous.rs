
use crate::dist::*;
use crate::dist::normal::*;

#[derive(Debug)]
pub struct Uniform {
  l: f64,
  s: f64
}

#[derive(Debug)]
pub struct Normal<T> {
  mu: f64,
  sigma: f64, 
  transform: T
}

#[derive(Debug)]
pub struct Exponential {
  lambda: f64
}

impl Uniform {
  pub fn new(l: f64, h: f64) -> Uniform {
    assert!(h > l);
    Uniform{l: l, s: h-l}
  }
}

impl Dist<f64> for Uniform {
  fn sample_1<R: RandomStream + Dimensionless>(&mut self, rng: &mut R) -> f64 {
    rng.uniform01() * self.s + self.l 
  } 

  fn sample_n<R: RandomStream + Dimensionless>(&mut self, n: usize, rng: &mut R) -> Vec<f64> {
    (0..n).map(|_| self.sample_1(rng)).collect()
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
  pub fn sample_n(&mut self, n: usize) -> Vec<f64> {
    self.transform.get_n(n).iter().map(|&r| self.mu + self.sigma * r).collect()
  } 
}

impl<R: RandomStream + Dimensionless + Rejectable> Normal<Polar<R>> {
  pub fn new(mean: f64, variance: f64, rng: R) -> Normal<Polar<R>> {
    assert!(variance > 0.0);
    Normal{mu: mean, sigma: variance.sqrt(), transform: Polar::new(rng) }
  }

  pub fn sample_1(&mut self) -> f64 {
    self.mu + self.sigma * self.transform.get_n(1)[0]
  } 

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
  pub fn sample_n(&mut self, n: usize) -> Vec<f64> {
    self.transform.get_n(n).iter().map(|&r| self.mu + self.sigma * r).collect()
  } 
}


impl Exponential {
  pub fn new(lambda: f64) -> Exponential {
    assert!(lambda > 0.0);
    Exponential{lambda}
  }
}

impl Dist<f64> for Exponential {
  fn sample_1<R: RandomStream + Dimensionless>(&mut self, rng: &mut R) -> f64 {
    -rng.uniform01().ln() / self.lambda 
  } 

  fn sample_n<R: RandomStream + Dimensionless>(&mut self, n: usize, rng: &mut R) -> Vec<f64> {
    (0..n).map(|_| self.sample_1(rng)).collect()
  } 
}

#[cfg(test)]
mod test {
  use super::*;

  const TRIALS: usize = 60000;

  #[test]
  fn test_uniform_lcg() {
    let mut u = Uniform::new(-1.0, 1.0);
    let mut rand = LCG::new(Some(19937));
    let mu: f64 = u.sample_n(TRIALS, &mut rand).iter().sum::<f64>() / (TRIALS as f64);
    assert!(mu.abs() < (TRIALS as f64).sqrt());
  }

  #[test]
  fn test_uniform_xorshift() {
    let mut u = Uniform::new(-1.0, 1.0);
    let mut rand = Xorshift64::new(Some(19937));
    let mu: f64 = u.sample_n(TRIALS, &mut rand).iter().sum::<f64>() / (TRIALS as f64);
    assert!(mu.abs() < (TRIALS as f64).sqrt());
  }

  #[test]
  #[should_panic]
  fn test_uniform_invalid() {
    Uniform::new(1.0, 1.0);
  }

  #[test]
  fn test_exponential_xorshift() {
    // test k from 1e-5 to 1e+5
    for i in -5..6 { 
      let k = 10.0f64.powi(i);
      let mut e = Exponential::new(k);
      let mut rand = Xorshift64::new(Some(19937));
      let mu: f64 = e.sample_n(TRIALS, &mut rand).iter().sum::<f64>() / (TRIALS as f64);
      println!("{} {}", mu, 1.0/k);
      // mean should be 1/k
      assert!((mu * k - 1.0).abs() < 1.0 / (TRIALS as f64).sqrt());
    }
  }

  #[test]
  #[should_panic]
  fn test_exponential_invalid() {
    Exponential::new(0.0);
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

  // #[test]
  // #[should_panic]
  // fn test_normal_invalid_combination() {
  //   // can't use rejection sampling with quasirandom generator
  //   let mut dist = Normal::<Polar>::new(0.0, 1.0);
  //   let mut rng = Sobol::new(1);
  //   let x = dist.sample_n(1, &mut rng);
  // }

  // #[test]
  // fn test_normal_quasi() {
  //   // can't use rejection sampling with quasirandom generator
  //   let mut dist = Normal::<InverseCumulative>::new(0.0, 1.0);
  //   let mut rng = Sobol::new(1);
  //   let x = dist.sample_n(1, &mut rng);
  // }
}
