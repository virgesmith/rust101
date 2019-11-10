use crate::gen::*;

pub trait Dist<T> {
  fn sample_n(&mut self, n: usize) -> Vec<T>;
}

pub mod discrete;
pub mod continuous;

pub mod normal;
pub mod exponential;

pub mod moments;
pub mod compare;