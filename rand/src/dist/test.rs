// test stuff
use crate::gen::*;
use crate::gen::pseudo::*;
use crate::gen::quasi::*;

// non-rejection sampler
struct NoRej<R> 
{ 
  rng: R
}

impl<R: RandomStream + Dimensioned> NoRej<R> {
  fn new(n: u32) -> NoRej<R> {
    NoRej{ rng: R::new(n) }
  }
  fn get_n(&mut self, n: usize) -> Vec<f64> {
    self.rng.uniforms01(n)
  }
}

// rejection sampler. no Sobol allowed
struct Rej<R> 
{ 
  rng: R
}

impl<R: RandomStream + Dimensionless + Seeded> Rej<R> {
  fn new(n: u32) -> Rej<R> {
    Rej{ rng: R::new(Some(n)) }
  }
  pub fn get_n(&mut self, n: usize) -> Vec<f64> {
    self.rng.uniforms01(n)
  }
}

pub struct QNormal<T> {
  sampler: T
}

impl<R: RandomStream + Dimensioned> QNormal<NoRej<R>> {
  pub fn new(n: u32) -> QNormal<NoRej<R>> {
    QNormal{ sampler: NoRej::<R>::new(n) }
  }

  fn sample_n(&mut self, n: usize) -> Vec<f64>
  {
    self.sampler.get_n(n)
  }
}

impl<R: RandomStream + Dimensionless + Seeded> QNormal<Rej<R>> {
  pub fn new(n: u32) -> QNormal<Rej<R>> {
    QNormal{ sampler: Rej::<R>::new(n) }
  }

  fn sample_n(&mut self, n: usize) -> Vec<f64>
  {
    self.sampler.get_n(n)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn tdd() {
    let mut d0 = QNormal::<Rej<MT19937>>::new(1);
    let mut d1 = QNormal::<NoRej<Sobol>>::new(8);
    //let mut no = QNormal::<Rej<Sobol>>::new(8);
    println!("{:?}", d0.sample_n(10));
    println!("{:?}", d1.sample_n(8));
    println!("{:?}", d1.sample_n(8));
    //assert!(false);
    // not allowed
  }
}