
extern crate num;
use num::traits::{Zero, One};
use num::{Signed, Integer, Float, Bounded};

#[derive(Debug)]
pub enum NumericalError {
  // hardware FP exceptions
  DivZero,
  Overflow,
  InvalidOp,
  // software exceptions
  NotRealNumber,
  Infinite
}

pub fn abs<T>(x : T) -> Result<T, NumericalError> 
  where T: Signed + Integer + Zero + Bounded + Copy {
  match x {
    // min_value comes from Bounded trait
    x if x == T::min_value() => Err(NumericalError::Overflow),
    x if x < T::zero() => Ok(-x), // E0008: moves value into pattern guard: error fixed by adding copy trait
    _ => Ok(x)
  }
}

#[derive(Copy, Clone, Debug)]
pub enum Number<T> where T: Into<f64> + Float {
  R(T),
  C{ r: T, i: T },
  // TODO +/-inf for comparison and closer to IEEE754
  Inf(bool) // sign bit (true means -ve)
}

impl <T: PartialEq> PartialEq for Number<T> where f64: std::convert::From<T>, T: Float {
  fn eq(&self, other: &Number<T>) -> bool {
    match (self, other) {
      (&Number::R(ref a), &Number::R(ref b)) => a == b,
      (&Number::C{r: ref ra, i: ref ia}, &Number::C{r: ref rb, i: ref ib}) => ra == rb && ia == ib,
      (&Number::Inf(ref a), &Number::Inf(ref b)) => a == b,
      _ => false,
    }
  }
}


impl<T> Number<T> where T: Into<f64> + Zero + One + Float {
  // pointless...
  // pub fn real(x: T) -> Self {
  //   Number::R(x)
  // }
  // pub fn complex(r:T, i: T) -> Self {
  //   Number::C{r:r, i:i}
  // }
  // pub fn inf(sign: bool) -> Self {
  //   Number::Inf(sign)
  // }

  // TODO how to overload new?
  // pub fn new(re: T, im: T) -> Self {
  //   Number::C{r:re, i:im}
  // }

  // TODO implicit cast? overload operator "as"

  // 
  pub fn R(self) -> Result<T, NumericalError> {
    match self {
      Number::R(x) => Ok(x),
      _ => Err(NumericalError::NotRealNumber)
    }
  }

  pub fn C(self) -> Result<Number<T>, NumericalError> {
    match self {
      Number::R(x) => Ok(Number::C{r:x, i:T::zero()}),
      Number::C{r,i} => Ok(Number::C{r:r, i:i}),
      _ => Err(NumericalError::Infinite)
    }
  }

  pub fn re(self) -> T {
    match self {
      Number::R(val) => val,
      Number::C{r, i:_} => r,
      Number::Inf(_) => panic!("infinite!")
    }
  }

  pub fn im(self) -> T {
    match self {
      Number::R(val) => T::zero(),
      Number::C{r:_,i} => i,
      Number::Inf(_) => panic!("infinite!")
    }
  }
}

//use std::fmt;
impl std::fmt::Display for NumericalError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let msg = match *self {
      NumericalError::DivZero => "divide by zero",
      NumericalError::Overflow => "overflow",
      NumericalError::InvalidOp => "invalid operation",
      NumericalError::NotRealNumber => "not a real number",
      NumericalError::Infinite => "infinity"
    };
    write!(f, "{}", msg)
  }
}

pub fn sqrt(x: f64) -> Number<f64> {
  match x {
    x if x < 0.0 => Number::C{r:0.0, i: (-x).sqrt() },
    _ => Number::R(x.sqrt()) // sqrt is a "member" 
  }
}

pub fn ln(x: f64) -> Number<f64> {
  match x {
    // only does the root in [0,pi]
    x if x < 0.0 => Number::C{ r: (-x).ln(), i: std::f64::consts::PI },
    x if x == 0.0 => Number::Inf(true),
    _ => Number::R(x.ln()) 
  }
}

fn f(x: f64, y: f64) -> Result<f64, NumericalError> {
  // f(x,y) = sqrt(x)/y
  match (x, y) {
    (x, _) if x < 0.0 => Err(NumericalError::InvalidOp),
    (_, y) if y == 0.0 => Err(NumericalError::DivZero),
    (_, y) if y.abs() < 1.0e-300 => Err(NumericalError::Overflow),
    (x, y) => Ok(x.sqrt() / y)
  }
}

