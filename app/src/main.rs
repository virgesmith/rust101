use rand::gen::*;
use rand::dist::*;

fn main() {
  let mut rng = LCG::new();
  let dist = Exponential::new(2.0);
  println!("{}", dist.sample_1(&mut rng));
}
