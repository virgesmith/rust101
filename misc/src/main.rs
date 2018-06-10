
//use std::num;

// fn first_word(s: &String) -> &str {
//   let bytes = s.as_bytes();
//   for (i, &item) in bytes.iter().enumerate() {
//     if item == b' ' {
//       return &s[0..i];
//     }
//   }
//   &s[..]
// }

fn abs(x : i8) -> Result<i8, String> {
  match x {
    -128 => Err("overflow".to_string()),
    x if x < 0 => Ok(-x),
    _ => Ok(x)
  }
}

//struct c64 { r: f64, i: f64 }

#[derive(Debug)]
enum Number<T> {
  R(T),
  C(T, T),
  Inf()
}

fn sqrt(x: f64) -> Number<f64> {
  match x {
    x if x < 0.0 => Number::C(0.0, (-x).sqrt()),
    _ => Number::R(x.sqrt()) // sqrt is a "member" weird!
  }
}

fn ln(x: f64) -> Number<f64> {
  match x {
    x if x < 0.0 => Number::C((-x).ln(), std::f64::consts::PI),
    x if x == 0.0 => Number::Inf(),
    _ => Number::R(x.ln()) // weird!
  }
}

fn main() {
  let mut a = [1,2,3,4,5,6,7,8];

  let mut s = &a[2..5];

  //s[0] = 9;
  //s.clear(); // Error!
  println!("{:?}", a);
  println!("{:?}", abs(0));
  println!("{:?}", abs(10));
  println!("{:?}", abs(-10));
  println!("{:?}", abs(-128));

  println!("{:?}", sqrt(-64.));
  println!("{:?}", sqrt(256.));

  println!("{:?}", ln(-256.));
  println!("{:?}", ln(0.));
  println!("{:?}", ln(-0.));
  println!("{:?}", ln(256.));
}