// Floating-point comparisons

pub fn close_rel_eps(x: f64, y: f64, eps: Option<f64>) -> bool {
  let eps = eps.unwrap_or(std::f64::EPSILON);
  let rel = (x/y-1.0).abs();
  if rel > eps {
    println!("{}, {} relative diff is {} ({})", x, y, rel, eps);
  }
  rel <= eps
}

pub fn close_abs_eps(x: f64, y: f64, eps: Option<f64>) -> bool {
  let eps = eps.unwrap_or(std::f64::EPSILON);
  let abs = (x - y).abs();
  if abs > eps {
    println!("{}, {} abs diff is {}", x, y, abs);
  }
  abs <= eps
}

#[cfg(test)]
mod test {

  #[test]
  fn close() {
    // assert!(close_abs_eps(pdf(-1.0, 1.0), 0.0, None));
    // assert!(close_rel_eps(pdf(1.0, 1.0), (-1.0f64).exp(), None));
    // assert!(close_rel_eps(pdf(1.0, 2.0), 2.0 * (-2.0f64).exp(), None));
    // assert!(close_rel_eps(pdf(2.0, 1.0), (-2.0f64).exp(), None));
    // assert!(close_abs_eps(pdf(std::f64::INFINITY, 1.0), 0.0, None));

    // assert!(close_abs_eps(cdf(-1.0, 1.0), 0.0, None));
    // assert!(close_rel_eps(cdf(1.0, 1.0), 1.0 - (-1.0f64).exp(), None));
    // assert!(close_rel_eps(cdf(1.0, 2.0), 1.0 - (-2.0f64).exp(), None));
    // assert!(close_rel_eps(cdf(2.0, 1.0), 1.0 - (-2.0f64).exp(), None));
    // assert!(close_abs_eps(cdf(std::f64::INFINITY, 1.0), 1.0, None));
  }
}