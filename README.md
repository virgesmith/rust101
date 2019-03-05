
[![Travis Build Status](https://travis-ci.org/virgesmith/rust101.svg?branch=master)](https://travis-ci.org/virgesmith/rust101)

# rust 101
Learning rust...(work in progress) with the help of [codewars](https://www.codewars.com):

![https://www.codewars.com/users/virgesmith/badges/micro](https://www.codewars.com/users/virgesmith/badges/micro)

I've a C++ background (high-performance numerical computing) and Rust sounded interesting as it claims to prevent all(?) UB and enforces rules at compile time that I wish I could implement in C++, and expressly doesn't use a garbage collector. I'm extremely curious to see what runtime performance penalty (if any) all of this incurs.

#### Disclaimers
- spoiler alert: this repo contains solutions to some codewars Kata
- none of the code here is intended to be production quality. 

### Resources
- https://doc.rust-lang.org/

## Rectangle
This is basically a few attempts at shoehorning C++ ways of doing things (classes, templates) into rust. 

## Vanity
A port of some C++ code I wrote to generate bitcoin vanity addresses. A nice simple problem for concurrency. Threads only need to talk when one of them has found the answer.

It's actually running faster than my C++ implementation, given enough threads. (There seems to be some thread blocking issue with openssl in my C++ implementation that means you don't see any performance improvement by increasing the number of threads - I 'fixed' it by using MPI)

## Number
Messing about with
I'm new to algebraic enumerations and I like them! A lot.

If you think the (signed) integer absolute value function `int abs(int)` is safe (in terms of having well-defined output for any input) you'd be wrong!
The way two's complement works means there's one more negative integer than positive: with 8 bits that means the range of values is -128..127. So `abs(-128)` return value is outside its domain. 

### How would you solve this in C++?
Adding a runtime check
```cpp
int myabs(int x)
{
  if (x == std::numeric_limits<int>::min())
    throw runtime_error("integer overflow");
  return x < 0 ? -x : x;
}
```
will kill performance - this function might get called *a lot*. You could tell the CPU/signal handler (structured exception handler in Windows terms) to raise an integer overflow exception, but beware: any 3rd party library you depend on might not function as advertised if you change these settings, and, you've no guarantee that any code that *calls* your code hasn't changed the settings itself, and/or might do so while your library is loaded. In practice (my experience at least) is you trap the exception at hardware level when you're running your *comprehensive set of regression tests*, but in production you just hope for the best...

Likewise, plenty of floating point operations can return numbers outside the real number domain (never mind outside the IEE754 specification), such as `ln(0.0)` and `sqrt(-1.0)`. Whilst the IEEE754 spec provides for infinity (and NaN), unless you check at runtime, these values will just permetate through your computations like a super-contagious virus unless you put in loads of performance-crippling runtime checks.

So, basically, you don't really solve it. You just either compromise performance, or hope that you have sufficient regression tests and sanity checks that cover (and continue to cover) production use cases.

### And so how does rust help?

Rust uses a functional programming concept called [algebraic enumerations](). Unlike C and C++ enumerations, which simply map a symbol to a value, e.g. `BLUE=0x0000FF` (simply allowing you to make your code clearer), these map the result of one or more input values to an output *type* (and value). So, for our `myabs` function we can use the `Result` enumeration that rust provides (the function either returns a valid value, or an error)

```rust
// repeating the rust definition of Result:
enum Result<T, E> {
  Ok(T),
  Err(E)
}
// so Ok contains a result of type T and Err one of E

// here we either return an 8-bit signed int, or an error string
fn myabs(x : i8) -> Result<i8, String> {
  match x {
    -128 => Err("overflow".to_string()),
    x if x < 0 => Ok(-x),
    _ => Ok(x)
  }
}
```
Similarly, if we wanted to permit our transcendental functions to operate on the entire real number line, we would have to allow for infinite (IEEE754 covers this already) and complex (IEEE754 uses NaN for this) results:

```rust
#[derive(Debug)]
enum Number<T> where T: Into<f64> {
  R(T),
  C{ r: T, i: T},
  Inf(bool) // sign bit, true means negative
}
```
where
- `T` must be castable to a double 
- there's only one infinity, unlike IEEE754 (but would be a simple modification to have positive and negative)

So for `sqrt` its either real or imaginary, depending on sign:

```rust
fn sqrt(x: f64) -> Number<f64> {
  match x {
    x if x < 0.0 => Number::C{ r:0.0, i: (-x).sqrt() },
    _ => Number::R(x.sqrt()) // sqrt is a "member"
  }
}
```
and for logarithm, the result is negative infinity for zero, complex for negative input, otherwise real.
```rust
fn ln(x: f64) -> Number<f64> {
  match x {
    x if x < 0.0 => Number::C{ r: (-x).ln(), i: std::f64::consts::PI },
    x if x == 0.0 => Number::Inf(),
    _ => Number::R(x.ln())
  }
}
```

### Is it better?

Clarity: yes, once you get your head round the concept.

Performance: I don't know yet! TODO Profile/disassembler comparisons of the C++ (above) and rust. Since the check must be a runtime one, I can't see how it could outperform a (clever) C++ implementation (one that doesn't use exceptions and the accompanying baggage).

Usability: not sure. I'm not experienced enough with rust. The returned type needs to be unwrapped, or math functions need to be provided that accept a Number<>? unwrapping into the wrong type will result in a panic.

## Server

A simple web server

## Linked List

A linked list implementation using algebraic enums, based on a codewars kata solution

## Vector

Another kata solution

## Rand

More reinventing the wheel to learn rust and how to integrate with C and C++. A random number library with C linkage. Generators:
- C++11 minstd implementation of an LCG generator
- 64-bit xor shift generator
- Mersenne twister (C++11 std lib implementation)
- Sobol quasirandom sequence generator (C implementation)
Distributions:
- Discrete (integer) uniform
- Discrete (integer) weighted
- Discrete (integer) without-replacement
- Continuous uniform
- Normal
- Exponential

