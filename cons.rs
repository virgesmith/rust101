#[derive(Debug, PartialEq, Eq)]
enum Cons<T: Clone> {
  Cons(T, Box<Cons<T>>),
  Null
}

impl<T: Clone> Cons<T> {
  pub fn new(head: T, tail: Self) -> Self {
    Cons::Cons(head, Box::new(tail))
  }

  pub fn to_vec(&self) -> Vec<T> {
    match self {
      &Cons::Null => vec![],
      &Cons::Cons(ref head, ref tail) => {
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
      Some(x) => Cons::new(x, Cons::from_iter(iter)),
      None => Cons::Null      
    }
  }

  pub fn filter<F>(&self, fun: F) -> Self
    where F: Fn(&T) -> bool
  {
    match self {
      &Cons::Null => Cons::Null,
      &Cons::Cons(ref head, ref tail) => {
        match fun(head) {
          true => Cons::new(head.clone(), tail.filter(fun)),
          false => tail.filter(fun)
        }
      }
    }
  }

  pub fn map<F,S>(&self, fun: F) -> Cons<S>
    where F: Fn(T) -> S, S: Clone
  {
    match self {
      &Cons::Null => Cons::Null,
      &Cons::Cons(ref head, ref tail) => {
        Cons::new(fun(head.clone()), tail.map(fun))
      }
    }
  }
}

fn main() {
  let numbers = Cons::new(1, Cons::new(2, Cons::new(3, Cons::new(4, Cons::new(5, Cons::Null)))));
  println!("{:?}", numbers); 
  println!("{:?}", numbers.to_vec()); 
  println!("{:?}", Cons::from_iter(vec!["1","2","3","4","5"])); 

  println!("{:?}", numbers.filter(|x| x % 2 == 0).to_vec());  // yields [2,4]
  println!("{:?}", numbers.map(|x| x * x).to_vec());  // yields [1,4,9,16,25]

}