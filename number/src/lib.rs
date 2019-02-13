
// TODO generalise to any signed int
pub fn abs(x : i8) -> Result<i8, String> {
  match x {
    -128 => Err("overflow".to_string()),
    x if x < 0 => Ok(-x),
    _ => Ok(x)
  }
}

#[derive(Copy, Clone, Debug)]
pub enum Number<T> where T: Into<f64> {
  R(T),
  C{ r: T, i: T },
  // TODO +/-inf for comparison and closer to IEEE754
  Inf(bool) // sign bit (true means -ve)
}

impl <T: PartialEq> PartialEq for Number<T> where f64: std::convert::From<T> {
  fn eq(&self, other: &Number<T>) -> bool {
    match (self, other) {
      (&Number::R(ref a), &Number::R(ref b)) => a == b,
      (&Number::C{r: ref ra, i: ref ia}, &Number::C{r: ref rb, i: ref ib}) => ra == rb && ia == ib,
      (&Number::Inf(ref a), &Number::Inf(ref b)) => a == b,
      _ => false,
    }
  }
}


impl<T> Number<T> where T: Into<f64> {
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
  pub fn re(self) -> T {
    match self {
      Number::R(val) => val,
      Number::C{r:r, i:_} => r,
      Number::Inf(_) => panic!("infinite!")
    }
  }
  // pub fn im(self) -> T {
  //   match self {
  //     Number::R(val) => val,
  //     Number::C{r:_,i:_} => panic!("complex!"),
  //     Number::Inf(_) => panic!("infinite!")
  //   }
  // }
}

#[derive(Debug)]
enum FloatingPointError {
  DivZero,
  Overflow,
  InvalidOp,
}

//use std::fmt;
impl std::fmt::Display for FloatingPointError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let msg = match *self {
      FloatingPointError::DivZero => "divide by zero",
      FloatingPointError::Overflow => "overflow",
      FloatingPointError::InvalidOp => "invalid operation"
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

fn f(x: f64, y: f64) -> Result<f64, FloatingPointError> {
  // f(x,y) = sqrt(x)/y
  match (x, y) {
    (x, _) if x < 0.0 => Err(FloatingPointError::InvalidOp),
    (_, y) if y == 0.0 => Err(FloatingPointError::DivZero),
    (_, y) if y.abs() < 1.0e-300 => Err(FloatingPointError::Overflow),
    (x, y) => Ok(x.sqrt() / y)
  }
}

