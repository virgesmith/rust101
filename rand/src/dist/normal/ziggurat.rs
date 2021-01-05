// Algorithms to transform uniform variates to normal
use crate::gen::RandomStream;
use crate::gen::Rejectable;
use crate::gen::Dimensionless;

// adapted from https://www.doornik.com/research/ziggurat.pdf

const NSTRIPS: usize = 128;

// Marsaglia's Ziggurat method of sampling normals
pub struct Ziggurat<R> {
  rng: R,
  //v: f64,
  x: [f64; NSTRIPS+1],
  r: [f64; NSTRIPS]
}

impl<R: RandomStream + Dimensionless + Rejectable> Ziggurat<R> {
  pub fn new(rng: R) -> Ziggurat<R> {

    static X1: f64 = 3.442619855899; /* start of the right tail */
    static V: f64 = 9.91256303526217e-3;

    let mut x = [0.0; NSTRIPS+1];
    let mut r = [0.0; NSTRIPS];

    let f = (0.5 * X1 * X1).exp();
    x[0] = V / f;
    x[1] = X1;
    //x[NSTRIPS] = 0.0;
    for i in 2..NSTRIPS {
      x[i] = (-2.0 * (V / x[i-1] + f).ln()).sqrt();
    }
    for i in 0..NSTRIPS {
      r[i] = x[i+1] / x[i];
    }

    Ziggurat{rng, x, r}
  }

  // #define ZIGNOR_C 128                   /* number of blocks */
  // #define ZIGNOR_R 3.442619855899 /* start of the right tail */
  // /* (R * phi(R) + Pr(X>=R)) * sqrt(2\pi) */
  // #define ZIGNOR_V 9.91256303526217e-3
  // /* s_adZigX holds coordinates, such that each rectangle has*/
  // /* same area; s_adZigR holds s_adZigX[i + 1] / s_adZigX[i] */
  // static double s_adZigX[ZIGNOR_C + 1], s_adZigR[ZIGNOR_C];

  // static void zigNorInit(int iC, double dR, double dV)
  // {
  //   int i;
  //   double f;
  //   f = exp(-0.5 * dR * dR);
  //   s_adZigX[0] = dV / f; /* [0] is bottom block: V / f(R) */
  //   s_adZigX[1] = dR;s_adZigX[iC] = 0;
  //   for (i = 2; i < iC; ++i)
  //   {
  //     s_adZigX[i] = sqrt(-2 * log(dV / s_adZigX[i - 1] + f));
  //     f = exp(-0.5 * s_adZigX[i] * s_adZigX[i]);
  //   }
  //   for (i = 0; i < iC; ++i)
  //     s_adZigR[i] = s_adZigX[i + 1] / s_adZigX[i];
  // }

  //static double DRanNormalTail(double dMin, int iNegative)
  //{
  //  double x, y;
  //  do
  //   {
  //     x = log(DRanU()) / dMin;
  //     y = log(DRanU());
  //   } while (-2 * y < x * x);
  //   return iNegative ? x - dMin : dMin - x;
  // }

  // TODO remove dmin (=x[1])
  fn tail(&mut self, dmin: f64, neg: bool) -> f64 {
    let mut x;
    let mut y;
    loop {
      x = self.rng.uniform01().ln() / dmin;
      y = self.rng.uniform01().ln();
      if -2.0 * y >= x * x { break };
    }
    if neg { x - dmin } else { dmin - x }
  }

  //double  DRanNormalZig(void){
    // unsigned int i;
    // double x, u, f0, f1;
    // for (;;){
    //   u = 2 * DRanU() - 1;
    //   i = IRanU() & 0x7F;
    //   /* first try the rectangular boxes */
    //   if (fabs(u) < s_adZigR[i])return u * s_adZigX[i];
    //   /* bottom box: sample from the tail */
    //   if (i == 0)return DRanNormalTail(ZIGNOR_R, u < 0);
    //   /* is this a sample from the wedges? */
    //   x = u * s_adZigX[i];
    //   f0 = exp(-0.5 * (s_adZigX[i] * s_adZigX[i] - x * x) );
    //   f1 = exp(-0.5 * (s_adZigX[i+1] * s_adZigX[i+1] - x * x) )
    //   if (f1 + DRanU() * (f0 - f1) < 1.0)
    //   return x;
    //   }
    // }

  fn get_impl(&mut self) -> f64 {
    loop {
      let u = 2.0 * self.rng.uniform01() - 1.0;
      let i = self.rng.next_1() as usize % NSTRIPS;
      if u.abs() < self.r[i] { return u * self.x[i]; }
      if i == 0 { return self.tail(self.x[1], u < 0.0); }
      let x = u * self.x[i];
      let f0 = (-0.5 * (self.x[i] * self.x[i] - x * x)).exp();
      let f1 = (-0.5 * (self.x[i+1] * self.x[i+1] - x * x)).exp();
      if f1 + self.rng.uniform01() * (f0 - f1) < 1.0 { return x; }
    }
  }

  pub fn get_n(&mut self, n: usize) -> Vec<f64> {
    (0..n).map(|_| self.get_impl()).collect()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::gen::pseudo::*;

  const N: usize = 60000;

  #[test]
  fn test_ziggurat() {

    let mut z = Ziggurat::new(MT19937::new(Some(19937)));
    let v = z.get_n(N);
    // mean should be < 1/sqrt(N) so sum should be < sqrt(N)
    assert!(v.iter().sum::<f64>() < (N as f64).sqrt());
  }
}

