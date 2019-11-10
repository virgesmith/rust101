
#[derive(Debug)]
pub struct Moments {
  mean: f64,
  variance: f64,
  skew: f64,
  kurtosis: f64  
}

impl Moments {
  pub fn new(v: &[f64]) -> Moments {
    let n = v.len();
    let mean = v.iter().sum::<f64>() / n as f64;
    let var = v.iter().fold(0.0, |acc, &x| acc + (x - mean).powi(2) ) / (n as f64);
    let skew = v.iter().fold(0.0, |acc, &x| acc + (x - mean).powi(3) ) / (n as f64 * var.sqrt());
    let kurt = v.iter().fold(0.0, |acc, &x| acc + (x - mean).powi(4) ) / (n as f64 * var.sqrt());
    Moments{ mean: mean, variance: var, skew: skew, kurtosis: kurt }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::gen::pseudo::*;
  use crate::gen::quasi::*;
  use crate::dist::normal::polar::Polar;
  use crate::dist::normal::acklam::InverseCumulative;
  use crate::dist::*;
  use crate::dist::continuous::Normal;

  #[test]
  fn normal_moments() {
    let n = 1000000;
    let err_p = 3.0 / (n as f64).sqrt();
    // TODO Sobol convergence doesn't look right
    let err_q = err_p;
    let mp = Moments::new(&Normal::<Polar<MT19937>>::new(0.0, 1.0, MT19937::new(Some(19937))).sample_n(n));
    println!("{:?}", mp);
    assert!(mp.mean.abs() < err_p);
    assert!((mp.variance - 1.0).abs() < err_p);
    assert!(mp.skew.abs() < err_p);
    assert!((mp.kurtosis - 3.0).abs() < 5.0 * err_p);
    let mq = Moments::new(&Normal::<InverseCumulative<Sobol>>::new(0.0, 1.0, Sobol::new(100)).sample_n(n));
    println!("{:?}", mq);
    assert!(mq.mean.abs() < err_q);
    assert!((mq.variance - 1.0).abs() < err_q);
    assert!(mq.skew.abs() < err_q);
    assert!((mq.kurtosis - 3.0).abs() < 10.0 * err_q);
  }
}