
use crate::dist::*;

#[derive(Debug)]
pub struct Uniform {
  l: f64,
  s: f64
}

#[derive(Debug)]
pub struct Normal {
  mu: f64,
  sigma: f64, 
  // for Box-Muller
  is_cached: bool,
  cached_val: f64
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
  fn sample_1(&mut self, rng: &mut impl PRNG) -> f64 {
    rng.uniform01() * self.s + self.l 
  } 

  fn sample_n(&mut self, n: usize, rng: &mut impl PRNG) -> Vec<f64> {
    (0..n).map(|_| self.sample_1(rng)).collect()
  } 
}

impl Normal {
  pub fn new(mean: f64, variance: f64) -> Normal {
    assert!(variance > 0.0);
    Normal{mu: mean, sigma: variance.sqrt(), is_cached: false, cached_val: std::f64::NAN }
  }
}

impl Dist<f64> for Normal {
  // won't work: impl stricter than trait not allowed
  //fn sample_1<T>(&mut self, rng: &mut T)  -> f64 where T: Gen + Rejectable {
  fn sample_1(&mut self, rng: &mut impl PRNG) -> f64 {
    if self.is_cached {
      self.is_cached = false;
      return self.cached_val;
    }
    loop {
      let (x,y) = (rng.uniform01() * 2.0 - 1.0, rng.uniform01() * 2.0 - 1.0);
      let s = x*x + y*y;
      if s > 0.0 && s < 1.0 {
        let m = (-2.0 * s.ln() / s).sqrt();
        self.is_cached = true;
        self.cached_val = self.mu + self.sigma * y * m;
        return self.mu + self.sigma * x * m;
      }
    }
  } 

  /// Returns a vector of n normal variates
  ///
  /// # Arguments
  ///
  /// * `n` - The number of variates to return
  /// * `rng` - An instance of a pseudorandom generator
  ///
  /// # Example
  /// ```
  /// // Sample 100 normal variates with zero mean and unit variance 
  /// // using Mersenne Twister as the underlying random number generator
  /// use rand::gen::pseudo::*;
  /// use rand::dist::Dist;
  /// use rand::dist::continuous::*;
  /// let mut normdist = Normal::new(0.0, 1.0);
  /// let mut rng = MT19937::new();
  /// let v = normdist.sample_n(100, &mut rng);
  /// ```
  fn sample_n(&mut self, n: usize, rng: &mut impl PRNG) -> Vec<f64> {
    (0..n).map(|_| self.sample_1(rng)).collect()
  } 
}


impl Exponential {
  pub fn new(lambda: f64) -> Exponential {
    assert!(lambda > 0.0);
    Exponential{lambda}
  }
}

impl Dist<f64> for Exponential {

  fn sample_1(&mut self, rng: &mut impl PRNG) -> /*T*/ f64 {
    -rng.uniform01().ln() / self.lambda 
  } 

  fn sample_n(&mut self, n: usize, rng: &mut impl PRNG) -> /*T*/ Vec<f64> {
    (0..n).map(|_| self.sample_1(rng)).collect()
  } 
}

#[cfg(test)]
mod test {
  use super::*;
  //use crate::gen::*;

  const TRIALS: usize = 60000;

  #[test]
  fn test_uniform_lcg() {
    let mut u = Uniform::new(-1.0, 1.0);
    let mut rand = LCG::seed(19937);
    let mu: f64 = u.sample_n(TRIALS, &mut rand).iter().sum::<f64>() / (TRIALS as f64);
    assert!(mu.abs() < (TRIALS as f64).sqrt());
  }

  #[test]
  fn test_uniform_xorshift() {
    let mut u = Uniform::new(-1.0, 1.0);
    let mut rand = Xorshift64::seed(19937);
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
      let mut rand = Xorshift64::seed(19937);
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
  fn test_normal_xorshift() {
    // test variance from 1e-5 to 1e+5
    for i in -5..=5 { 
      let var = 10.0f64.powi(i);
      let mut e = Normal::new(0.0, var);
      let mut rand = Xorshift64::seed(19937);
      let mu: f64 = e.sample_n(TRIALS, &mut rand).iter().sum::<f64>() / (TRIALS as f64);
      // mean should be 0.0 +/- 
      assert!(mu.abs() < (var / (TRIALS as f64)).sqrt());
    }
  }

  #[test]
  #[should_panic]
  fn test_normal_invalid() {
    Normal::new(0.0, 0.0);
  }
}
