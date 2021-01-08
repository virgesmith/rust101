extern crate num;
use num::traits::{One, Zero};
use num::{Bounded, Float, Integer, Signed};

use core::ops::Add;

#[derive(Debug, PartialEq)] // PartialEq required to test error values
pub enum NumericalError {
  // hardware FP exceptions
  DivZero,
  Overflow,
  InvalidOp,
  // software exceptions
  NotRealNumber,
  Infinite,
}

pub fn abs<T>(x: T) -> Result<T, NumericalError>
where
  T: Signed + Integer + Zero + Bounded + Copy,
{
  match x {
    // min_value comes from Bounded trait
    x if x == T::min_value() => Err(NumericalError::Overflow),
    x if x < T::zero() => Ok(-x), // E0008: moves value into pattern guard: error fixed by adding copy trait
    _ => Ok(x),
  }
}

#[derive(Copy, Clone, Debug)]
pub enum Number<T> where T: Into<f64> + Float + Copy,
{
  R(T),
  C { r: T, i: T },
  Inf(bool), // use T's inf...otherwise cant resolve type of T. how do we resolve x-Inf?
}


impl<T> Number<T> where T: Into<f64> + Float + Copy,
{
  pub fn from_real(x: T) -> Number<T> {
    // TODO check for inf...
    // match x {
    //   _ => Number::R(x)
    // }
    Number::R(x)
  }

  pub fn from_complex(r: T, i: T) -> Number<T> {
    Number::C{r, i}
  }

  pub fn from_inf(is_neg: bool) -> Number<T> {
    Number::Inf(is_neg)
  }

}

impl<T: PartialEq> PartialEq for Number<T>
where
  f64: std::convert::From<T>,
  T: Float + Copy,
{
  fn eq(&self, other: &Number<T>) -> bool {
    match (*self, *other) {
      (Number::R(ref a), Number::R(ref b)) => a == b,
      (Number::R(ref a), Number::C { r: ref rb, i: ref ib }) => a == rb && &T::zero() == ib,
      (Number::C { r: ref ra, i: ref ia }, Number::R(ref b)) => ra == b && &T::zero() == ia,
      (Number::C { r: ref ra, i: ref ia }, Number::C { r: ref rb, i: ref ib }) => ra == rb && ia == ib,
      (Number::Inf(ref a), Number::Inf(ref b)) => a == b,
      _ => false,
    }
  }
}


impl<T> Add<Number<T>> for Number<T> where T: Into<f64> + Float + Copy {
	type Output = Number<T>;
	fn add(self, rhs: Number<T>) -> Number<T> {
		match (self, rhs) {
      (Number::R(a), Number::R(b)) => Number::R(a+b),
      (Number::R(a), Number::C{r:rb, i:ib}) => Number::C{r:a+rb, i:ib},
      (Number::C{r:ra, i:ia}, Number::R(b)) => Number::C{r:ra+b, i:ia},
      (Number::C{r:ra, i:ia}, Number::C{r:rb, i:ib}) => Number::C{r:ra+rb, i:ia+ib},
      _ => Number::Inf(false)
    }
	}
}



impl<T> Number<T>
where
  T: Into<f64> + Zero + One + Float,
{
  //
  #[allow(non_snake_case)]
  pub fn R(self) -> Result<T, NumericalError> {
    match self {
      Number::R(x) => Ok(x),
      _ => Err(NumericalError::NotRealNumber),
    }
  }

  // convert to (r,i) tuple,
  #[allow(non_snake_case)]
  pub fn C(self) -> Result<(T, T), NumericalError> {
    match self {
      Number::R(x) => Ok((x, T::zero())),
      Number::C { r, i } => Ok((r, i)),
      _ => Err(NumericalError::Infinite),
    }
  }

  pub fn re(self) -> T {
    match self {
      Number::R(val) => val,
      Number::C { r, i: _ } => r,
      Number::Inf(_) => panic!("infinite!"),
    }
  }

  pub fn im(self) -> T {
    match self {
      Number::R(_) => T::zero(),
      Number::C { r: _, i } => i,
      Number::Inf(_) => panic!("infinite!"),
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
      NumericalError::Infinite => "infinity",
    };
    write!(f, "{}", msg)
  }
}

pub fn sqrt(x: f64) -> Number<f64> {
  match x {
    x if x < 0.0 => Number::C { r: 0.0, i: (-x).sqrt() },
    _ => Number::R(x.sqrt()), // sqrt is a "member"
  }
}

pub fn ln(x: f64) -> Number<f64> {
  match x {
    // only does the root in [0,pi]
    x if x < 0.0 => Number::C {
      r: (-x).ln(),
      i: std::f64::consts::PI,
    },
    x if x == 0.0 => Number::Inf(true),
    _ => Number::R(x.ln()),
  }
}

// impl Div for Number<T> {
//   fn div()
// }

#[cfg(test)]

mod tests {
  use super::Number::*;
  use super::*;

  fn f(x: f64, y: f64) -> Result<f64, NumericalError> {
    // f(x,y) = sqrt(x)/y
    match (x, y) {
      (x, _) if x < 0.0 => Err(NumericalError::InvalidOp),
      (_, y) if y == 0.0 => Err(NumericalError::DivZero),
      (_, y) if y.abs() < 1.0e-300 => Err(NumericalError::Overflow),
      (x, y) => Ok(x.sqrt() / y),
    }
  }

  #[test]
  fn test_eq() {
    let r = R(1.0);
    assert_eq!(r, R(1.0));
    assert_eq!(r, C { r: 1.0, i: 0.0 });
    assert_eq!(C { r: 1.0, i: 0.0 }, r);
    assert_ne!(r, C { r: 1.0, i: 0.1 });
    assert_ne!(C { r: 1.1, i: 0.0 }, r);
    assert_eq!(Inf(std::f64::INFINITY), Inf(std::f64::INFINITY));
    assert_eq!(Inf(std::f64::NEG_INFINITY), Inf(std::f64::NEG_INFINITY));
    assert_ne!(Inf(std::f64::INFINITY), Inf(std::f64::NEG_INFINITY));
    assert_ne!(Inf(std::f64::NEG_INFINITY), Inf(std::f64::INFINITY));
  }

  #[test]
  fn integer_abs() {
    assert_eq!(abs(0i8).unwrap(), 0);
    assert_eq!(abs(100i8).unwrap(), 100);
    assert_eq!(abs(-100i8).unwrap(), 100);
    assert!(abs(-128i8).is_err());

    assert_eq!(abs(0i16).unwrap(), 0);
    assert_eq!(abs(100i16).unwrap(), 100);
    assert_eq!(abs(-100i16).unwrap(), 100);
    assert!(abs(-32768i16).is_err());

    assert_eq!(abs(0i32).unwrap(), 0);
    assert_eq!(abs(100i32).unwrap(), 100);
    assert_eq!(abs(-100i32).unwrap(), 100);
    assert!(abs(-2147483648i32).is_err());
  }

  #[test]
  fn test() {
    assert_eq!(R(8.0).re(), 8.0);
    assert_eq!(R(8.0).im(), 0.0);
    assert_eq!(C { r: 0.0, i: 8.0 }.re(), 0.0);
    assert_eq!(C { r: 0.0, i: 8.0 }.im(), 8.0);

    assert_eq!(sqrt(64.), R(8.0));
    assert_eq!(sqrt(-64.), C { r: 0.0, i: 8.0 });

    assert_eq!(ln(1.0), R(0.0));
    assert_eq!(ln(0.0), Inf(std::f64::NEG_INFINITY));
    assert_eq!(ln(std::f64::consts::E), R(1.0));
    assert_eq!(
      ln(-std::f64::consts::E),
      C {
        r: 1.0,
        i: std::f64::consts::PI
      }
    );

    //let x: f64 = ln(-1.0); // compile-time error!

    let x: f64 = ln(0.1).re();
    assert_eq!(x, 0.1f64.ln());
    assert!(ln(x).R().is_err());
    assert!(!ln(x).C().is_err());

    assert_eq!(f(0.0, 1.0), Ok(0.0));
    assert_eq!(f(-1.0, 1.0), Err(NumericalError::InvalidOp));
    assert_eq!(f(1.0, 0.0), Err(NumericalError::DivZero));
    assert_eq!(f(1.0, 1.0e-308), Err(NumericalError::Overflow));
  }
}
