use rand::gen::*;
use rand::gen::pseudo::*;
use rand::gen::quasi::*;
use rand::dist::*;
use rand::dist::discrete::*;
use rand::dist::continuous::*;
use rand::dist::normal::*;
use number::Number; //::{R,C,Inf};
use number::{abs, sqrt, ln};

fn main() {
  let mut rng = LCG::new(None);
  let mut dist = WithoutReplacement::new(&[(1,2),(2,2),(3,2),(4,2),(5,2),(6,2)]);
  
  while !dist.empty() {
    print!("{} ", dist.sample_1(&mut rng));
  }
  println!();
  
  let mut rng = Sobol::new(8);
  println!("{:?}", rng.next_n(8));

  let mut rng = MT19937::new(None);
  let mut dist = Normal::<Polar>::new(5.0, 1.0);
  println!("{:?}", dist.sample_n(10, &mut rng));
  
  let mut normdist = Normal::<InverseCumulative>::new(0.0, 1.0);
  let mut rng = MT19937::new(None);
  let v = normdist.sample_n(100, &mut rng);

  let x/*: Number<f64>*/ = -1.0; 
  let z = sqrt(x);
  println!("{:?}", z)
  
}
