#[cfg(test)]

mod tests {
  use number::Number::{R,C,Inf};
  use number::{sqrt, ln};
  //::{R,C,Inf};

  #[test]
  fn test_eq() {
    let r = R(1.0);
    assert_eq!(r, R(1.0));
    //assert_eq!(r, C{r:1.0, i:0.0});
   // assert_eq!(r, 1.0);
  }

  #[test]
  fn main() {
  //   println!("{:?}", abs(0));
  //   println!("{:?}", abs(10));
  //   println!("{:?}", abs(-10));
  //   println!("{:?}", abs(-128));

    assert_eq!(R(8.0).re(), 8.0);
    assert_eq!(R(8.0).im(), 0.0);
    assert_eq!(C{r:0.0, i:8.0}.re(), 0.0);
    assert_eq!(C{r:0.0, i:8.0}.im(), 8.0);

    assert_eq!(sqrt( 64.), R(8.0));
    assert_eq!(sqrt(-64.), C{r:0.0, i:8.0});

    assert_eq!(ln(1.0), R(0.0));
    assert_eq!(ln(0.0), Inf(true));
    assert_eq!(ln( std::f64::consts::E), R(1.0));
    assert_eq!(ln(-std::f64::consts::E), C{r: 1.0, i: std::f64::consts::PI});
  
    //let x: f64 = ln(-1.0); // compile-time error!

    let x: f64 = ln(0.1).re();
    assert_eq!(x, 0.1f64.ln());
    assert!(ln(x).R().is_err());
    assert!(!ln(x).C().is_err()); 
    //assert_eq!(x, ) 
  //   let z/*: Cplx<f64>*/ = ln(x);
  //   println!("{:?}", z);

  //   // NOTE ordering: NOT Number::<f64>::R
  //   let a = Number::R::<f64>;

  //   let _ = a;

  //   println!("{:?}", f(1.0, 1.0).unwrap());
  //   // println!("{:?}", f(-1.0, 1.0).unwrap());
  //   // println!("{:?}", f(1.0, 0.0).unwrap());
  //   match f(-1.0, 1.0) {
  //     Ok(x) => println!("{}", x),
  //     Err(e) => println!("{}", e)
  //   }
  //   match f(1.0, 0.0) {
  //     Ok(x) => println!("{}", x),
  //     Err(e) => println!("{}", e)
  //   }
  //   match f(1.0, 1.0e-308) {
  //     Ok(x) => println!("{}", x),
  //     Err(e) => println!("{}", e)
  //   }
  }
}
