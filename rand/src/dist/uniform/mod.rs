// uniform mod.rs

// Algorithms to transform uniform variates

pub fn pdf(x: f64, l: f64, h: f64) -> f64 {
  assert!(l < h);
  match x {
    x if x < l || x > h => 0.0,
    _ => 1.0 / (h - l)
  }
}

// inverse pdf is not well defined:
//   0 -> any value <l or >h
//   1/(h-l) -> any value in [l,h]

pub fn cdf(x: f64, l: f64, h: f64) -> f64 {
  assert!(l < h);
  match x {
    x if x <= l => 0.0,
    x if x >= h => 1.0,
    _ => (x - l) / (h - l)
  }
}

pub fn inv_cdf(f: f64, l: f64, h: f64) -> f64 {
  assert!(l < h);
  assert!((0.0..=1.0).contains(&f));
  l + h * f
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::dist::compare::close_rel_eps;

  #[test]
  fn uniform() {
    for i in 1..10 {
      let x = i as f64 * 0.1;
      assert!(close_rel_eps(inv_cdf(cdf(x, 0.0, 1.0), 0.0, 1.0), x, Some(1024.0 * std::f64::EPSILON)));
      assert!(close_rel_eps(pdf(x, 0.0, 1.0), 1.0, Some(1024.0 * std::f64::EPSILON)));
    }
  }
}