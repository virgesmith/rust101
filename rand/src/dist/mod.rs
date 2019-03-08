use crate::gen::*;
// use crate::gen::pseudo::*;
// use crate::gen::quasi::*;

pub trait Dist<T> {
  fn sample_n(&mut self, n: usize) -> Vec<T>;
}

pub mod discrete;
pub mod continuous;
pub mod normal;
