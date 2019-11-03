use rand::gen::*;
use rand::gen::pseudo::*;
use rand::gen::quasi::*;
use rand::dist::*;
use rand::dist::discrete::*;
use rand::dist::continuous::*;
use rand::dist::normal::*;
use rand::dist::ziggurat::*;
//use number::Number; //::{R,C,Inf};
use number::{sqrt};

fn main() {
  let mut rng = LCG::new(None);
  let mut dist = WithoutReplacement::new(&[(1,2),(2,2),(3,2),(4,2),(5,2),(6,2)], rng);
  
  while !dist.empty() {
    print!("{:?} ", dist.sample_n(2));
  }
  println!();
  
  let mut rng = Sobol::new(8);
  println!("{:?}", rng.next_n(8));
  println!("{:?}", rng.next_n(8));

  let mut dist = Normal::<Polar<MT19937>>::new(0.0, 1.0, MT19937::new(None));
  println!("{:?}", dist.sample_n(10));
  
  let mut normdist = Normal::<InverseCumulative<MT19937>>::new(0.0, 1.0, MT19937::new(None));
  println!("{:?}", dist.sample_n(10));

  let mut normdist = Ziggurat::<MT19937>::new(MT19937::new(None));
  let v: f64 = normdist.get_n(1000).into_iter().sum();
  println!("{:?}", v);

  let x/*: Number<f64>*/ = -1.0; 
  let z = sqrt(x);
  println!("{:?}", z) 
}
