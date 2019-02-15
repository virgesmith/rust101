
extern crate num;
use num::traits::{Zero, One};
use num::{Signed, Integer, Float, Bounded};

#[derive(Debug, PartialEq)] // PartialEq required to test error values
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
pub enum Number<T> where T: Into<f64> + Float + Copy {
  R(T),
  C{ r: T, i: T },
  Inf(T) // use T's inf...otherwise cant resolve type of T
}

// impl <T: PartialEq> PartialEq for Number<T> where f64: std::convert::From<T>, T: Float { 
//   fn eq(&self, other: &Number<T>) -> bool {
//     match (self, other) {
//       (&Number::R(ref a), &Number::R(ref b)) => a == b,
//       (&Number::R(ref a), &Number::C{r: ref rb, i: ref ib}) => a == rb && &T::zero() == ib,
//       (&Number::C{r: ref ra, i: ref ia}, &Number::R(ref b)) => ra == b && &T::zero() == ia,
//       (&Number::C{r: ref ra, i: ref ia}, &Number::C{r: ref rb, i: ref ib}) => ra == rb && ia == ib,
//       (&Number::Inf(ref a), &Number::Inf(ref b)) => a == b,
//       _ => false,
//     }
//   }
// }

impl <T: PartialEq> PartialEq for Number<T> where f64: std::convert::From<T>, T: Float + Copy {
  fn eq(&self, other: &Number<T>) -> bool {
    match (self, other) {
      (Number::R(a), Number::R(b)) => a == b,
      //                                                    which is best below?
      (Number::R(a), Number::C{r: rb, i: ib}) => a == rb && T::zero() == *ib,
      (Number::C{r: ra, i: ia}, Number::R(b)) => ra == b && &T::zero() == ia,
      (Number::C{r: ra, i: ia}, Number::C{r: rb, i: ib}) => ra == rb && ia == ib,
      (Number::Inf(a), Number::Inf(b)) => a == b,
      _ => false,
    }
  }
}


impl<T> Number<T> where T: Into<f64> + Zero + One + Float {
  // 
  pub fn R(self) -> Result<T, NumericalError> {
    match self {
      Number::R(x) => Ok(x),
      _ => Err(NumericalError::NotRealNumber)
    }
  }

  // convert to (r,i) tuple,
  pub fn C(self) -> Result<(T,T), NumericalError> {
    match self {
      Number::R(x) => Ok((x, T::zero())),
      Number::C{r,i} => Ok((r, i)),
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
    x if x == 0.0 => Number::Inf(std::f64::NEG_INFINITY),
    _ => Number::R(x.ln()) 
  }
}


