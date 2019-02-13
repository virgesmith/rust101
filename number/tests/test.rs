#[cfg(test)]

mod tests {
  use number::Number::{R,C,Inf};
  //::{R,C,Inf};

  #[test]
  fn test_eq() {
    let r = R(1.0);
    assert_eq!(r, R(1.0));
    assert_eq!(r, C{r:1.0, i:0.0});
   // assert_eq!(r, 1.0);
  }

}