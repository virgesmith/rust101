#[cfg(test)]
mod tests {
  use vector::{Vector, dot, cross, parallel, perpendicular};

  fn are_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.000001
  }

  #[test]
  fn constructor_test() {
    let v = Vector::new(1.0, 2.0, 3.0);
    assert_eq!(1.0, v.i, "Value of first argument passed into struct constructor should be assigned to \"i\"");
    assert_eq!(2.0, v.j, "Value of second argument passed into struct constructor should be assigned to \"j\"");
    assert_eq!(3.0, v.k, "Value of third argument passed into struct constructor should be assigned to \"k\"");

    let v = Vector::new(-4.0 / 3.0, 40.0 / 27.0, 68.0 / 69.0);
    assert!(are_equal(v.i, -4.0 / 3.0), "\"i\" of Vector should equal -4 / 3");
    assert!(are_equal(v.j, 40.0 / 27.0), "\"j\" of Vector should equal 40 / 27");
    assert!(are_equal(v.k, 68.0 / 69.0), "\"k\" of Vector should equal 68 / 69");
  }

  #[test]
  fn modulus_test() {
    let v = Vector::new(6.0, 10.0, -3.0);
    assert!(are_equal(v.modulus(), (145 as f64).sqrt()));
    // note operator precedence here
    assert!(are_equal(-v.modulus(), -(145 as f64).sqrt()));
    assert!(are_equal((-v).modulus(), (145 as f64).sqrt()));
  }

  #[test]
  fn static_methods_test() {
    let z = Vector::zero();
    let i = Vector::unit_i();
    let j = Vector::unit_j();
    let k = Vector::unit_k();
    assert_eq!(0.0, z.i);    assert_eq!(0.0, z.j);    assert_eq!(0.0, z.k);
    assert_eq!(1.0, i.i);    assert_eq!(0.0, i.j);    assert_eq!(0.0, i.k);
    assert_eq!(0.0, j.i);    assert_eq!(1.0, j.j);    assert_eq!(0.0, j.k);
    assert_eq!(0.0, k.i);    assert_eq!(0.0, k.j);    assert_eq!(1.0, k.k);
  }

  #[test]
  fn addsub_test() {
    let v = Vector::new(3.0, 7.0 / 2.0, -3.0 / 2.0);
    let s: Vector = v + &Vector::new(-27.0, 3.0, 4.0);
    assert!(are_equal(s.i, -24.0));
    assert!(are_equal(s.j, 13.0 / 2.0));
    assert!(are_equal(s.k, 5.0 / 2.0));
    let s: Vector = s - &Vector::new(-27.0, 3.0, 4.0);
    assert!(are_equal(s.i, 3.0));
    assert!(are_equal(s.j, 7.0 / 2.0));
    assert!(are_equal(s.k, -3.0 / 2.0));
  }

  #[test]
  fn muldiv_test() {
    let v = Vector::new(1.0 / 3.0, 177.0 / 27.0, -99.0);
    let e = v * (-3.0 / 7.0);
    assert!(are_equal(e.i, -1.0 / 7.0));
    assert!(are_equal(e.j, -59.0 / 21.0));
    assert!(are_equal(e.k, 297.0 / 7.0));
    let e = e / (-3.0 / 7.0);
    assert!(are_equal(e.i, 1.0 / 3.0));
    assert!(are_equal(e.j, 177.0 / 27.0));
    assert!(are_equal(e.k, -99.0));
  }

  #[test]
  fn dot_test() {
    let v = Vector::new(-99.0 / 71.0, 22.0 / 23.0, 45.0);
    assert!(are_equal(dot(&v, &Vector::new(-5.0, 4.0, 7.0)), 325.7979179));
    assert!(are_equal(dot(&v, &Vector::new(5.0, -4.0, -7.0)), -325.7979179));
  }

  #[test]
  fn cross_test() {
    let a = Vector::new(2.0, 1.0, 3.0);
    let b = Vector::new(4.0, 6.0, 5.0);
    let a_cross_b = cross(&a, &b);
    let b_cross_a = cross(&b, &a);
    assert!(are_equal(a_cross_b.i, -13.0));
    assert!(are_equal(a_cross_b.j, 2.0));
    assert!(are_equal(a_cross_b.k, 8.0));
    assert!(are_equal(b_cross_a.i, 13.0));
    assert!(are_equal(b_cross_a.j, -2.0));
    assert!(are_equal(b_cross_a.k, -8.0));
  }

  #[test]
  fn parallel_test() {
    let a = Vector::new(1045.0 / 23.0, -666.0 / 37.0, 15.0);
    let b = Vector::new(161.3385037, -59124.0 / 925.0, 9854.0 / 185.0);
    assert!(parallel(&a, &b));
    assert!(parallel(&b, &a));
    let c = Vector::new(-3.0, 0.0, 5.0);
    let d = Vector::new(-12.0, 1.0, 20.0);
    assert!(!parallel(&c, &d));
    assert!(!parallel(&d, &c));
  }

  #[test]
  fn perpendicular_test() {
    let a = Vector::new(3.0, 4.0, 7.0);
    let b = Vector::new(1.0 / 3.0, 2.0, -9.0 / 7.0);
    assert!(perpendicular(&a, &b));
    assert!(perpendicular(&b, &a));
    let c = Vector::new(1.0, 3.0, 5.0);
    let d = Vector::new(-2.0, -7.0, 4.4);
    println!("{:?}", dot(&c, &d));
    assert!(!perpendicular(&c, &d));
    assert!(!perpendicular(&d, &c));
  }

  #[test]
  fn normalize_test() {
    let v = Vector::new(-1.0, -1.0, 1.0);
    let u = v.normalize();
    assert!(are_equal(u.modulus(), 1.0));
    assert!(are_equal(u.i, -1.0 / (3.0 as f64).sqrt()));
    assert!(are_equal(u.j, -1.0 / (3.0 as f64).sqrt()));
    assert!(are_equal(u.k, 1.0 / (3.0 as f64).sqrt()));
  }

  #[test]
  fn is_normalized_test() {
    let a = Vector::new(-1.0 / (2.0 as f64).sqrt(), 0.0, 1.0 / (2.0 as f64).sqrt());
    let b = Vector::new(1.0, 1.0, 1.0);
    assert!(a.is_normalized());
    assert!(!b.is_normalized());
  }
}
