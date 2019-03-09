
/// General traits of random 
pub trait RandomStream {
  /// return n integers (u32)
  fn next_n(&mut self, n: usize) -> Vec<u32>;
  /// return n doubles
  fn uniforms01(&mut self, n: usize) -> Vec<f64>;
}

pub trait Seeded {
  /// initialise from a given seed 
  fn seed(&self) -> u32;
}

// TODO merge with Rejectable?
pub trait Dimensionless {
  /// return n integers (u32)
  fn next_1(&mut self) -> u32;
  /// return n doubles
  fn uniform01(&mut self) -> f64;
}

pub trait Dimensioned {
  /// Get dimension
  fn dim(&self) -> u32;
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
pub mod entropy;