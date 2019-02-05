
#[derive(Clone, Debug)]
pub struct Vector {
  pub i: f64,
  pub j: f64,
  pub k: f64
}

static EPS: f64 = 1e-6; //2.0f64.powi(-26); // sqrt double epsilon

impl Vector {

  pub fn new(i: f64, j: f64, k: f64) -> Vector {
    Vector{i: i, j: j, k: k}
  }

  pub fn get_magnitude(self: &Vector) -> f64 {
    (self.i.powi(2) + self.j.powi(2) + self.k.powi(2)).sqrt()
  }

  pub fn add(self: &Vector, rhs: &Vector) -> Vector {
    Vector{ i: self.i + rhs.i, j: self.j + rhs.j, k: self.k + rhs.k }
  }

  pub fn multiply_by_scalar(self: &Vector, x: f64) -> Vector {
    Vector{ i: self.i * x, j: self.j * x, k: self.k * x }
  }

  pub fn dot(self: &Vector, rhs: &Vector) -> f64 {
    self.i * rhs.i + self.j * rhs.j + self.k * rhs.k
  }

  pub fn cross(self: &Vector, rhs: &Vector) -> Vector {
    Vector{ 
      i: self.j * rhs.k - self.k * rhs.j,
      j: self.k * rhs.i - self.i * rhs.k,
      k: self.i * rhs.j - self.j * rhs.i
    }
  }

  pub fn normalize(self: &Vector) -> Vector {
    let len = self.get_magnitude();
    if len == 0.0 {
      panic!("cannot normalise zero vector")
    }
    Vector{ i: self.i / len, j: self.j / len, k: self.k / len }
  }

  pub fn is_normalized(self: &Vector) -> bool {
    (self.get_magnitude() - 1.0).abs() < EPS
  }

  pub fn is_parallel_to(self: &Vector, rhs: &Vector) -> bool {
    self.get_magnitude() > 0.0 &&
    rhs.get_magnitude() > 0.0 &&
    self.cross(rhs).get_magnitude() < EPS
  }

  pub fn is_perpendicular_to(self: &Vector, rhs: &Vector) -> bool {
    self.get_magnitude() > 0.0 &&
    rhs.get_magnitude() > 0.0 &&
    self.dot(rhs).abs() < EPS 
  }

  pub fn get_zero() -> Vector {
    Vector{ i: 0.0, j: 0.0, k: 0.0 }
  }

  pub fn get_i() -> Vector {
    Vector{ i: 1.0, j: 0.0, k: 0.0 }
  }

  pub fn get_j() -> Vector {
    Vector{ i: 0.0, j: 1.0, k: 0.0 }
  }

  pub fn get_k() -> Vector {
    Vector{ i: 0.0, j: 0.0, k: 1.0 }
  }
}



