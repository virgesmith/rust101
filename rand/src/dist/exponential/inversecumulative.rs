
// Ziggurat algorithms for exponential distribution 
use crate::gen::RandomStream;
use crate::gen::Rejectable;
use crate::gen::Dimensionless;
use super::inv_cdf;
// adapted from Marsaglia & Tsang The Ziggurat Method for Generating Random Variables https://core.ac.uk/download/pdf/6287927.pdf

// Marsaglia's Ziggurat method of sampling exponentials
pub struct InverseCumulative<R> {
  rng: R
}

impl<R: RandomStream + Dimensionless + Rejectable> InverseCumulative<R> {
  pub fn new(rng: R) -> InverseCumulative<R> {
    InverseCumulative{rng: rng}
  }

  fn get_impl(&mut self) -> f64 {
    inv_cdf(self.rng.uniform01(), 1.0)
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
  fn inverse_cumulative() {

    let mut d = InverseCumulative::new(MT19937::new(Some(19937)));
    let v = d.get_n(N);
    // mean should be < 1/lambda (=1) so sum should be ~N
    let eps = (N as f64).sqrt();
    assert!(v.iter().sum::<f64>() < (N as f64) + eps);
    assert!(v.iter().sum::<f64>() > (N as f64) - eps);
  }

}

