
// Ziggurat algorithms for exponential distribution 
use crate::gen::RandomStream;
use crate::gen::Rejectable;
use crate::gen::Dimensionless;

// adapted from Marsaglia & Tsang The Ziggurat Method for Generating Random Variables https://core.ac.uk/download/pdf/6287927.pdf

const NSTRIPS: usize = 256;

// Marsaglia's Ziggurat method of sampling exponentials
pub struct Ziggurat<R> {
  rng: R,
  //v: f64,
  x: [f64; NSTRIPS+1],
  r: [f64; NSTRIPS]
}

impl<R: RandomStream + Dimensionless + Rejectable> Ziggurat<R> {
  pub fn new(rng: R) -> Ziggurat<R> {
    static X1: f64 = 7.697117470131487; /* start of the right tail */
    static V: f64 = 3.949659822581572e-3;

    let mut x = [0.0; NSTRIPS+1];
    let mut r = [0.0; NSTRIPS];

    let f = V * X1.exp();
    x[0] = V / f;
    x[1] = X1;
    //x[NSTRIPS] = 0.0;

    for i in 2..NSTRIPS {
      x[i] = (-2.0 * (V / x[i-1] + f).ln()).sqrt();
      x[i] = -(V / x[i-1] + (-f).exp()).ln();
    }
    for i in 0..NSTRIPS {
      r[i] = x[i+1] / x[i];
    }

    Ziggurat{rng: rng, x: x, r: r}
  }

// voidzigset(unsignedlongjsrseed)
// {
//   const double m1=2147483648.0,m2=4294967296.;
//   double dn=3.442619855899,tn=dn,vn=9.91256303526217e-3,q;
//   double de=7.697117470131487,te=de,ve=3.949659822581572e-3;
//   inti;
//   jsr=jsrseed;
//   /*TablesforRNOR:*/
//   q=vn/exp(-.5*dn*dn);
//   kn[0]=(dn/q)*m1;
//   kn[1]=0;
//   wn[0]=q/m1;
//   wn[127]=dn/m1;
//   fn[0]=1.;
//   fn[127]=exp(-.5*dn*dn);
//   for(i=126;i>=1;i--){
//     dn=sqrt(-2.*log(vn/dn+exp(-.5*dn*dn)));
//     kn[i+1]=(dn/tn)*m1;
//     tn=dn;fn[i]=exp(-.5*dn*dn);
//     wn[i]=dn/m1;
//   }
//   /*TablesforREXP*/
//   q=ve/exp(-de);
//   ke[0]=(de/q)*m2;
//   ke[1]=0;
//   we[0]=q/m2;
//   we[255]=de/m2;
//   fe[0]=1.;
//   fe[255]=exp(-de);for(i=254;i>=1;i--){
//     de=-log(ve/de+exp(-de));
//     ke[i+1]=(de/te)*m2;te=de;
//     fe[i]=exp(-de);
//     we[i]=de/m2;}
//   }
// }

  // TODO
  fn tail(&mut self, dmin: f64, neg: bool) -> f64 {
    let mut x;
    let mut y;
    loop {
      x = self.rng.uniform01().ln() / dmin;
      y = self.rng.uniform01().ln();
      if -2.0 * y >= x * x { break };
    }
    // let mult = if neg { 1.0 } else { -1.0 };
    // return (dmin - x) * mult;
    if neg { return x - dmin; } else { return dmin - x; }
  }

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
    // mean should be < 1/lambda (=1) so sum should be ~N
    let eps = 2.0 * (N as f64).sqrt();
    assert!(v.iter().sum::<f64>() < (N as f64) + eps);
    //assert!(v.iter().sum::<f64>() > (N as f64) - eps);
  }

}

