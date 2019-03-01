use rand::gen::*;
use rand::dist::*;

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
}
