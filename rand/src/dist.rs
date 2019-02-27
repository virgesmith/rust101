
#[derive(Debug)]
// TODO template
struct Discrete {
  v: Vec<f64>,
}

#[derive(Debug)]
// TODO template
struct DiscreteWeighted {
  v: Vec<f64>,
  p: Vec<f64>
}

#[derive(Debug)]
struct Uniform {
  l: f64,
  s: f64
}

#[derive(Debug)]
struct Normal {
  mu: f64,
  sigma: f64
}

#[derive(Debug)]
struct Exponential {
  k: f64
}

use crate::gen::Gen;

trait Dist {
  fn sample_1(&self, rng: &mut impl Gen) -> f64;
  fn sample_n(&self, n: usize, rng: &mut impl Gen) -> Vec<f64>;
}


impl Discrete {
  pub fn new(a: &[f64]) -> Discrete {
    Discrete{v:a.to_vec()}
  }
}

impl Dist for Discrete {
  fn sample_1(&self, rng: &mut impl Gen) -> /*T*/ f64 {
    self.v[rng.next_1() as usize % self.v.len()]
  } 

  fn sample_n(&self, n: usize, rng: &mut impl Gen) -> /*T*/ Vec<f64> {
    (0..n).map(|_| self.v[rng.next_1() as usize % self.v.len()]).collect()
  } 
}

impl DiscreteWeighted {
  pub fn new(a: &[(f64,f64)]) -> DiscreteWeighted {
    let mut s = 0.0;
    let p = a.iter().fold(Vec::with_capacity(a.len()), |mut acc, p| { s += p.1; acc.push(s); acc });
    // check probabilities sum to unity 
    assert!(p.last().unwrap().abs() - 1.0 < std::f64::EPSILON);
    DiscreteWeighted{ v: a.iter().fold(Vec::with_capacity(a.len()), |mut acc, p| { acc.push(p.0); acc }),
              p: p}
  }
}

impl Dist for DiscreteWeighted {
  fn sample_1(&self, rng: &mut impl Gen) -> /*T*/ f64 {
    let r = rng.uniform01();
    // first element of p > r
    for i in 0..self.p.len() {
      if self.p[i] >= r {
        return self.v[i];
      } 
    }
    // TODO better way?
    panic!("DiscreteWeighted sample failure, is Generator working correctly?");
  } 

  fn sample_n(&self, n: usize, rng: &mut impl Gen) -> /*T*/ Vec<f64> {
    let mut result = Vec::with_capacity(n);
    for _ in 0..n {
      let r = rng.uniform01();
      // first element of p > r
      for i in 0..self.p.len() {
        if self.p[i] >= r {
          result.push(self.v[i]);
        } 
      }
      // TODO better way?
      panic!("DiscreteWeighted sample failure, is Generator working correctly?");
    }
    result
  } 
}

impl Uniform {
  fn new(l: f64, h: f64) -> Uniform {
    assert!(h > l);
    Uniform{l: l, s: h-l}
  }
}

impl Dist for Uniform {
  fn sample_1(&self, rng: &mut impl Gen) -> /*T*/ f64 {
    rng.uniform01() * self.s + self.l 
  } 

  fn sample_n(&self, n: usize, rng: &mut impl Gen) -> /*T*/ Vec<f64> {
    (0..n).map(|_| rng.uniform01() * self.s + self.l).collect()
  } 
}

impl Exponential {
  fn new(k: f64) -> Exponential {
    assert!(k > 0.0);
    Exponential{k}
  }
}

impl Dist for Exponential {
  fn sample_1(&self, rng: &mut impl Gen) -> /*T*/ f64 {
    -rng.uniform01().ln() / self.k 
  } 

  fn sample_n(&self, n: usize, rng: &mut impl Gen) -> /*T*/ Vec<f64> {
    (0..n).map(|_| -rng.uniform01().ln() / self.k).collect()
  } 
}


mod test {
  use super::*;
  use crate::gen::*;

  const TRIALS: usize = 60000;

  #[test]
  fn test_discrete_lcg() {
    let mut h = vec![0; 6];
    let die = Discrete::new(&vec![1.0,2.0,3.0,4.0,5.0,6.0]);
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
  fn test_discrete_xorshift() {
    let mut h = vec![0; 6];
    let die = Discrete::new(&vec![1.0,2.0,3.0,4.0,5.0,6.0]);
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
    let fair_die = DiscreteWeighted::new(&vec![(1.0, p), (2.0, p), (3.0, p), (4.0, p), (5.0, p), (6.0, p)]);
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
    let fair_die = DiscreteWeighted::new(&vec![(1.0, 0.5), (2.0, 0.1), (3.0, 0.1), (4.0, 0.1), (5.0, 0.1), (6.0, 0.1)]);
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
  fn test_uniform_lcg() {
    let u = Uniform::new(-1.0, 1.0);
    let mut rand = LCG::seed(19937);
    let mu: f64 = u.sample_n(TRIALS, &mut rand).iter().sum::<f64>() / (TRIALS as f64);
    assert!(mu.abs() < (TRIALS as f64).sqrt());
  }

  #[test]
  fn test_uniform_xorshift() {
    let u = Uniform::new(-1.0, 1.0);
    let mut rand = Xorshift64::seed(19937);
    let mu: f64 = u.sample_n(TRIALS, &mut rand).iter().sum::<f64>() / (TRIALS as f64);
    assert!(mu.abs() < (TRIALS as f64).sqrt());
  }

  #[test]
  fn test_exponential_xorshift() {
    // test k from 1e-5 to 1e+5
    for i in -5..6 { 
      let k = 10.0f64.powi(i);
      let e = Exponential::new(k);
      let mut rand = Xorshift64::seed(19937);
      let mu: f64 = e.sample_n(TRIALS, &mut rand).iter().sum::<f64>() / (TRIALS as f64);
      println!("{} {}", mu, 1.0/k);
      // mean should be 1/k
      assert!((mu * k - 1.0).abs() < 1.0 / (TRIALS as f64).sqrt());
    }
  }

}
