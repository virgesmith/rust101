#[derive(Debug, PartialEq, Eq)]
enum List<T: Clone> {
  Node(T, Box<List<T>>),
  End
}

// use std::iter::{self, Sum};
// use std::ops::Add;
// use std::Zero;

impl<T: Clone> List<T> {
  pub fn new(head: T, tail: Self) -> Self {
    List::Node(head, Box::new(tail))
  }

  pub fn to_vec(&self) -> Vec<T> {
    match self {
      &List::End => vec![],
      &List::Node(ref head, ref tail) => {
        let mut head = vec![head.clone()];
        head.extend(tail.to_vec());
        head
      }
    }
  }

  pub fn from_iter<I>(it: I) -> Self
    where I: IntoIterator<Item=T>
  {
    let mut iter = it.into_iter();
    match iter.next() {
      Some(x) => List::new(x, List::from_iter(iter)),
      None => List::End      
    }
  }

  pub fn filter<F>(&self, fun: F) -> Self
    where F: Fn(&T) -> bool
  {
    match self {
      &List::End => List::End,
      &List::Node(ref head, ref tail) => {
        match fun(head) {
          true => List::new(head.clone(), tail.filter(fun)),
          false => tail.filter(fun)
        }
      }
    }
  }

  pub fn map<F,S>(&self, fun: F) -> List<S>
    where F: Fn(T) -> S, S: Clone
  {
    match self {
      &List::End => List::End,
      &List::Node(ref head, ref tail) => {
        List::new(fun(head.clone()), tail.map(fun))
      }
    }
  }

  // //pub fn sum<S>(&self) -> S where S: Sum<Self::Item> {
  // fn sum<S>(&self) -> S 
  //   where S: Add<Self::Item, Output=S> + Zero, Self: Sized 
  // {
  //   match self {
  //     &Cons::Null => S::sum(0),
  //     &Cons::Cons(ref head, ref tail) => S::sum(head.clone() + tail.sum())
  //   }
  // }
}

fn main() {
  let numbers = List::new(1, List::new(2, List::new(3, List::new(4, List::new(5, List::End)))));
  println!("{:?}", numbers); 
  println!("{:?}", numbers.to_vec()); 
  println!("{:?}", List::from_iter(vec!["1","2","3","4","5"])); 

  println!("{:?}", numbers.filter(|x| x % 2 == 0).to_vec());  // yields [2,4]
  println!("{:?}", numbers.map(|x| x * x).to_vec());  // yields [1,4,9,16,25]
}