
use crate::dist::*;

//#[link(name = "m")] // libm.so already linked?
extern {
  fn erfc(x: f64) -> f64;
}

fn cdf(x: f64) -> f64 {
   0.5 * unsafe { erfc(-x * std::f64::consts::FRAC_1_SQRT_2) }
}

// Marsaglia's polar method of sampling normals
struct Polar {
  is_cached: bool,
  cached_val: f64
}

impl Polar {
  fn new() -> Polar {
    Polar{is_cached: false, cached_val: std::f64::NAN}
  }

  fn get(&mut self, rng: &mut impl PRNG) -> f64 {
    if self.is_cached {
      self.is_cached = false;
      return self.cached_val;
    }
    loop {
      let (x,y) = (rng.uniform01() * 2.0 - 1.0, rng.uniform01() * 2.0 - 1.0);
      let s = x*x + y*y;
      if s > 0.0 && s < 1.0 {
        let m = (-2.0 * s.ln() / s).sqrt();
        self.is_cached = true;
        self.cached_val = /*self.mu + self.sigma **/ y * m;
        return /*self.mu + self.sigma **/ x * m;
      }
    }
  }
}

// Peter Acklam's inverse cumulative normal approximation
fn inv_cdf(x: f64) -> f64
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
  const D3: f64 =  3.754408661907416e+00;;
                                
  let t;
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
  let mut t = cdf(u)-q;    /* error */
  const SQRT2PI: f64 = 2.0 * std::f64::consts::SQRT_2 / std::f64::consts::FRAC_2_SQRT_PI;
  t = t*SQRT2PI*(u*u/2.0).exp();   /* f(u)/df(u) */
  u = u-t/(1.0+u*t/2.0);     /* Halley's method */

  match x {
    x if x > 0.5 => -u,
    _ => u
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_polar() {

  }

  #[test]
  fn test_acklam() {
    let f = (0..1000).map(|i| (i as f64)/1000.0).collect::<Vec<f64>>();
    let x = f.iter().map(|&fi| inv_cdf(fi)).collect::<Vec<f64>>();
    for i in (0..1000) {
      //println!("{} {}", x, cdf(x));
      assert!((f[i] - cdf(x[i])).abs() < std::f64::EPSILON);
    }
  }

}
