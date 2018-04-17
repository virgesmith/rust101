
// This is pretty ugly and verbose compared to C++ (IMHO). see main.cpp
// which is runtime polymorphic requiring unique_ptr to be used
// not sure how to capture this in rust, below errors
//  let rect: Shape = Rect{ w:30, h:50 };

// Also interesting how separating struct and impl allows deferral of type errors

use std::ops::Mul;
use std::convert::Into;


// no constraints on T here? but assume better practice to enforce traits here? (as opposed to impl) 
#[derive(Debug)]
struct Rect<T> {
  w: T,
  h: T
}

#[derive(Debug)]
struct Ellipse<T> {
  rx: T,
  ry: T
}

fn main() {
  let width1 = 30;
  let height1 = 50;
  println!("The area of the rectangle is {} square pixels.", area(width1, height1));

  let dims = (30, 50);
  println!("The area of the rectangle is {} square pixels.", area_t(dims));

  let rect = Rect{ w:30, h:50 };
  println!("The area of the rectangle {:?} is {} square pixels aspect {}.", 
    rect, rect.area(), rect.aspect());

  let oval = Ellipse{ ry:50, rx:30 };
  println!("The area of the ellipse {:?} is {} square pixels aspect {}.", 
    oval, oval.area(), oval.aspect());

  // no compile error on this line (T restrictions are on impl only)
  let _error = Rect{ w:"hello", h:"world"};
  // error here... error.area();
}
  
fn area(width: u32, height:u32) -> u32 {
  width * height
}

// tuple version
fn area_t(dims: (u32, u32)) -> u32 {
  dims.0 * dims.1
}

// struct version
trait Shape {
  fn area(&self) -> f64;
  fn aspect(&self) -> f64;
}

// restrict T to a type that supports multiplication and conversion to double
// no idea why clone() is required? Perhaps into() returns a ref to self something like this:
// template<typename T> T& into() { return *static_cast<T*>(this); }
impl<T> Shape for Rect<T> where T: Mul<Output=T> + Clone + Into<f64> {
  fn area(&self) -> f64 {
    self.w.clone().into() * self.h.clone().into() //as f64
  }

  fn aspect(&self) -> f64 {
    self.w.clone().into() / self.h.clone().into() 
  }
}

// struct version
impl<T> Shape for Ellipse<T> where T: Mul<Output=T> + Clone + Into<f64> {
  fn area(&self) -> f64 {
    self.rx.clone().into() * self.ry.clone().into() * std::f64::consts::PI 
  }

  fn aspect(&self) -> f64 {
    self.rx.clone().into() / self.ry.clone().into() 
  }
}