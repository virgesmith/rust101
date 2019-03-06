
/// General traits of random 
pub trait RandomStream {
  /// return n integers (u32)
  fn next_n(&mut self, n: usize) -> Vec<u32>;
  /// return n doubles
  fn uniforms01(&mut self, n: usize) -> Vec<f64>;
}

pub trait Seeded {
  /// initialise from a given seed 
  fn new(seed: Option<u32>) -> Self;
}

pub trait Dimensionless {
  /// initialises from a random seed (the time)
  fn new() -> Self;
  /// return n integers (u32)
  fn next_1(&mut self) -> u32;
  /// return n doubles
  fn uniform01(&mut self) -> f64;
}

pub trait Dimensioned {
  /// Init from dimension (conflicts with Dimensioned?)
  fn new(d: u32) -> Self;
}

pub trait Rejectable { 
  /// Can be used in rejection sampling
  fn rejectable() -> bool { true }    
}

pub trait Resettable {
  /// Reset the generator to its initial state
  fn reset(&mut self) -> &mut Self;
  /// Skip n (*dim) values
  fn skip(&mut self, n: u32) -> &mut Self;
}

pub mod pseudo;
pub mod quasi;