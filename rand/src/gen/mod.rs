
pub trait Seeded {
  
}

pub trait Dimensioned {

}

pub trait Dimensionless {

}

pub trait Rejectable {
  /// Can be used in rejection sampling
  fn rejectable() -> bool { true }
}

pub mod pseudo;
pub mod quasi;