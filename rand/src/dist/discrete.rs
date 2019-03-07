
extern crate num;
use num::Num;

use crate::dist::*;
use crate::gen::*;

#[derive(Debug)]
// TODO template
pub struct Discrete<T> {
  v: Vec<T>,
}

#[derive(Debug)]
// TODO template?
pub struct DiscreteWeighted<T> {
  v: Vec<T>,
  p: Vec<f64>
}

#[derive(Debug)]
pub struct WithoutReplacement<T> {
  v: Vec<T>,
  f: Vec<u32>
}

impl<T: Num + Clone + Copy> Discrete<T> {
  pub fn new(a: &[T]) -> Discrete<T> {
    assert!(a.len() > 0);
    Discrete{v:a.to_vec()}
  }
}


//  fn sample_n<R: RandomStream + Resettable>(&mut self, n: usize, rng: &mut R) -> f64

impl<T: Num + Clone + Copy> Dist<T> for Discrete<T> {
  fn sample_1<R: RandomStream + Dimensionless>(&mut self, rng: &mut R) -> T {
    let i = rng.next_1() as usize % self.v.len(); 
    self.v[i] // cannot move out of borrowed context without Copy trait bound
  } 

  // TODO remove dimensionless requirement
  fn sample_n<R: RandomStream + Dimensionless>(&mut self, n: usize, rng: &mut R) -> Vec<T> {
    (0..n).map(|_| self.sample_1(rng)).collect()
  } 
}

impl<T: Num + Clone + Copy> DiscreteWeighted<T> {
  pub fn new(a: &[(T,f64)]) -> DiscreteWeighted<T> {
    assert!(a.len() > 0);
    let mut s = 0.0;
    // check probs in [0,1] (dummy sum)
    a.iter().fold(0.0, |_, p| { assert!(p.1 >= 0.0 && p.1 <= 1.0); p.1 } );
    let p = a.iter().fold(Vec::with_capacity(a.len()), |mut acc, p| { s += p.1; acc.push(s); acc });
    // check probabilities sum to unity
    assert!(p.last().unwrap().abs() - 1.0 < std::f64::EPSILON);
    DiscreteWeighted{ v: a.iter().fold(Vec::with_capacity(a.len()), |mut acc, p| { acc.push(p.0); acc }),
              p: p}
  }
}

impl<T: Num + Clone + Copy> Dist<T> for DiscreteWeighted<T> {
  fn sample_1<R: RandomStream + Dimensionless>(&mut self, rng: &mut R) -> T {
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

  fn sample_n<R: RandomStream + Dimensionless>(&mut self, n: usize, rng: &mut R) -> Vec<T> {
    (0..n).map(|_| self.sample_1(rng)).collect()
  } 
}

impl<T: Num + Clone + Copy> WithoutReplacement<T> {
  pub fn new(state_occs: &[(T,u32)]) -> WithoutReplacement<T> {
    assert!(state_occs.len() > 0);
    WithoutReplacement{ v: state_occs.iter().map(|&(v,_)| v).collect(), 
                        f: state_occs.iter().map(|&(_,f)| f).collect() }
  }

  pub fn empty(&self) -> bool {
    self.f.iter().sum::<u32>() == 0
  }
}

impl<T: Num + Clone + Copy> Dist<T> for WithoutReplacement<T> {
  fn sample_1<R: RandomStream + Dimensionless>(&mut self, rng: &mut R) -> T
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

  fn sample_n<R: RandomStream + Dimensionless>(&mut self, n: usize, rng: &mut R) -> Vec<T> {
    (0..n).map(|_| self.sample_1(rng)).collect()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  //use crate::gen::*;

  const TRIALS: usize = 60000;

  #[test]
  fn test_discrete_lcg() {
    let mut h = vec![0; 6];
    let mut die = Discrete::new(&vec![1,2,3,4,5,6]);
    let mut rand = LCG::new(Some(19937));
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
    let v: Vec<i8> = vec![];
    Discrete::new(&v);
  }

  #[test]
  fn test_discrete_xorshift() {
    let mut h = vec![0; 6];
    let mut die = Discrete::new(&vec![1,2,3,4,5,6]);
    let mut rand = Xorshift64::new(Some(19937));
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
    let mut rand = Xorshift64::new(Some(19937));
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
    let mut rand = Xorshift64::new(Some(19937));
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
    let v: Vec<(f64, f64)> = vec![];
    DiscreteWeighted::new(&v);
  }

  #[test]
  #[should_panic]
  fn test_discrete_weighted_invalid2() {
    DiscreteWeighted::new(&vec![(1, 0.0),(2, 1.1),(3, -0.1)]);
  }

  #[test]
  fn test_without_replacement_xorshift() {
    // sample all at once
    {
      let state_occs = (1..=10).map(|i| (i,1)).collect::<Vec<(i32, u32)>>();
      //let state_occs2 = (1..=10).into_iter().zip(&vec![10;1]).collect::<Vec<(i32, u32)>>();
      let mut dist = WithoutReplacement::new(&state_occs);
      let mut rng = Xorshift64::new(Some(19937));
      let mut res = dist.sample_n(state_occs.len(), &mut rng);
      res.sort();
      assert_eq!(res, state_occs.iter().map(|&(v,_)| v).collect::<Vec<i32>>());
    }
    // sample one at a time until exhausted
    {
      let state_occs = (1..=10).map(|i| (i,1)).collect::<Vec<(i32, u32)>>();
      let mut dist = WithoutReplacement::new(&state_occs);
      let mut rng = Xorshift64::new(Some(19937));
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
    let v: Vec<(u32, u32)> = vec![];
    WithoutReplacement::new(&v);
  }
}
