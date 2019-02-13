
use std::collections::HashSet;
//use std::collections::HashMap;

// fn intersection(v: &[char;3], w: &[char;3]) -> Vec<char> {
//   v.iter().cloned().filter(|x| w.contains(x)).collect::<Vec<char>>()
// }

fn indexof(v: &Vec<char>, c: char) -> Option<usize> {
  for i in 0..v.len() {
    if v[i] == c {
      return Some(i);
    }
  }
  // this shouldnt happen
  None
}

fn swap(v: &mut Vec<char>, i0: usize, i1: usize) {
  let tmp = v[i0];
  v[i0] = v[i1];
  v[i1] = tmp;
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {

  // first 
  let mut set = HashSet::new();
  //let mut map: HashMap<&char, i32> = HashMap::new();
  for t in &triplets {
    set.insert(t[0]);
    set.insert(t[1]);
    set.insert(t[2]);
    //map[t[0]] += 1; //map[t[0]]
    //println!("{:?}", intersection(t, &triplets[0]));
  }  
  // need cloned() otherwise Vec value_type will be &char
  let mut v = set.iter().cloned().collect::<Vec<_>>();
  //println!("{:?}", v);

  loop { 
    let mut swapped = false;
    for t in &triplets {
      let p0 = indexof(&v, t[0]).unwrap();
      let p1 = indexof(&v, t[1]).unwrap();
      let p2 = indexof(&v, t[2]).unwrap();
      if p0 > p1 {
        swap(&mut v, p0, p1);
        swapped = true;
      }
      if p1 > p2 {
        swap(&mut v, p1, p2);
        swapped = true;
      }
    }
    println!("{}", v.iter().collect::<String>());
    if !swapped { break; }
  }

  v.into_iter().collect()
}

fn example_test() {
  assert_eq!(recover_secret(vec![ 
      ['t','u','p'],
      ['w','h','i'],
      ['t','s','u'],
      ['a','t','s'],
      ['h','a','p'],
      ['t','i','s'],
      ['w','h','s']])
    , "whatisup");
  assert_eq!(recover_secret(vec![ 
      ['a','b','c'],
      ['c','d','e'],
      ['e','f','g'],
      ['g','h','i'],
      ['i','j','k'],
      ['k','l','m'],
      ['m','n','o'],
      ['o','p','q'],
      ['q','r','s'],
      ['p','q','r'],
      ['s','t','u'],
      ['a','c','d'],
      ['e','l','u'],
      ['f','u','v'],
      ['g','j','w'],
      ['v','w','x'],
      ['x','y','z'],
      ['b','g','z'],
      ['c','g','z'],
      ['e','f','g'],
      ['h','i','x']])
    , "abcdefghijklmnopqrstuvwxyz");
}

fn main() {
  example_test();
}