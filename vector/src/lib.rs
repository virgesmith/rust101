
#[derive(Clone, Debug)]
pub struct Vector {
  pub i: f64,
  pub j: f64,
  pub k: f64
}

const EPS: f64 = 1.0 / (1u32 << 20) as f64; // ~6 dp

impl Vector {

  pub fn new(i: f64, j: f64, k: f64) -> Vector {
    Vector{i: i, j: j, k: k}
  }

  pub fn modulus(self: &Vector) -> f64 {
    (self.i.powi(2) + self.j.powi(2) + self.k.powi(2)).sqrt()
  }

  pub fn normalize(self: &Vector) -> Vector {
    let len = self.modulus();
    if len == 0.0 {
      panic!("cannot normalise zero vector")
    }
    Vector{ i: self.i / len, j: self.j / len, k: self.k / len }
  }

  pub fn is_normalized(self: &Vector) -> bool {
    (self.modulus() - 1.0).abs() < EPS
  }

  pub fn zero() -> Vector {
    Vector{ i: 0.0, j: 0.0, k: 0.0 }
  }

  pub fn unit_i() -> Vector {
    Vector{ i: 1.0, j: 0.0, k: 0.0 }
  }

  pub fn unit_j() -> Vector {
    Vector{ i: 0.0, j: 1.0, k: 0.0 }
  }

  pub fn unit_k() -> Vector {
    Vector{ i: 0.0, j: 0.0, k: 1.0 }
  }
}

pub fn dot(lhs: &Vector, rhs: &Vector) -> f64 {
  lhs.i * rhs.i + lhs.j * rhs.j + lhs.k * rhs.k
}

pub fn cross(lhs: &Vector, rhs: &Vector) -> Vector {
  Vector{ 
    i: lhs.j * rhs.k - lhs.k * rhs.j,
    j: lhs.k * rhs.i - lhs.i * rhs.k,
    k: lhs.i * rhs.j - lhs.j * rhs.i
  }
}

pub fn parallel(lhs: &Vector, rhs: &Vector) -> bool {
  lhs.modulus() > 0.0 &&
  rhs.modulus() > 0.0 &&
  cross(lhs, rhs).modulus() < EPS
}

pub fn perpendicular(lhs: &Vector, rhs: &Vector) -> bool {
  lhs.modulus() > 0.0 &&
  rhs.modulus() > 0.0 &&
  dot(lhs, rhs).abs() < EPS 
}


impl std::ops::Neg for Vector {
  type Output = Vector;
  fn neg(self) -> Vector {
    Vector{ i: -self.i, j: -self.j, k: -self.k }
  }
}

impl std::ops::Add<&Vector> for Vector {
  type Output = Vector;
  fn add(self, rhs: &Vector) -> Vector {
    Vector{ i: self.i + rhs.i, j: self.j + rhs.j, k: self.k + rhs.k }
  }
}

impl std::ops::Sub<&Vector> for Vector {
  type Output = Vector;
  fn sub(self, rhs: &Vector) -> Vector {
    Vector{ i: self.i - rhs.i, j: self.j - rhs.j, k: self.k - rhs.k }
  }
}

impl std::ops::Mul<f64> for Vector {
  type Output = Vector;
  fn mul(self, x: f64) -> Vector {
    Vector{ i: self.i * x, j: self.j * x, k: self.k * x }
  }
}

impl std::ops::Div<f64> for Vector {
  type Output = Vector;
  fn div(self, x: f64) -> Vector {
    Vector{ i: self.i / x, j: self.j / x, k: self.k / x }
  }
}

