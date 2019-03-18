// Algorithms to transform uniform variates to exponential 

pub fn pdf(x: f64, lambda: f64) -> f64 {
  match x {
    x if x < 0.0 => 0.0,
    _ => lambda * (-lambda * x).exp()
  }
}

pub fn cdf(x: f64, lambda: f64) -> f64 {
  match x {
    x if x < 0.0 => 0.0,
    _ => 1.0 - (-lambda * x).exp()
  }
}

// when sampling from uinform [0,1] randoms, by symmetry can just use f rather than 1-f 
pub fn inv_cdf(f: f64, lambda: f64) -> f64 {
  assert!(lambda > 0.0);
  assert!(f >= 0.0 && f <= 1.0);
  -(1.0-f).ln() / lambda
}

#[cfg(test)]
mod test {
  use super::*;
  // use crate::gen::*;
  // use crate::gen::pseudo::*;
  // use crate::gen::quasi::*;

  fn close_rel_eps(x: f64, y: f64, eps: Option<f64>) -> bool {
    let eps = eps.unwrap_or(std::f64::EPSILON);
    let rel = (x/y-1.0).abs();
    if !(rel <= eps) {
      println!("{}, {} relative diff is {} ({})", x, y, rel, eps);
    }
    rel <= eps
  }

  fn close_abs_eps(x: f64, y: f64, eps: Option<f64>) -> bool {
    let eps = eps.unwrap_or(std::f64::EPSILON);
    let abs = (x - y).abs();
    if !(abs <= eps) {
      println!("{}, {} abs diff is {}", x, y, abs);
    }
    abs <= eps
  }

  #[test]
  fn test_basics() {
    assert!(close_abs_eps(pdf(-1.0, 1.0), 0.0, None));
    assert!(close_rel_eps(pdf(1.0, 1.0), (-1.0f64).exp(), None));
    assert!(close_rel_eps(pdf(1.0, 2.0), 2.0 * (-2.0f64).exp(), None));
    assert!(close_rel_eps(pdf(2.0, 1.0), (-2.0f64).exp(), None));
    assert!(close_abs_eps(pdf(std::f64::INFINITY, 1.0), 0.0, None));

    assert!(close_abs_eps(cdf(-1.0, 1.0), 0.0, None));
    assert!(close_rel_eps(cdf(1.0, 1.0), 1.0 - (-1.0f64).exp(), None));
    assert!(close_rel_eps(cdf(1.0, 2.0), 1.0 - (-2.0f64).exp(), None));
    assert!(close_rel_eps(cdf(2.0, 1.0), 1.0 - (-2.0f64).exp(), None));
    assert!(close_abs_eps(cdf(std::f64::INFINITY, 1.0), 1.0, None));

    for i in 1..10 {
      let x = i as f64;
      assert!(close_rel_eps(inv_cdf(cdf(x, 1.0), 1.0), x, Some(1024.0 * std::f64::EPSILON)));
    }
  }
}