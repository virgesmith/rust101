//use crate::gen::*;

pub mod polar;
pub mod acklam;
pub mod ziggurat;

// use C libm implementation (as erfc not in rust libm)
//#[link(name = "m")] // libm.so already linked?
extern {
  fn erfc(x: f64) -> f64;
}

const SQRT2PI: f64 = 2.0 * std::f64::consts::SQRT_2 / std::f64::consts::FRAC_2_SQRT_PI;
const R_SQRT2PI: f64 = std::f64::consts::FRAC_2_SQRT_PI / (2.0 * std::f64::consts::SQRT_2);

pub fn pdf(x: f64, mu: f64, sigma: f64) -> f64 {
  (-0.5*((x-mu)/sigma).powi(2)).exp() * R_SQRT2PI / sigma
}

pub fn cdf(x: f64, mu: f64, sigma: f64) -> f64 {
  standard_cdf((x-mu)/sigma) 
} 

// pub fn inv_cdf(x: f64, mu: f64, sigma: f64) -> f64 {
//   //standard_inv_cdf(x) * sigma + mu
// }
// standard (zero mean unit variance) implementation provided for efficient in below algorithms

// standard normal i.e. zero mean unit variance 
fn standard_cdf(x: f64) -> f64 {
  0.5 * unsafe { erfc(-x * std::f64::consts::FRAC_1_SQRT_2) }
}


