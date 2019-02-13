#[allow(dead_code)]
fn min_value(mut digits: Vec<i32>) -> i32  {
    digits.sort(); 
    digits.dedup();
    //digits.iter().map(|n| n.to_string())
    let mut s: i32 = 0;
    for i in 0..digits.len() {
      s += digits[i] * 10_i32.pow((digits.len() - 1 - i) as u32);
    }
    s
}

#[allow(dead_code)]
fn solution(word: &str, ending: &str) -> bool {
  word.contains(ending) && &word[word.len() - ending.len()..word.len()] == ending
}

static GVALS: &'static [i32] = &[1,2,3,3,4,10];
static EVALS: &'static [i32] = &[1,2,2,2,3,5,10];

#[allow(dead_code)]
fn good_vs_evil(good: &str, evil: &str) -> String {

  let gvec: Vec<i32> = good.split(' ').map(|s| s.parse().unwrap()).collect();
  assert!(gvec.len() == 6);
  let mut gval = 0;
  // TODO more functional way
  for i in 0..6 {
    gval += GVALS[i] * gvec[i];
  }
  let evec: Vec<i32> = evil.split(' ').map(|s| s.parse().unwrap()).collect(); 
  assert!(evec.len() == 7);
  let mut eval = 0;
  for i in 0..7 {
    eval += EVALS[i] * evec[i];
  }
  println!("{:?}\n{:?}", gvec, evec);
  if gval > eval {
    String::from("Battle Result: Good triumphs over Evil")
  } else if gval < eval {
    String::from("Battle Result: Evil eradicates all trace of Good")
  } else {
    String::from("Battle Result: No victor on this battle field")
  }
}


#[allow(dead_code)]
fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
  match (h, bounce, window) {
    (h, _, _) if h < 0.0 => -1,
    (_, b, _) if b <= 0.0 || b >= 1.0 => -1,
    (h, _, w) if h <= w => -1,
    (h, b, w) => 1 + 2 * ((w / h).ln() / b.ln()) as i32
  }
}

#[allow(dead_code)]
fn max_number(n: u32) -> u32 {
  let mut digits: Vec<char> = n.to_string().chars().collect();
  digits.sort_by(|a, b| b.cmp(a));
  digits.into_iter().collect::<String>().parse::<u32>().unwrap()
}

#[allow(dead_code)]
fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
  (cell1.as_bytes()[0] + cell1.as_bytes()[1] 
  + cell2.as_bytes()[0] + cell2.as_bytes()[1]) % 2 == 0
}

#[allow(dead_code)]
fn remove_nb(m: i32) -> Vec<(i32, i32)> {
  let mf = m as f64;
  let n = mf * (mf + 1.) / 2.;

  // dont need to check all of 1..m, can start at l
  let l = ((n - mf) / (1.0 + mf)) as i32 + 1;

  let mut v = Vec::new();
  println!("{} {}", l, m);
  for i in l..m {
    let j = (n - i as f64) / (1.0 + i as f64);
    if j.fract() == 0.0 && i < m {
      v.push((i, j as i32));
    }
  }
  v
}

// advance to the next prime for sieving
fn next(idx: usize, p: &Vec<bool>) -> Option<usize> {
  for i in idx+1..p.len() {
    if p[i] { return Some(i); }
  }
  None    
}

fn sieve(n: usize) -> Vec<bool> {
  let mut p = vec![true; n];
  p[0] = false;
  p[1] = false;

  let s = (n as f64).sqrt() as usize + 1;   

  let mut i: usize = 2;
  while i < s {
    let mut c: usize = i * 2;
    while c < n {
      p[c] = false;
      c += i;
    } 
    i = next(i, &p).unwrap();     
  }
  return p;
}

#[allow(dead_code)]
fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
  let g = g as usize;
  let m = m as usize;
  let n = n as usize;
  let isprime = sieve(n + 1);

  let mut p0 = next(m-1, &isprime)?;
  for _ in m..n+1 {
    let mut p1 = next(p0, &isprime)?;
    if p0 + g == p1 { 
      return Some((p0 as u64, p1 as u64));
    }
    p0 = p1;
  }
  None
}

// see https://math.stackexchange.com/questions/397689/why-convolution-regularize-functions/398146#398146

fn die(k: i32, s: i32) -> i32
{
  match k {
    k if k > s || k < 1 => 0,
    _ => 1
  }
}

fn dice(k: i32, n: i32, s: i32) -> i32 {
  if n == 1 {
    die(k, s)
  } else {
    let mut sum: i32 = 0;
    for j in 1..s+1 {
      sum += dice(k - j, n-1, s) * die(j, s);  
    }
    sum
  }
}

#[allow(dead_code)]
fn prob(k: i32, n: i32) -> f64 {
  dice(k, n, 6) as f64 / 6.0f64.powi(n)
}

fn flip(d: char) -> Option<char> {
  match d {
    '0' => Some('1'),
    '1' => Some('0'),
    _ => None
  }
}

#[allow(dead_code)]
fn interpreter(tape: &str, data: &str) -> String {
  let tape: Vec<char> = tape.chars().collect();
  let mut data: Vec<char> = data.chars().collect();
  let mut dp = 0;
  let mut done = false;
  while !done {
    for ip in 0..tape.len() {
      match tape[ip] {
        '0' => dp += 1,
        '1' => data[dp] = flip(data[dp]).unwrap(),
        _ => panic!() 
      };
      // need to check done at every iteration in loop and break immediately
      if dp == data.len() { 
        done = true;
        break; 
      }
    }
  }
  data.into_iter().collect::<String>()
}

#[allow(dead_code)]
fn fib(n: usize) -> u64 {
  let mut w: [u64;2] = [0,1];
  // keep running sum of previous 2
  for i in 2..n+1 {
    w[i%2] += w[(i+1)%2];
  }
  w[n%2]
}

//let PHI = 0.5 * (1.0 + 5.0f64.sqrt());

#[allow(dead_code)]
fn product_fib(prod: u64) -> (u64, u64, bool) {
  let mut w: [u64;2] = [0,1];
  let mut i = 0;
  while w[0] * w[1] < prod {
    w[i%2] += w[(i+1)%2];
    i += 1;
  }
  (w[i%2], w[(i+1)%2], w[0] * w[1] == prod)
}

#[allow(dead_code)]
fn sum_fib(n: usize) -> u64 {
  let mut w: [u64;2] = [0,1];
  let mut s = 0;
  for i in 1..n+2 {
    w[i%2] += w[(i+1)%2];
    s += w[i%2];
    //println!("{:?} {}", w, s);
  }
  s * 4
}

#[allow(dead_code)]
fn dbl_linear(n: u32) -> u32 {
  let mut u = vec![1];
  let mut ix = 0;
  let mut iy = 0;
  loop {
    let x = 2 * u[ix] + 1;
    let y = 3 * u[iy] + 1;
    if x <= y { 
      if *u.last().unwrap() != x { u.push(x); }
      ix += 1;
    } else {
      if *u.last().unwrap() != y { u.push(y) };
      iy += 1;
    }
    if u.len() > n as usize { break; }
  }
  *u.last().unwrap()
}


fn getmin(v: &Vec<u32>) -> usize
{
  let mut minval = std::u32::MAX;
  let mut mindex = 0;
  for (i, val) in v.iter().enumerate() {
    if *val < minval {
      minval = *val;
      mindex = i;
    }
  }
  mindex
}

#[allow(dead_code)]
fn n_linear(m: &[u32], n: usize) -> u32 {
  let mut u = vec![1];
  let mut idx = vec![0; m.len()];
  let mut v: Vec<u32> = m.iter().map(|x| 1 + x).collect();
  loop {
    if u.len() > n as usize { break; }
    let midx = getmin(&v);
    if *u.last().unwrap() != v[midx] { u.push(v[midx]); }
    idx[midx] += 1;
    v[midx] = m[midx] * u[idx[midx]] + 1;
  }
  *u.last().unwrap()
}


// fn bisect(m: f64) -> f64 {
//   let mut xl = TOL;
//   let mut xh = 1.0 - TOL.sqrt();
//   let mut xm;
//   let fl = f(xl) - m;
//   let fh = f(xh) - m;

//   (fm - fl) * (xh - xl) = (fh - fl) * (xm - xl) 
// }
  // let mut xl = TOL;
  // let mut xh = 1.0 - TOL.sqrt();
  // let mut xm;
  // loop {
  //   let fl = f(xl) - m;
  //   let fh = f(xh) - m;
  //   assert!(fl * fh < 0.0);
  //   xm = 0.5 * (xl + xh);
  //   let fm = f(xm) - m;
  //   if fm.abs() < TOL { break; }
  //   // if fm same sign as fl, fl<-fm 
  //   if fm * fl > 0.0 {
  //     xl = xm;
  //   } else {
  //     xh = xm;  
  //   }
  //   //println!("{} {} {}", xl, xm, xh);
  // }
  // xm
const TOL: f64 = 1e-12;

fn f(x: f64) -> (f64, f64) {
  assert!(x > 0.0 && x < 1.0);
  let mut t = x;
  let mut a = 1.0;
  let mut sum = a * t;
  let mut dsum = 1.0;
  loop {
    a += 1.0;
    dsum += a * a * t;
    t *= x;
    sum += a * t;
    if a * t < TOL { break; }
  }
  (sum, dsum)
}

// Newton(-Raphson)
#[allow(dead_code)]
fn solve(m: f64) -> f64 {
  let mut x = 0.5;
  let mut xn;
  loop 
  {
    let (f, fprime) = f(x);
    xn = x - (f - m) / fprime;
    if (xn - x).abs() < TOL { break; }
    //println!("{} {}: {} -> {}", f, fprime, x, xn);
    x = xn;
    if x > 0.9999 { x = 0.9999; }
  }
  xn
}


fn solequa(n: u64) -> Vec<(u64, u64)> {
  let mut v = Vec::new();
  let k = (n as f64).sqrt() as u64;
  for a in 1..=k {
    let b = n as f64 / a as f64;
    if b.fract() == 0.0 {
      let b = b as u64;
      if (a + b) % 2 == 0 && (b - a) % 4 == 0 {
        v.push(((a+b)/2, (b-a)/4));
      }
    }
  }
  v
}


fn testing(n: u64, e: Vec<(u64, u64)>) -> () {
  println!("{}: {:?} {:?}", n, solequa(n), e);
}

fn main() {
  testing(5, vec![(3, 1)]);
  testing(20, vec![(6, 2)]); 
  testing(9001, vec![(4501, 2250)]);
  testing(9004, vec![(2252, 1125)]);
  testing(90005, vec![(45003, 22501), (9003, 4499), (981, 467), (309, 37)]);
  testing(90002, vec![]);
}
