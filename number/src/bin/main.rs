

use number::*;

fn main() {
  println!("{:?}", Number::R(0.0));
  println!("{:?}", Number::C{r:0.0, i:0.0});
  println!("{:?}", Number::<f64>::Inf(false));

  println!("{:?}", ln(0.0));
  println!("{:?}", Number::C{r:0.0, i:0.0});
  println!("{:?}", Number::<f64>::Inf(true));

  let x = Number::from_real(1.0);
  let y = Number::from_real(2.0);
  let z = Number::from_complex(-1.0, 1.0);
  println!("{:?}", x + y);
  println!("{:?}", x + z);
  println!("{:?}", z + z);

}