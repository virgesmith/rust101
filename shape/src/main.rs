

use std::convert::Into;

// struct version
trait Shape {
  fn area(&self) -> f64;
  fn aspect(&self) -> f64;
}


// no constraints on T here? but assume better practice to enforce traits here? (as opposed to impl?)
#[derive(Debug)]
struct Rectangle<T> {
  w: T,
  h: T
}


#[derive(Debug)]
struct Circle<T> {
  r: T,
}

// restrict T to a type that supports multiplication and conversion to double
// no idea why clone() is required?
impl<T> Shape for Rectangle<T> where T: Clone + Into<f64> {
  fn area(&self) -> f64 {
    self.w.clone().into() * self.h.clone().into() //as f64
  }

  fn aspect(&self) -> f64 {
    self.w.clone().into() / self.h.clone().into()
  }
}

// struct version
impl<T> Shape for Circle<T> where T: Clone + Into<f64> {
  fn area(&self) -> f64 {
    self.r.clone().into() * self.r.clone().into() * std::f64::consts::PI
  }

  fn aspect(&self) -> f64 {
    1.0
  }
}

fn main() {

  let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle::<i32>{r:1}),
    Box::new(Rectangle::<f64>{w:1.0, h:3.14})
  ];

  shapes.iter().for_each(|s| println!("area={:.3} aspect={:.3}", s.area(), s.aspect()));
}


