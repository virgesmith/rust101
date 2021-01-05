use std::ops::{Add, Div, Mul, Neg, Sub};
use std::fmt;
use num_traits::Float;

#[derive(Debug, Clone, Copy)]
pub struct Cplx<T>
where
  T: Into<f64> + Float + Copy,
{
  r: T,
  i: T,
}

impl<T> Cplx<T>
where
  T: Into<f64> + Float + Copy,
{
  pub fn from_real(x: T) -> Cplx<T> {
    Cplx { r: x, i: T::zero() }
  }

  pub fn new(r: T, i: T) -> Cplx<T> {
    Cplx { r, i }
  }

  pub fn from_normarg(r: T, theta: T) -> Cplx<T> {
    Cplx {
      r: r * theta.cos(),
      i: r * theta.sin(),
    }
  }

  pub fn re(&self) -> T {
    self.r
  }

  pub fn im(&self) -> T {
    self.i
  }

  pub fn conj(&self) -> Cplx<T> {
    Cplx {
      r: self.r,
      i: -self.i,
    }
  }

  pub fn norm(&self) -> T {
    (self.i * self.i + self.r * self.r).sqrt()
  }

  pub fn arg(&self) -> T {
    self.i.atan2(self.r)
  }

  pub fn sqrt(&self) -> Cplx<T> {
    let a = self.norm().sqrt();
    let theta = self.arg() * T::from(0.5).unwrap();
    Cplx::<T>::from_normarg(a, theta)
  }

  pub fn exp(&self) -> Cplx<T> {
    Cplx::from_normarg(self.r.exp(), self.i)
  }

  pub fn ln(&self) -> Cplx<T> {
    Cplx {
      r: self.norm().ln(),
      i: self.arg(),
    }
  }

  // since we cant overload to enable expressions like 1.0 / z
  pub fn recip(&self) -> Cplx<T> {
    Cplx::from_normarg(T::one() / self.norm(), -self.arg())
  }
}

// convenience non-member wrappers - sqrt(z) instead of z.sqrt()
pub fn sqrt<T>(z: Cplx<T>) -> Cplx<T> where T: Into<f64> + Float + Copy {
  z.sqrt()
}

impl<T> PartialEq for Cplx<T> where T: Into<f64> + Float + Copy {
  fn eq(&self, rhs: &Self) -> bool {
    self.r == rhs.r && self.i == rhs.i
  }
}

impl fmt::Display for Cplx<f64> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}{:+}i)", self.r, self.i)
  }
}

// Implicit cast from real to complex
impl<T> From<T> for Cplx<T>
where
  T: Into<f64> + Float + Copy
{
  fn from(x: T) -> Cplx<T> {
    Cplx { r: x, i: T::zero() }
  }
}

impl<T> Add<Cplx<T>> for Cplx<T>
where
  T: Into<f64> + Float + Copy,
{
  type Output = Cplx<T>;
  fn add(self, rhs: Cplx<T>) -> Cplx<T> {
    Cplx {
      r: self.r + rhs.r,
      i: self.i + rhs.i,
    }
  }
}

impl<T> Add<T> for Cplx<T>
where
  T: Into<f64> + Float + Copy,
{
  type Output = Cplx<T>;
  fn add(self, rhs: T) -> Cplx<T> {
    Cplx {
      r: self.r + rhs,
      i: self.i,
    }
  }
}

impl<T> Sub<Cplx<T>> for Cplx<T>
where
  T: Into<f64> + Float + Copy,
{
  type Output = Cplx<T>;
  fn sub(self, rhs: Cplx<T>) -> Cplx<T> {
    Cplx {
      r: self.r - rhs.r,
      i: self.i - rhs.i,
    }
  }
}

impl<T> Sub<T> for Cplx<T>
where
  T: Into<f64> + Float + Copy,
{
  type Output = Cplx<T>;
  fn sub(self, rhs: T) -> Cplx<T> {
    Cplx {
      r: self.r - rhs,
      i: self.i,
    }
  }
}

impl<T> Mul<Cplx<T>> for Cplx<T>
where
  T: Into<f64> + Float + Copy,
{
  type Output = Cplx<T>;
  fn mul(self, rhs: Cplx<T>) -> Cplx<T> {
    Cplx {
      r: self.r * rhs.r - self.i * rhs.i,
      i: self.r * rhs.i + self.i * rhs.r,
    }
  }
}

impl<T> Mul<T> for Cplx<T>
where
  T: Into<f64> + Float + Copy,
{
  type Output = Cplx<T>;
  fn mul(self, rhs: T) -> Cplx<T> {
    Cplx {
      r: self.r * rhs,
      i: self.i * rhs,
    }
  }
}

impl<T> Div<Cplx<T>> for Cplx<T>
where
  T: Into<f64> + Float + Copy,
{
  type Output = Cplx<T>;
  fn div(self, rhs: Cplx<T>) -> Cplx<T> {
    Cplx::from_normarg(self.norm() / rhs.norm(), self.arg() - rhs.arg())
  }
}

impl<T> Div<T> for Cplx<T>
where
  T: Into<f64> + Float + Copy,
{
  type Output = Cplx<T>;
  fn div(self, rhs: T) -> Cplx<T> {
    Cplx {
      r: self.r / rhs,
      i: self.i / rhs,
    }
  }
}

impl<T> Neg for Cplx<T>
where
  T: Into<f64> + Float + Copy,
{
  type Output = Cplx<T>;
  fn neg(self) -> Cplx<T> {
    Cplx {
      r: -self.r,
      i: -self.i,
    }
  }
}

// relative comparision switching to absolute below the tolerance threshhold
// symmetric so compare(x,y) always same as compare(y,x)
// handles cases where x and y are not same sign
pub fn compare(lhs: f64, rhs: f64, reltol: Option<f64>) -> bool {
  let eps = match reltol {
    Some(x) if x > 0.0 => x,
    Some(_) => panic!("compare tolerance must be strictly greater than zero"),
    None => std::f64::EPSILON
  };

  let diff = (lhs - rhs).abs();
  let mid = 0.5 * (lhs + rhs).abs();

  match mid {
    z if z < eps => diff < eps,
    _ => diff < eps * mid
  }
}

pub fn compare_z(z0: Cplx<f64>, z1: Cplx<f64>) -> bool {
  compare(z0.r, z1.r, None) && compare(z0.i, z1.i, None)
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn basics() {
    let i = Cplx::new(0.0, 1.0);
    assert_eq!(i.r, 0.0);
    assert_eq!(i.i, 1.0);
    let j = Cplx::from_normarg(5.0, (3.0/4.0).atan());
    assert_eq!(j.r, 4.0);
    assert_eq!(j.i, 3.0);
    let k = Cplx::from(5.0);
    assert_eq!(k.r, 5.0);
    assert_eq!(k.i, 0.0);
  }

  // cargo test -- --nocapture to see print output

  #[test]
  fn ops() {
    // unary negation
    let i = -Cplx::new(0.0, 1.0);
    assert_eq!(i.re(), 0.0);
    assert_eq!(i.im(), -1.0);
    let j = -Cplx::from_normarg(5.0, (0.75).atan());
    assert_eq!(j.re(), -4.0);
    assert_eq!(j.im(), -3.0);

    // Addition
    assert!(compare_z(i+j, Cplx::new(-4.0, -4.0)));
    assert!(compare_z(i+1.0, Cplx::new(1.0, -1.0)));

    // Subtraction
    //println!("sqrt({}-{})={}",i,j,i-j);
    assert!(compare_z(i-j, Cplx::new(4.0, 2.0)));
    assert!(compare_z(j-3.0, Cplx::new(-7.0, -3.0)));

    // Multiplication
    // println!("{}*{}={}",i,j,i * j);
    // println!("{}*{}={}",i,2.0,i * 2.0);
    assert!(compare_z(i*j, Cplx::new(-3.0, 4.0)));
    assert!(compare_z(i*2.0, Cplx::new(0.0, -2.0)));

    // Division
    // println!("{}/{}={}",i,j,i / j);
    // println!("{}/{}={}",i,2.0,i / 2.0);
    assert!(compare_z(i/j, Cplx::new(0.12, 0.16)));
    assert!(compare_z(j/2.0, Cplx::new(-2.0, -1.5)));
    // 1/i
    assert!(compare_z(i.recip(), Cplx::new(0.0, 1.0)));
  }


  #[test]
  fn funcs() {
    let i = Cplx::new(0.0, 1.0);
    let i2 = Cplx::from(-1.0);

    assert!(compare_z(i, Cplx::new(0.0, -1.0).conj()));

    assert!(compare_z(i2.sqrt(), i));
    assert!(compare_z(sqrt(i2), i));

  }
}
