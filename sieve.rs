// 

// advance to the next prime for sieving
fn next(mut i: usize, p: &Vec<bool>) -> usize {
  loop {
    i += 1;
    if i >= p.len() || p[i] { return i; }
  }    
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
    i = next(i, &p);     
  }
  return p;
}

fn help() {
    println!("usage: sieve <m> <n>");
    println!("returns primes in [m,n)");
    println!("requires m,n>0 and m<n");
    // use this to return nonzero to OS
    std::process::exit(1);
}

fn main() {

  let args: Vec<String> = std::env::args().collect();

  if args.len() != 3 {
    help();
  } 

  let m = args[1].parse::<usize>().unwrap();
  let n = args[2].parse::<usize>().unwrap();

  if n <= m {
    help();
  }

  let isprime = sieve(n);
  let mut primes = Vec::new();

  // reserve space based on num primes in [0,n] ~= n/ln(n)
  let cap = ((n as f64 / (n as f64).ln() - (m as f64 / (m as f64).ln())) * 1.15) as usize + 1;
  primes.reserve(cap);

  for (i,p) in isprime.iter().enumerate() {
    if i >= m && *p { primes.push(i); }
  }
  println!("reserved for {}, got {} primes", cap, primes.len());
  println!("{:?}", primes);
  // by default returns 0 to OS
}
