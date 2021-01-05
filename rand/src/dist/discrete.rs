
extern crate num;
use num::Num;

use crate::dist::*;

#[derive(Debug)]
pub struct Discrete<R, T> {
  v: Vec<T>,
  rng: R
}

#[derive(Debug)]
pub struct DiscreteWeighted<R, T> {
  v: Vec<T>,
  p: Vec<f64>,
  rng: R
}

#[derive(Debug)]
pub struct WithoutReplacement<R, T> {
  v: Vec<T>,
  f: Vec<u32>,
  rng: R
}

impl<R: RandomStream, T: Num + Clone + Copy> Discrete<R, T> {
  pub fn new(a: &[T], rng: R) -> Discrete<R, T> {
    assert!(!a.is_empty());
    Discrete{v:a.to_vec(), rng}
  }
}

impl<R: RandomStream, T: Num + Clone + Copy> Dist<T> for Discrete<R, T> {
  // fn sample_1<R: RandomStream + Dimensionless>(&mut self, rng: &mut R) -> T {
  //   let i = rng.next_1() as usize % self.v.len();
  //   self.v[i] // cannot move out of borrowed context without Copy trait bound
  // }

  fn sample_n(&mut self, n: usize) -> Vec<T> {
    self.rng.next_n(n).iter().map(|&r| self.v[r as usize % self.v.len()]).collect()
  }
}

// bisect to find lowest i where v[i] <= x
fn bisect<T: PartialOrd>(x: T, v: &[T]) -> usize {
  // let mut l = 0;
  // let mut h = v.len() - 1;
  // if v[0] >= x { return 0; }
  // loop {
  //   let m = (l + h) / 2;
  //   if v[m] <= x { l = m; } else { h = m; }
  //   // return h when l,h adjacent
  //   if h - l <= 1 {
  //     return h;
  //   }
  // }
  // sequential version: return i where v[i] <= x < v[i+1]
  for (i, v) in v.iter().enumerate() {
    if &x <= v {
      return i;
    }
  }
  panic!("DiscreteWeighted sample failure, is Generator working correctly?");
}

impl<R: RandomStream, T: Num + Clone + Copy> DiscreteWeighted<R, T> {
  pub fn new(a: &[(T,f64)], rng: R) -> DiscreteWeighted<R, T> {
    assert!(!a.is_empty());
    let mut s = 0.0;
    // check probs in [0,1] (dummy sum)
    a.iter().fold(0.0, |_, p| { assert!(p.1 >= 0.0 && p.1 <= 1.0); p.1 } );
    let p = a.iter().fold(Vec::with_capacity(a.len()), |mut acc, p| { s += p.1; acc.push(s); acc });
    // check probabilities sum to unity
    assert!(p.last().unwrap().abs() - 1.0 < std::f64::EPSILON);
    DiscreteWeighted{ v: a.iter().fold(Vec::with_capacity(a.len()), |mut acc, p| { acc.push(p.0); acc }),
              p, rng }
  }

  fn sample_1(&mut self, r: f64) -> T {
    // first element in p > r
    self.v[bisect(r, &self.p)]
  }

}

impl<R: RandomStream, T: Num + Clone + Copy> Dist<T> for DiscreteWeighted<R, T> {
  fn sample_n(&mut self, n: usize) -> Vec<T> {
    self.rng.uniforms01(n).iter().map(|&r| self.sample_1(r)).collect()
  }
}

impl<R: RandomStream, T: Num + Clone + Copy> WithoutReplacement<R, T> {
  pub fn new(state_occs: &[(T,u32)], rng: R) -> WithoutReplacement<R, T> {
    assert!(!state_occs.is_empty());
    WithoutReplacement{ v: state_occs.iter().map(|&(v,_)| v).collect(),
                        f: state_occs.iter().map(|&(_,f)| f).collect(),
                        rng }
  }

  fn sample_1(&mut self, r: u32) -> T
  {
    let mut s = 0;
    let cumul = self.f.iter().fold(Vec::with_capacity(self.f.len()), |mut acc, f| { s += f; acc.push(s); acc });
    let r = r % cumul.last().unwrap();
    // broken: samples empty states
    // let i = bisect(r, &self.f);
    // self.f[i] -= 1;
    // self.v[i]
    for (i, c) in cumul.iter().enumerate() {
      if c > &r {
        self.f[i] -= 1;
        return self.v[i];
      }
    }
    // TODO better way?
    panic!("WithoutReplacement sample failure, is Generator working correctly?");
  }

  pub fn empty(&self) -> bool {
    self.f.iter().sum::<u32>() == 0
  }
}

impl<R: RandomStream, T: Num + Clone + Copy> Dist<T> for WithoutReplacement<R, T> {
  fn sample_n(&mut self, n: usize) -> Vec<T> {
    // ensure there's enough populatio left
    assert!(n as u32 <= self.f.iter().sum::<u32>());
    self.rng.next_n(n).iter().map(|&r| self.sample_1(r)).collect()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::gen::pseudo::*;
  use crate::gen::quasi::*;

  const TRIALS: usize = 60000;

  #[test]
  fn test_discrete_lcg() {
    let mut h = vec![0; 6];
    let mut die = Discrete::new(&vec![1,2,3,4,5,6], LCG::new(Some(19937)));
    let r = die.sample_n(TRIALS);
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
    Discrete::new(&v, LCG::new(None));
  }

  #[test]
  fn test_discrete_xorshift() {
    let mut h = vec![0; 6];
    let mut die = Discrete::new(&vec![1,2,3,4,5,6], Xorshift64::new(Some(19937)));
    for _ in 0..TRIALS {
      h[die.sample_n(1)[0] as usize-1] += 1;
    }
    let lo = (TRIALS as f64 / 6.0 - 1.0 * (TRIALS as f64).sqrt()) as i32;
    let hi = (TRIALS as f64 / 6.0 + 1.0 * (TRIALS as f64).sqrt()) as i32;
    for n in h {
      assert!(n > lo && n < hi);
    }
  }

  #[test]
  fn test_bisect() {
    let v = vec![0.0, 0.5, 1.5, 2.5, 4.0, 5.0];
    assert_eq!(bisect(0.0, &v), 0);
    assert_eq!(bisect(1.0, &v), 2);
    assert_eq!(bisect(2.0, &v), 3);
    assert_eq!(bisect(3.0, &v), 4);
    assert_eq!(bisect(4.0, &v), 4);
    assert_eq!(bisect(5.0, &v), 5);
  }

  #[test]
  fn test_discrete_flat_weighted_xorshift() {
    let mut h = vec![0; 6];
    let p = 1.0 / 6.0;
    let mut fair_die = DiscreteWeighted::new(&vec![(1, p), (2, p), (3, p), (4, p), (5, p), (6, p)], Xorshift64::new(Some(19937)));
    for _ in 0..TRIALS {
      h[fair_die.sample_n(1)[0] as usize-1] += 1;
    }
    println!("{:?}", h);
    let lo = (TRIALS as f64 / 6.0 - 1.0 * (TRIALS as f64).sqrt()) as i32;
    let hi = (TRIALS as f64 / 6.0 + 1.0 * (TRIALS as f64).sqrt()) as i32;
    for n in h {
      assert!(n > lo && n < hi);
    }
  }

  #[test]
  fn test_discrete_weighted_xorshift() {
    let mut h = vec![0; 6];
    let mut fair_die = DiscreteWeighted::new(&vec![(1, 0.5), (2, 0.1), (3, 0.1), (4, 0.1), (5, 0.1), (6, 0.1)], Xorshift64::new(Some(19937)));
    for _ in 0..TRIALS {
      h[fair_die.sample_n(1)[0] as usize-1] += 1;
    }
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
    DiscreteWeighted::new(&v, LCG::new(None));
  }

  #[test]
  #[should_panic]
  fn test_discrete_weighted_invalid2() {
    DiscreteWeighted::new(&vec![(1, 0.0),(2, 1.1),(3, -0.1)], LCG::new(None));
  }

  #[test]
  fn test_without_replacement_xorshift() {
    // sample all at once
    {
      let state_occs = (1..=10).map(|i| (i,1)).collect::<Vec<(i32, u32)>>();
      //let state_occs2 = (1..=10).into_iter().zip(&vec![10;1]).collect::<Vec<(i32, u32)>>();
      let rng = Xorshift64::new(Some(19937));
      // compiler doesnt complain as rng is moved
      let mut dist = WithoutReplacement::new(&state_occs, rng);
      // but rng can't be used now
      // rng.next_1();
      // ^^^ value borrowed here after move
      let mut res = dist.sample_n(state_occs.len());
      res.sort();
      assert_eq!(res, state_occs.iter().map(|&(v,_)| v).collect::<Vec<i32>>());
    }
    // sample one at a time until exhausted
    {
      let state_occs = (1..=10).map(|i| (i,1)).collect::<Vec<(i32, u32)>>();
      let mut dist = WithoutReplacement::new(&state_occs, Xorshift64::new(Some(19937)));
      let mut res = Vec::with_capacity(state_occs.len());
      while !dist.empty() {
        res.push(dist.sample_n(1)[0]);
      }
      res.sort();
      assert_eq!(res, state_occs.iter().map(|&(v,_)| v).collect::<Vec<i32>>());
    }
  }

  #[test]
  fn test_without_replacement_sobol() {
    // state i has occupacy i
    let state_occs = (1..=10).map(|i| (i,i as u32)).collect::<Vec<(i32, u32)>>();
    // one sample from 55-dimensional sobol should take the entire population
    let mut dist = WithoutReplacement::new(&state_occs, Sobol::new(55));
    let res = dist.sample_n(55);
    // reconstruct the dist
    let hist = res.iter().fold(vec![0u32;10],
                                |mut acc, &v| { let i = (v-1) as usize; acc[i] += 1; acc })
                          .into_iter().collect::<Vec<u32>>();
    println!("{:?}", hist);
    assert_eq!(hist, (1..=10).collect::<Vec<u32>>());
  }

  #[test]
  #[should_panic]
  fn test_without_replacement_invalid() {
    let v: Vec<(u32, u32)> = vec![];
    WithoutReplacement::new(&v, LCG::new(None));
  }
}
