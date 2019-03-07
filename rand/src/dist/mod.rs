use crate::gen::*;
use crate::gen::pseudo::*;
use crate::gen::quasi::*;

pub trait Dist<T> {
  fn sample_1<R: RandomStream + Dimensionless>(&mut self, rng: &mut R) -> T;
  // TODO remove Dimensionless trait bound
  fn sample_n<R: RandomStream + Dimensionless>(&mut self, n: usize, rng: &mut R) -> Vec<T>;
}

pub mod discrete;
pub mod continuous;
pub mod normal;