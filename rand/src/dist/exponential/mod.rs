
pub mod inversecumulative;
pub mod ziggurat;

// Algorithms to transform uniform variates to exponential 

pub fn pdf(x: f64, lambda: f64) -> f64 {
  match x {
    x if x < 0.0 => 0.0,
    _ => lambda * (-lambda * x).exp()
  }
}

pub fn inv_pdf(f: f64, lambda: f64) -> f64 {
  assert!(f > 0.0);
  -(f / lambda).ln() / lambda
}

pub fn cdf(x: f64, lambda: f64) -> f64 {
  match x {
    x if x < 0.0 => 0.0,
    _ => 1.0 - (-lambda * x).exp()
  }
}

// when sampling from uniform [0,1] randoms, by symmetry can just use f rather than 1-f ?
pub fn inv_cdf(f: f64, lambda: f64) -> f64 {
  assert!(lambda > 0.0);
  assert!(f >= 0.0 && f <= 1.0);
  -(1.0-f).ln() / lambda
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::dist::compare::{close_abs_eps, close_rel_eps};


  #[test]
  fn exp_basics() {
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
      assert!(close_rel_eps(inv_pdf(pdf(x, 1.0), 1.0), x, Some(1024.0 * std::f64::EPSILON)));
    }
  }
}