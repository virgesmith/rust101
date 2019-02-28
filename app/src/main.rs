use rand::gen::*;
use rand::dist::*;

fn main() {
  let mut rng = LCG::new();
  let mut dist = WithoutReplacement::new(&[(1,2),(2,2),(3,2),(4,2),(5,2),(6,2)]);
  
  while !dist.empty() {
    println!("{}", dist.sample_1(&mut rng));
  }
  
  let rng = Sobol::new(8);
  println!("{:?}", rng.next_d())
}
