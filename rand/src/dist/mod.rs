use crate::gen::pseudo::*;
use crate::gen::quasi::*;

pub trait Dist<T> {
  fn sample_1(&mut self, rng: &mut impl PRNG) -> T;
  fn sample_n(&mut self, n: usize, rng: &mut impl PRNG) -> Vec<T>;
}

pub mod discrete;
pub mod continuous;
pub mod normal;