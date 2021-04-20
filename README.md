
# rust 101

Learning rust...(work in progress) with the help of [codewars](https://www.codewars.com):

![https://www.codewars.com/users/virgesmith/badges/micro](https://www.codewars.com/users/virgesmith/badges/micro)

I've a C++ background (high-performance numerical computing) and Rust sounded interesting as it claims to prevent all(?) UB and enforces rules at compile time that I wish I could implement in C++, and expressly doesn't use a garbage collector. I'm extremely curious to see what runtime performance penalty (if any) all of this incurs.

#### Disclaimers

- spoiler alert: this repo contains solutions to some codewars Kata
- none of the code here is, or is intended to be production quality.

### Resources

- https://doc.rust-lang.org/

|*Contents*
|----------
|[complex](#complex)
|[linked-list](#linked-list)
|[rand](#rand)
|[shape](#shape)
|[vector](#vector)
|[webserver](#server)


|*Moved to dedicated repos*
|----------
|[crypto](https://github.com/virgesmith/crypto-rs)
|[pycrypto](https://github.com/virgesmith/pycrypto-rs)

## Complex

Reinventing the wheel to learn (mainly) how operator overloading works in rust. Which seem to be a little restrictive for noncommutative operations - the first argument must be your new type, e.g this doesn't seem (from what I understand) to be possible with the normal implementation

```rust
impl<T> Div<T> for Cplx<T>
where
  T: Into<f64> + Float + Copy,
{
  type Output = Cplx<T>;
  fn div(&self, rhs: T) -> Cplx<T> {
    Cplx {
      r: &self.r / rhs,
      i: &self.i / rhs,
    }
  }
}
```

I haven't yet figured out how to implement both orderings of noncommutative binary operators, so e.g. as a workaround I just implemented a `recip()` function rather than an overload:

```rust
  pub fn recip(&self) -> Cplx<T> {
    Cplx::from_normarg(T::one() / self.norm(), -self.arg())
  }
```

## Number

This was an attempt to understand algebraic enumerations... NB the code no longer closely reflects what follows.

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

Likewise, plenty of floating point operations can return numbers outside the real number domain (never mind outside the IEE754 specification), such as `ln(0.0)` and `sqrt(-1.0)`. Whilst the IEEE754 spec provides for infinity (and NaN), unless you implement performance-crippling runtime checks, these values will just permetate through your computations corrupting the results.

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
- infinity is a special case, like IEEE754 there are positive and negative variants

So for `sqrt` its either real or imaginary, depending on sign:

```rust
fn sqrt(x: f64) -> Number<f64> {
  match x {
    x if x < 0.0 => Number::C{ r:0.0, i: (-x).sqrt() },
    _ => Number::R(x.sqrt()) // sqrt is a "member"
  }
}
```

and for logarithms, the result* is negative infinity for zero, complex for negative input, otherwise real.

```rust
fn ln(x: f64) -> Number<f64> {
  match x {
    x if x < 0.0 => Number::C{ r: (-x).ln(), i: std::f64::consts::PI },
    x if x == 0.0 => Number::Inf(true),
    _ => Number::R(x.ln())
  }
}

```

* i = (2n+1).pi for all integer n, we just take the n=0 root.

## Rand

A random number library. More reinventing the wheel to learn rust, specifically:

- package structure and tests, documentation, and doctests
- how to integrate with C and C++
- using traits to define relationships (or lack thereof) between types
- iterators and functional constructs

The following generators are implemented:

- C++11 minstd implementation of an LCG generator
- 64-bit xor shift generator
- Mersenne twister (link to C++11 std lib implementation)
- Sobol quasirandom sequence generator (link to C implementation)
- "EntropySource": true(ish) random using /dev/urandom (/dev/random too slow)

Which implement one or more of the traits

- RandomStream: produces vectors of `u32` and `f64`
- Seeded: requires a seed for initialisation, defaults to current nanoseconds
- Dimensioned: has inherent dimension (i.e. Sobol)
- Dimensionless: can sample one at a time (i.e. not Sobol)
- Rejectable: variates can be dropped and randomness properties are retained (i.e. not Sobol)
- Resettable: can be reset to initial state (not EntropySource)

and the distributions:

- Discrete uniform
- Discrete weighted
- Discrete without-replacement
- Continuous uniform
- Normal, three variants:
  - Marsaglia's polar version of the Box-Muller algorithm,
  - Marsaglia's ziggurat algorithm,
  - Acklam's approximation to the inverse normal CDF
- Exponential (using inverse CDF)

...have different "trait bounds", the point being to structure the code so that it's not possible to combine invalid combinations of random streams and distribution algorithms, thus:

```rust
let mut dist = Normal::<InverseCumulative<Sobol>>::new(0.0, 1.0, Sobol::new(1));
```

is fine, whereas

```rust
let mut dist = Normal::<Polar<Sobol>>::new(0.0, 1.0, Sobol::new(1));
```

gives the (admittedly not entirely obvious) error

```
error[E0599]: no function or associated item named `new` found for type `dist::continuous::Normal<dist::normal::Polar<gen::quasi::Sobol>>` in the current scope
   --> src/dist/continuous.rs:210:44
    |
13  | pub struct Normal<T> {
    | -------------------- function or associated item `new` not found for this
...
210 |     let mut dist = Normal::<Polar<Sobol>>::new(0.0, 1.0, Sobol::new(1));
    |                    ------------------------^^^
    |                    |
    |                    function or associated item not found in `dist::continuous::Normal<dist::normal::Polar<gen::quasi::Sobol>>`
    |
    = note: the method `new` exists but the following trait bounds were not satisfied:
            `gen::quasi::Sobol : gen::Dimensionless`
            `gen::quasi::Sobol : gen::Rejectable`
```

the point being that the polar algorithm is a rejection algorithm and Sobol sequences require all variates to be used to preserve their statistical properties. Thus, `Sobol` doesn't implement the `Rejectable` trait which is made a requirement for `Polar`'s template parameter.

## Shape

Implementing polymorphism in C++ and rust

## Server

A simple web server

## Linked List

A linked list implementation using algebraic enums, based on a codewars kata solution. See [here](cons.rs).

## Vector

Another kata solution
