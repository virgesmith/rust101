
// extern crate num;
// use num::Num;

#[derive(Debug)]
// TODO template
pub struct Discrete {
  v: Vec<i32>,
}

#[derive(Debug)]
// TODO template?
pub struct DiscreteWeighted {
  v: Vec<i32>,
  p: Vec<f64>
}

pub struct WithoutReplacement {
  v: Vec<i32>,
  f: Vec<u32>
}

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

use crate::gen::PRNG;

pub trait Dist<T> {
  fn sample_1(&mut self, rng: &mut impl PRNG) -> T;
  fn sample_n(&mut self, n: usize, rng: &mut impl PRNG) -> Vec<T>;
}


impl Discrete {
  pub fn new(a: &[i32]) -> Discrete {
    assert!(a.len() > 0);
    Discrete{v:a.to_vec()}
  }
}

impl Dist<i32> for Discrete {
  fn sample_1(&mut self, rng: &mut impl PRNG) -> i32 {
    let i = rng.next_1() as usize % self.v.len(); 
    self.v[i]
  } 

  fn sample_n(&mut self, n: usize, rng: &mut impl PRNG) -> Vec<i32> {
    (0..n).map(|_| self.v[rng.next_1() as usize % self.v.len()]).collect()
  } 
}

impl DiscreteWeighted {
  pub fn new(a: &[(i32,f64)]) -> DiscreteWeighted {
    assert!(a.len() > 0);
    let mut s = 0.0;
    // check probs in [0,1]
    let p = a.iter().fold(Vec::with_capacity(a.len()), |mut acc, p| { s += p.1; acc.push(s); acc });
    // check probabilities sum to unity
    assert!(p.last().unwrap().abs() - 1.0 < std::f64::EPSILON);
    DiscreteWeighted{ v: a.iter().fold(Vec::with_capacity(a.len()), |mut acc, p| { acc.push(p.0); acc }),
              p: p}
  }
}

impl Dist<i32> for DiscreteWeighted {
  fn sample_1(&mut self, rng: &mut impl PRNG) -> i32 {
    let r = rng.uniform01();
    // first element of p > r
    // TODO bisect?
    for i in 0..self.p.len() {
      if self.p[i] > r {
        return self.v[i];
      } 
    }
    // TODO better way?
    panic!("DiscreteWeighted sample failure, is Generator working correctly?");
  } 

  fn sample_n(&mut self, n: usize, rng: &mut impl PRNG) -> /*T*/ Vec<i32> {
    (0..n).map(|_| self.sample_1(rng)).collect()
  } 
}

impl WithoutReplacement {
  pub fn new(state_occs: &[(i32,u32)]) -> WithoutReplacement {
    assert!(state_occs.len() > 0);
    WithoutReplacement{ v: state_occs.iter().map(|&(v,_)| v).collect(), 
                        f: state_occs.iter().map(|&(_,f)| f).collect() }
  }

  pub fn empty(&self) -> bool {
    self.f.iter().sum::<u32>() == 0
  }
}

impl Dist<i32> for WithoutReplacement {
  fn sample_1(&mut self, rng: &mut impl PRNG) -> i32
  {
    let mut s = 0;
    let cumul = self.f.iter().fold(Vec::with_capacity(self.f.len()), |mut acc, f| { s += f; acc.push(s); acc });
    let r = rng.next_1() % cumul.last().unwrap();
    for i in 0..cumul.len() {
      if cumul[i] > r {
        self.f[i] -= 1;
        return self.v[i];
      } 
    }
    // TODO better way?
    panic!("WithoutReplacement sample failure, is Generator working correctly?");
  }

  fn sample_n(&mut self, n: usize, rng: &mut impl PRNG) -> /*T*/ Vec<i32> {
    (0..n).map(|_| self.sample_1(rng)).collect()
  }
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
    (0..n).map(|_| rng.uniform01() * self.s + self.l).collect()
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
  /// use rand::gen::*;
  /// use rand::dist::*;
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
    (0..n).map(|_| -rng.uniform01().ln() / self.lambda).collect()
  } 
}


mod test {
  use super::*;
  use crate::gen::*;

  const TRIALS: usize = 60000;

  #[test]
  fn test_discrete_lcg() {
    let mut h = vec![0; 6];
    let mut die = Discrete::new(&vec![1,2,3,4,5,6]);
    let mut rand = LCG::seed(19937);
    let r = die.sample_n(TRIALS, &mut rand);
    for i in 0..TRIALS {
      h[r[i] as usize - 1] += 1;
    }
    let lo = (TRIALS as f64 / 6.0 - 2.0 * (TRIALS as f64).sqrt()) as i32; 
    let hi = (TRIALS as f64 / 6.0 + 2.0 * (TRIALS as f64).sqrt()) as i32; 
    for n in h {
      assert!(n > lo && n < hi);      
    }
  }

  #[test]
  #[should_panic]
  fn test_discrete_invalid() {
    Discrete::new(&vec![]);
  }

  #[test]
  fn test_discrete_xorshift() {
    let mut h = vec![0; 6];
    let mut die = Discrete::new(&vec![1,2,3,4,5,6]);
    let mut rand = Xorshift64::seed(19937);
    for _ in 0..TRIALS {
      h[die.sample_1(&mut rand) as usize-1] += 1;
    }
    let lo = (TRIALS as f64 / 6.0 - 1.0 * (TRIALS as f64).sqrt()) as i32; 
    let hi = (TRIALS as f64 / 6.0 + 1.0 * (TRIALS as f64).sqrt()) as i32; 
    for n in h {
      assert!(n > lo && n < hi);      
    }
  }

  #[test]
  fn test_discrete_flat_weighted_xorshift() {
    let mut h = vec![0; 6];
    let p = 1.0 / 6.0;
    let mut fair_die = DiscreteWeighted::new(&vec![(1, p), (2, p), (3, p), (4, p), (5, p), (6, p)]);
    let mut rand = Xorshift64::seed(19937);
    for _ in 0..TRIALS {
      h[fair_die.sample_1(&mut rand) as usize-1] += 1;
    }
    let lo = (TRIALS as f64 / 6.0 - 1.0 * (TRIALS as f64).sqrt()) as i32; 
    let hi = (TRIALS as f64 / 6.0 + 1.0 * (TRIALS as f64).sqrt()) as i32; 
    for n in h {
      assert!(n > lo && n < hi);      
    }
  }

  #[test]
  fn test_discrete_weighted_xorshift() {
    let mut h = vec![0; 6];
    let mut fair_die = DiscreteWeighted::new(&vec![(1, 0.5), (2, 0.1), (3, 0.1), (4, 0.1), (5, 0.1), (6, 0.1)]);
    let mut rand = Xorshift64::seed(19937);
    for _ in 0..TRIALS {
      h[fair_die.sample_1(&mut rand) as usize-1] += 1;
    }
    println!("{:?}", h);
    let lo = (TRIALS as f64 / 10.0 - 1.0 * (TRIALS as f64).sqrt()) as i32; 
    let hi = (TRIALS as f64 / 10.0 + 1.0 * (TRIALS as f64).sqrt()) as i32; 
    for i in 1..h.len() {
      assert!(h[i] > lo && h[i] < hi);      
    }
  }

  #[test]
  #[should_panic]
  fn test_discrete_weighted_invalid() {
    DiscreteWeighted::new(&vec![]);
  }

  #[test]
  fn test_without_replacement_xorshift() {
    // sample all at once
    {
      let state_occs = (1..=10).map(|i| (i,1)).collect::<Vec<(i32, u32)>>();
      let mut dist = WithoutReplacement::new(&state_occs);
      let mut rng = Xorshift64::seed(19937);
      let mut res = dist.sample_n(state_occs.len(), &mut rng);
      res.sort();
      assert_eq!(res, state_occs.iter().map(|&(v,_)| v).collect::<Vec<i32>>());
    }
    // sample one at a time until exhausted
    {
      let state_occs = (1..=10).map(|i| (i,1)).collect::<Vec<(i32, u32)>>();
      let mut dist = WithoutReplacement::new(&state_occs);
      let mut rng = Xorshift64::seed(19937);
      let mut res = Vec::with_capacity(state_occs.len());
      while !dist.empty() {
        res.push(dist.sample_1(&mut rng));
      }
      res.sort();
      assert_eq!(res, state_occs.iter().map(|&(v,_)| v).collect::<Vec<i32>>());
    }
  }

  #[test]
  #[should_panic]
  fn test_without_replacement_invalid() {
    WithoutReplacement::new(&vec![]);
  }

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
