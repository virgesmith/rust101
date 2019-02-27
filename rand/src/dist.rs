
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

use crate::gen::Gen;

trait Dist {
  fn sample(&self, rng: &mut impl Gen) -> f64;
}


impl Discrete {
  pub fn new(a: &[f64]) -> Discrete {
    Discrete{v:a.to_vec()}
  }
}

impl Dist for Discrete {
  fn sample(&self, rng: &mut impl Gen) -> /*T*/ f64 {
    let bucket = rng.next() as usize % self.v.len();
    self.v[bucket]
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
  fn sample(&self, rng: &mut impl Gen) -> /*T*/ f64 {
    let r = rng.next01();
    // first element of p > r
    for i in 0..self.p.len() {
      if self.p[i] >= r {
        return self.v[i];
      } 
    }
    // TODO better way?
    panic!("DiscreteWeighted sample failure, is Generator working correctly?");
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
    for _ in 0..60000 {
      h[die.sample(&mut rand) as usize-1] += 1;
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
      h[die.sample(&mut rand) as usize-1] += 1;
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
      h[fair_die.sample(&mut rand) as usize-1] += 1;
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
      h[fair_die.sample(&mut rand) as usize-1] += 1;
    }
    println!("{:?}", h);
    let lo = (TRIALS as f64 / 10.0 - 1.0 * (TRIALS as f64).sqrt()) as i32; 
    let hi = (TRIALS as f64 / 10.0 + 1.0 * (TRIALS as f64).sqrt()) as i32; 
    for i in 1..h.len() {
      assert!(h[i] > lo && h[i] < hi);      
    }
  }
}
