// Floating-point comparisons

pub fn close_rel_eps(x: f64, y: f64, eps: Option<f64>) -> bool {
  let eps = eps.unwrap_or(std::f64::EPSILON);
  let rel = (x/y-1.0).abs();
  if !(rel <= eps) {
    println!("{}, {} relative diff is {} ({})", x, y, rel, eps);
  }
  rel <= eps
}

pub fn close_abs_eps(x: f64, y: f64, eps: Option<f64>) -> bool {
  let eps = eps.unwrap_or(std::f64::EPSILON);
  let abs = (x - y).abs();
  if !(abs <= eps) {
    println!("{}, {} abs diff is {}", x, y, abs);
  }
  abs <= eps
}

