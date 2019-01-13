

fn abs(x : i8) -> Result<i8, String> {
  match x {
    -128 => Err("overflow".to_string()),
    x if x < 0 => Ok(-x),
    _ => Ok(x)
  }
}

#[derive(Debug)]
pub struct Cplx<T> { r: T, i: T }

#[derive(Debug)]
pub enum Number<T> /*where T: Into<f64>*/ {
  R(T),
  C(Cplx<T>),
  // TODO +/-inf for comparison and closer to IEEE754
  Inf(bool) // sign bit (true means -ve)
}

impl<T> Number<T> {
  pub fn r(self) -> T {
    match self {
      Number::R(val) => val,
      Number::C(_) => panic!("complex!"),
      Number::Inf(_) => panic!("infinite!")
    }
  }
  // // can we overload?
  // pub fn cunwrap(self) -> Cplx<T> {
  //   match self {
  //     Number::R(val) => Cplx{r: val, i: 0.0},
  //     Number::C(cval) => cval,
  //     Number::Inf(_) => panic!("infinite!")
  //   }
  // }
}

use Number::R;

fn sqrt(x: f64) -> Number<f64> {
  match x {
    x if x < 0.0 => Number::C(Cplx{r:0.0, i: (-x).sqrt() }),
    _ => Number::R(x.sqrt()) // sqrt is a "member" 
  }
}

fn ln(x: f64) -> Number<f64> {
  match x {
    x if x < 0.0 => Number::C(Cplx{ r: (-x).ln(), i: std::f64::consts::PI }),
    x if x == 0.0 => Number::Inf(true),
    _ => Number::R(x.ln()) 
  }
}

fn main() {
  println!("{:?}", abs(0));
  println!("{:?}", abs(10));
  println!("{:?}", abs(-10));
  println!("{:?}", abs(-128));

  println!("{:?}", sqrt(-64.));
  println!("{:?}", sqrt(256.).r());

  println!("{:?}", ln(-256.));
  println!("{:?}", ln(0.));
  println!("{:?}", ln(-0.));
  println!("{:?}", ln(256.));

  //let x: f64 = ln(-1.0); // compile-time error!

  let x: f64 = ln(0.1).r();
  println!("{}", x);
  //let x: f64 = ln(x).R(); // panics 
  let z/*: Cplx<f64>*/ = ln(x);
  println!("{:?}", z);

  let y = Number<f64>::R;
}
