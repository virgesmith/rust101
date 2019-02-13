
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

// impl PartialEq for Number<T> {
//   fn eq(&self, other: &Number<T>) -> bool {
//     self.isbn == other.isbn
//   }
// }

impl <T: PartialEq> PartialEq for Number<T> where f64: std::convert::From<T> {
  fn eq(&self, other: &Number<T>) -> bool {
    match (self, other) {
      (&Number::R(ref a), &Number::R(ref b)) => a == b,
      //(&R(ref a), &Right(ref b)) => a == b,
      _ => false,
    }
  }
}
impl<T> Number<T> where T: Into<f64> {
  pub fn new(x: T) -> Self {
    Number::R(x)
  }

  // TODO how to overload new?
  // pub fn new(re: T, im: T) -> Self {
  //   Number::C{r:re, i:im}
  // }

  // TODO implicit cast? overload operator "as"
  pub fn r(self) -> T {
    match self {
      Number::R(val) => val,
      Number::C{r:_,i:_} => panic!("complex!"),
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

fn sqrt(x: f64) -> Number<f64> {
  match x {
    x if x < 0.0 => Number::C{r:0.0, i: (-x).sqrt() },
    _ => Number::R(x.sqrt()) // sqrt is a "member" 
  }
}

fn ln(x: f64) -> Number<f64> {
  match x {
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

// fn main() {
//   println!("{:?}", abs(0));
//   println!("{:?}", abs(10));
//   println!("{:?}", abs(-10));
//   println!("{:?}", abs(-128));

//   println!("{:?}", sqrt(-64.));
//   println!("{:?}", sqrt(256.).r());

//   println!("{:?}", ln(-256.));
//   println!("{:?}", ln(0.));
//   println!("{:?}", ln(-0.));
//   println!("{:?}", ln(256.));

//   //let x: f64 = ln(-1.0); // compile-time error!

//   // TODO implicit cast? "as" operator? 
//   let x: f64 = ln(0.1).r();
//   println!("{}", x);
//   //let x: f64 = ln(x).R(); // panics 
//   let z/*: Cplx<f64>*/ = ln(x);
//   println!("{:?}", z);

//   // NOTE ordering: NOT Number::<f64>::R
//   let a = Number::R::<f64>;

//   let _ = a;

//   println!("{:?}", f(1.0, 1.0).unwrap());
//   // println!("{:?}", f(-1.0, 1.0).unwrap());
//   // println!("{:?}", f(1.0, 0.0).unwrap());
//   match f(-1.0, 1.0) {
//     Ok(x) => println!("{}", x),
//     Err(e) => println!("{}", e)
//   }
//   match f(1.0, 0.0) {
//     Ok(x) => println!("{}", x),
//     Err(e) => println!("{}", e)
//   }
//   match f(1.0, 1.0e-308) {
//     Ok(x) => println!("{}", x),
//     Err(e) => println!("{}", e)
//   }

// }
