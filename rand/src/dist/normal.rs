// Algorithms to transform uniform variates to normal 
use crate::gen::RandomStream;
use crate::gen::Rejectable;
use crate::gen::Dimensionless;

// Marsaglia's polar method of sampling normals
#[derive(Debug)]
pub struct Polar<R> {
  rng: R,
  is_cached: bool,
  cached_val: f64
}

impl<R: RandomStream + Dimensionless + Rejectable> Polar<R> {
  pub fn new(rng: R) -> Polar<R> {
    Polar{rng: rng, is_cached: false, cached_val: std::f64::NAN}
  }

  fn get_impl(&mut self) -> f64 {
    if self.is_cached {
      self.is_cached = false;
      return self.cached_val;
    }
    loop {
      let (x,y) = (self.rng.uniform01() * 2.0 - 1.0, self.rng.uniform01() * 2.0 - 1.0);
      let s = x*x + y*y;
      if s > 0.0 && s < 1.0 {
        let m = (-2.0 * s.ln() / s).sqrt();
        self.is_cached = true;
        self.cached_val = y * m;
        return x * m;
      }
    }
  }

  pub fn get_n(&mut self, n: usize) -> Vec<f64> {
    (0..n).map(|_| self.get_impl()).collect()
  }
}

// Acklam's inverse cumulative normal appproximation
#[derive(Debug)]
pub struct InverseCumulative<R> 
{ 
  rng: R
}

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

pub fn inv_cdf(x: f64, mu: f64, sigma: f64) -> f64 {
  standard_inv_cdf(x) * sigma + mu
}
// standard (zero mean unit variance) implementation provided for efficient in below algorithms

// standard normal i.e. zero mean unit variance 
fn standard_cdf(x: f64) -> f64 {
  0.5 * unsafe { erfc(-x * std::f64::consts::FRAC_1_SQRT_2) }
}

// Peter Acklam's inverse cumulative standard normal approximation
fn standard_inv_cdf(x: f64) -> f64
{                       
  const A0: f64 = -3.969683028665376e+01;
  const A1: f64 =  2.209460984245205e+02; 
  const A2: f64 = -2.759285104469687e+02;
  const A3: f64 =  1.383577518672690e+02; 
  const A4: f64 = -3.066479806614716e+01;
  const A5: f64 =  2.506628277459239e+00;

  const B0: f64 = -5.447609879822406e+01;
  const B1: f64 =  1.615858368580409e+02;
  const B2: f64 = -1.556989798598866e+02;
  const B3: f64 =  6.680131188771972e+01;
  const B4: f64 = -1.328068155288572e+01;
                              
  const C0: f64 = -7.784894002430293e-03; 
  const C1: f64 = -3.223964580411365e-01;
  const C2: f64 = -2.400758277161838e+00; 
  const C3: f64 = -2.549732539343734e+00;
  const C4: f64 =  4.374664141464968e+00;
  const C5: f64 =  2.938163982698783e+00;
                                
  const D0: f64 =  7.784695709041462e-03;
  const D1: f64 =  3.224671290700398e-01; 
  const D2: f64 =  2.445134137142996e+00;
  const D3: f64 =  3.754408661907416e+00;
                                
  let mut t;
  let mut u;

  // is is_nan necessary?
  assert!(!x.is_nan() && x >= 0.0 && x <= 1.0);

  if x == 0.0 { return std::f64::NEG_INFINITY; }
  if x == 1.0 { return  std::f64::INFINITY; }

  // q = min(x, 1.0 - x);
  let q = match x {
    x if x < 0.5 => x,
    _ => 1.0 - x
  }; 

  if q > 0.02425 {
    /* Rational approximation for central region. */
    u = q-0.5;
    t = u*u;
    u = u*(((((A0*t+A1)*t+A2)*t+A3)*t+A4)*t+A5) / (((((B0*t+B1)*t+B2)*t+B3)*t+B4)*t+1.0);
  } else {
    /* Rational approximation for tail region. */
    t = (-2.0*q.ln()).sqrt();
    u = (((((C0*t+C1)*t+C2)*t+C3)*t+C4)*t+C5) / ((((D0*t+D1)*t+D2)*t+D3)*t+1.0);
  }

  /* The relative error of the approximation has absolute value less
    than 1.15e-9.  One iteration of Halley's rational method (third
    order) gives full machine precision... */
  t = standard_cdf(u)-q;    /* error */
  t = t*SQRT2PI*(u*u/2.0).exp();   /* f(u)/df(u) */
  u = u-t/(1.0+u*t/2.0);     /* Halley's method */

  match x {
    x if x > 0.5 => -u,
    _ => u
  }
}

impl<R: RandomStream> InverseCumulative<R> {
  
  pub fn new(rng: R) -> InverseCumulative<R> {
    InverseCumulative{ rng: rng }
  }

  pub fn get_n(&mut self, n: usize) -> Vec<f64> {
    self.rng.uniforms01(n).iter().map(|&x| standard_inv_cdf(x)).collect()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::gen::pseudo::*;
  use crate::gen::quasi::*;
  //use crate::gen::entropy::*;

  fn close_rel_eps(x: f64, y: f64) -> bool {
    let rel = (x/y-1.0).abs();
    if !(rel <= std::f64::EPSILON) {
      println!("{}, {} relative diff is {}", x, y, rel);
    }
    rel <= std::f64::EPSILON
  }

  fn close_abs_eps(x: f64, y: f64) -> bool {
    let abs = (x - y).abs();
    if !(abs <= std::f64::EPSILON) {
      println!("{}, {} abs diff is {}", x, y, abs);
    }
    abs <= std::f64::EPSILON
  }

  #[test]
  fn test_basics() {
    assert!(close_rel_eps(SQRT2PI, 2.5066282746310002));

    assert!(close_abs_eps(pdf(std::f64::NEG_INFINITY, 0.0, 1.0), 0.0));
    assert!(close_rel_eps(pdf(-1.0, 0.0, 1.0), 0.24197072451914337));
    assert!(close_rel_eps(pdf(0.0, 0.0, 1.0), 0.3989422804014327));
    assert!(close_rel_eps(pdf(0.5, 0.5, 1.0), 0.3989422804014327));
    assert!(close_rel_eps(pdf(1.0, 0.0, 1.0), 0.24197072451914337));
    assert!(close_rel_eps(pdf(0.5, 0.0, 0.5)*0.5, 0.24197072451914337));
    assert!(close_rel_eps(pdf(2.0, 0.0, 2.0)*2.0, 0.24197072451914337));
    assert!(close_abs_eps(pdf(std::f64::INFINITY, 0.0, 1.0), 0.0));

    assert!(close_abs_eps(cdf(std::f64::NEG_INFINITY, 0.0, 1.0), 0.0));
    assert!(close_rel_eps(cdf(-1.0, 0.0, 1.0), 0.15865525393145702));
    assert!(close_rel_eps(cdf(0.0, 0.0, 1.0), 0.5));
    assert!(close_rel_eps(cdf(1.0, 0.0, 1.0), 1.0-0.15865525393145702));
    assert!(close_rel_eps(cdf(2.0, 2.0, 1.0), 0.5));
    assert!(close_rel_eps(cdf(1.0, 2.0, 1.0), 0.15865525393145702));
    assert!(close_abs_eps(cdf(std::f64::INFINITY, 0.0, 1.0), 1.0));
  }

  #[test]
  fn test_polar() {
    const N: usize = 10000;
    let f = (0..N).map(|i| (i as f64)/(N as f64)).collect::<Vec<f64>>();
    let x = f.iter().map(|&fi| standard_inv_cdf(fi)).collect::<Vec<f64>>();
    for i in 0..N {
      assert!((f[i] - standard_cdf(x[i])).abs() < std::f64::EPSILON);
    }

    let mut polar = Polar::new(MT19937::new(Some(77027465)));
    let v = polar.get_n(N);
    println!("{} {}", N, v.iter().sum::<f64>());
    // mean should be < 1/sqrt(N) so sum should be < sqrt(N)
    assert!(v.iter().sum::<f64>() < (N as f64).sqrt());
  }

  #[test]
  fn test_acklam() {
    const N: usize = 10000;
    let f = (0..N).map(|i| (i as f64)/(N as f64)).collect::<Vec<f64>>();
    let x = f.iter().map(|&fi| standard_inv_cdf(fi)).collect::<Vec<f64>>();
    for i in 0..N {
      assert!((f[i] - standard_cdf(x[i])).abs() < std::f64::EPSILON);
    }

    let mut acklam = InverseCumulative::new(MT19937::new(Some(19937)));
    let v = acklam.get_n(N);
    // mean should be < 1/sqrt(N) so sum should be < sqrt(N)
    assert!(v.iter().sum::<f64>() < (N as f64).sqrt());
  }

  #[test]
  fn test_acklam_sobol() {
    const N: usize = 10000;
    let f = (0..N).map(|i| (i as f64)/(N as f64)).collect::<Vec<f64>>();
    let x = f.iter().map(|&fi| standard_inv_cdf(fi)).collect::<Vec<f64>>();
    for i in 0..N {
      assert!((f[i] - standard_cdf(x[i])).abs() < std::f64::EPSILON);
    }

    let mut acklam = InverseCumulative::new(Sobol::new(1));
    let v = acklam.get_n(N);
    // mean should be ~< 1/N so abs sum should be ~< 1
    assert!(v.iter().sum::<f64>().abs() < 3.0);
  }

}
