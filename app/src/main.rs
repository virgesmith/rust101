use rand::gen::pseudo::*;
use rand::gen::quasi::*;
use rand::dist::*;
use number::Number; //::{R,C,Inf};
use number::{abs, sqrt, ln};

fn main() {
  let mut rng = LCG::new();
  let mut dist = WithoutReplacement::new(&[(1,2),(2,2),(3,2),(4,2),(5,2),(6,2)]);
  
  while !dist.empty() {
    print!("{} ", dist.sample_1(&mut rng));
  }
  println!();
  
  let rng = Sobol::new(8);
  println!("{:?}", rng.next_d());

  let mut rng = MT19937::new();
  let mut dist = Normal::new(5.0, 1.0);
  println!("{:?}", dist.sample_n(10, &mut rng));
  
  let mut normdist = Normal::new(0.0, 1.0);
  let mut rng = MT19937::new();
  let v = normdist.sample_n(100, &mut rng);

  let x/*: Number<f64>*/ = -1.0; 
  let z = sqrt(x);
  println!("{:?}", z)
  
}
