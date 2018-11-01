//! Simple easing functions
//!
//! All functions have the signature `fn<F: Float>(F) -> F` (letting you use `f32`, `f64`, or any other type that implements [`num_traits`'s `Float`](https://docs.rs/num-traits/0.2.0/num_traits/float/trait.Float.html)). Input should range from `0.0` to `1.0`, and output is generally in the `0.0` to `1.0` range (except for `elastic` and `back`, which return values slightly outside). `0.0` always maps to `0.0`, and `1.0` always maps to `1.0`.

#![cfg_attr(test, feature(concat_idents))]

extern crate num_traits;
use num_traits::Float;

use std::f64::consts::{PI,FRAC_PI_2};

fn lit<F: Float>(f: f64) -> F {
  F::from(f).unwrap()
}

// Linear

#[inline]
pub fn linear<F: Float>(t: F) -> F {
  t
}

// Quadratic

#[inline]
pub fn quad_in<F: Float>(t: F) -> F {
  t * t
}

#[inline]
pub fn quad_out<F: Float>(t: F) -> F {
  -t * (t - lit::<F>(2.0))
}

#[inline]
pub fn quad_inout<F: Float>(t: F) -> F {
  if t < lit::<F>(0.5) {
    lit::<F>(2.0) * t * t
  } else {
    (lit::<F>(-2.0) * t * t) + (lit::<F>(4.0) * t) - lit::<F>(1.0)
  }
}

// Cubic

#[inline]
pub fn cubic_in<F: Float>(t: F) -> F {
  t * t * t
}

#[inline]
pub fn cubic_out<F: Float>(t: F) -> F {
  let f = t - lit::<F>(1.0);
  f * f * f + lit::<F>(1.0)
}

#[inline]
pub fn cubic_inout<F: Float>(t: F) -> F {
  if t < lit::<F>(0.5) {
    lit::<F>(4.0) * t * t * t
  } else {
    let f = (lit::<F>(2.0) * t) - lit::<F>(2.0);
    lit::<F>(0.5) * f * f * f + lit::<F>(1.0)
  }
}

// Quartic

#[inline]
pub fn quart_in<F: Float>(t: F) -> F {
  t * t * t * t
}

#[inline]
pub fn quart_out<F: Float>(t: F) -> F {
  let f = t - lit::<F>(1.0);
  f * f * f * (lit::<F>(1.0) - t) + lit::<F>(1.0)
}

#[inline]
pub fn quart_inout<F: Float>(t: F) -> F {
  if t < lit::<F>(0.5) {
    lit::<F>(8.0) * t * t * t * t
  } else {
    let f = t - lit::<F>(1.0);
    lit::<F>(-8.0) * f * f * f * f + lit::<F>(1.0)
  }
}

// Quintic

#[inline]
pub fn quint_in<F: Float>(t: F) -> F {
  t * t * t * t * t
}

#[inline]
pub fn quint_out<F: Float>(t: F) -> F {
  let f = t - lit::<F>(1.0);
  f * f * f * f * f + lit::<F>(1.0)
}

#[inline]
pub fn quint_inout<F: Float>(t: F) -> F {
  if t < lit::<F>(0.5) {
    lit::<F>(16.0) * t * t * t * t * t
  } else {
    let f = (lit::<F>(2.0) * t) - lit::<F>(2.0);
    lit::<F>(0.5) * f * f * f * f * f + lit::<F>(1.0)
  }
}

// Sine

#[inline]
pub fn sine_in<F: Float>(t: F) -> F {
  ((t - lit::<F>(1.0)) * lit::<F>(FRAC_PI_2)).sin() + lit::<F>(1.0)
}

#[inline]
pub fn sine_out<F: Float>(t: F) -> F {
  (t * lit::<F>(FRAC_PI_2)).sin()
}

#[inline]
pub fn sine_inout<F: Float>(t: F) -> F {
  lit::<F>(0.5) * (lit::<F>(1.0) - (t * lit::<F>(PI)).cos())
}

// Circular

#[inline]
pub fn circ_in<F: Float>(t: F) -> F {
  lit::<F>(1.0) - (lit::<F>(1.0) - t * t).sqrt()
}

#[inline]
pub fn circ_out<F: Float>(t: F) -> F {
  ((lit::<F>(2.0) - t) * t).sqrt()
}

#[inline]
pub fn circ_inout<F: Float>(t: F) -> F {
  if t < lit::<F>(0.5) {
    lit::<F>(0.5) * (lit::<F>(1.0) - (lit::<F>(1.0) - lit::<F>(4.0) * t * t).sqrt())
  } else {
    lit::<F>(0.5) * ((-(lit::<F>(2.0) * t - lit::<F>(3.0)) * (lit::<F>(2.0) * t - lit::<F>(1.0))).sqrt() + lit::<F>(1.0))
  }
}

// Exponential

#[inline]
pub fn expo_in<F: Float>(t: F) -> F {
  if t == lit::<F>(0.0) {
    lit::<F>(0.0)
  } else {
    lit::<F>(2.0).powf(lit::<F>(10.0) * (t - lit::<F>(1.0)))
  }
}


#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
#[inline]
pub fn expo_out<F: Float>(t: F) -> F {
  if t == lit::<F>(1.0) {
    lit::<F>(1.0)
  } else {
    lit::<F>(1.0) - lit::<F>(2.0).powf(lit::<F>(-10.0) * t)
  }
}

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
#[inline]
pub fn expo_inout<F: Float>(t: F) -> F {
  if t == lit::<F>(0.0) {
    lit::<F>(0.0)
  } else if t == lit::<F>(1.0) {
    lit::<F>(1.0)
  } else if t < lit::<F>(0.5) {
    lit::<F>(0.5) * lit::<F>(2.0).powf(lit::<F>(20.0) * t - lit::<F>(10.0))
  } else {
    lit::<F>(-0.5) * lit::<F>(2.0).powf(lit::<F>(-20.0) * t + lit::<F>(10.0)) + lit::<F>(1.0)
  }
}

// Elastic

#[inline]
pub fn elastic_in<F: Float>(t: F) -> F {
  (lit::<F>(13.0) * lit::<F>(FRAC_PI_2) * t).sin() * lit::<F>(2.0).powf(lit::<F>(10.0) * (t - lit::<F>(1.0)))
}

#[inline]
pub fn elastic_out<F: Float>(t: F) -> F {
  (lit::<F>(-13.0) * lit::<F>(FRAC_PI_2) * (t + lit::<F>(1.0))).sin() * lit::<F>(2.0).powf(lit::<F>(-10.0) * t) + lit::<F>(1.0)
}

#[inline]
pub fn elastic_inout<F: Float>(t: F) -> F {
  if t < lit::<F>(0.5) {
    lit::<F>(0.5) * (lit::<F>(13.0) * lit::<F>(FRAC_PI_2) * lit::<F>(2.0) * t).sin() * lit::<F>(2.0).powf(lit::<F>(10.0) * (lit::<F>(2.0) * t - lit::<F>(1.0)))
  } else {
    lit::<F>(0.5) * ((lit::<F>(-13.0) * lit::<F>(FRAC_PI_2) * lit::<F>(2.0) * t).sin() * (lit::<F>(2.0)).powf(lit::<F>(-10.0) * (lit::<F>(2.0) * t - lit::<F>(1.0))) + lit::<F>(2.0))
  }
}

// Back

#[inline]
pub fn back_in<F: Float>(t: F) -> F {
  t * t * t - t * (t * lit::<F>(PI)).sin()
}

#[inline]
pub fn back_out<F: Float>(t: F) -> F {
  let f = lit::<F>(1.0) - t;
  lit::<F>(1.0) - f * f * f + f * (f * lit::<F>(PI)).sin()
}

#[inline]
pub fn back_inout<F: Float>(t: F) -> F {
  if t < lit::<F>(0.5) {
    let f = lit::<F>(2.0) * t;
    lit::<F>(0.5) * (f * f * f - f * (f * lit::<F>(PI)).sin())
  } else {
    let f = lit::<F>(2.0) - lit::<F>(2.0) * t;
    lit::<F>(0.5) * (lit::<F>(1.0) - (f * f * f - f * (f * lit::<F>(PI)).sin())) + lit::<F>(0.5)
  }
}

// Bounce

#[inline]
pub fn bounce_in<F: Float>(t: F) -> F {
  lit::<F>(1.0) - bounce_out(lit::<F>(1.0) - t)
}

#[inline]
pub fn bounce_out<F: Float>(t: F) -> F {
  if t < lit::<F>(4.0 / 11.0) {
    lit::<F>(121.0 / 16.0) * t * t
  } else if t < lit::<F>(8.0 / 11.0) {
    lit::<F>(363.0 / 40.0) * t * t - lit::<F>(99.0 / 10.0) * t + lit::<F>(17.0 / 5.0)
  } else if t < lit::<F>(9.0 / 10.0) {
    lit::<F>(4356.0 / 361.0) * t * t - lit::<F>(35442.0 / 1805.0) * t + lit::<F>(16061.0 / 1805.0)
  } else {
    lit::<F>(54.0 / 5.0) * t * t - lit::<F>(513.0 / 25.0) * t + lit::<F>(268.0 / 25.0)
  }
}

#[inline]
pub fn bounce_inout<F: Float>(t: F) -> F {
  if t < lit::<F>(0.5) {
    lit::<F>(0.5) * bounce_in(t * lit::<F>(2.0))
  } else {
    lit::<F>(0.5) * bounce_out(t * lit::<F>(2.0) - lit::<F>(1.0)) + lit::<F>(0.5)
  }
}


#[cfg(test)]
mod tests {
  use num_traits::Float;
  use super::lit;

  fn assert_float_eq<A: Float, B: Float>(a: A, b: B) {
    let a = a.to_f64().unwrap();
    let b = b.to_f64().unwrap();

    assert!(a - b < 0.00001, "a = {}, b = {}", a, b);
  }

  macro_rules! tests {
    ($name:ident) => {
      mod $name{
        use super::*;
        use super::super::*;

        #[test] fn in32() { test::<f32, _>(concat_idents!($name, _in)); }
        #[test] fn in64() { test::<f64, _>(concat_idents!($name, _in)); }
        #[test] fn out32() { test::<f32, _>(concat_idents!($name, _out)); }
        #[test] fn out64() { test::<f64, _>(concat_idents!($name, _out)); }
        #[test] fn inout32() { test::<f32, _>(concat_idents!($name, _inout)); }
        #[test] fn inout64() { test::<f64, _>(concat_idents!($name, _inout)); }
        #[test] fn trio32() { test_trio::<f32, _, _, _>(concat_idents!($name, _in),
                                                        concat_idents!($name, _out),
                                                        concat_idents!($name, _inout)); }
        #[test] fn trio64() { test_trio::<f64, _, _, _>(concat_idents!($name, _in),
                                                        concat_idents!($name, _out),
                                                        concat_idents!($name, _inout)); }
      }
    }
  }

  fn test<T: Float, F>(f: F)
    where F: Fn(T) -> T {
    assert_float_eq(f(lit::<T>(0.0)), 0.0);
    assert_float_eq(f(lit::<T>(1.0)), 1.0);
  }

  fn test_trio<T: Float, FIN, FOUT, FINOUT>(fin: FIN, fout: FOUT, finout: FINOUT)
    where FIN: Fn(T) -> T,
          FOUT: Fn(T) -> T,
          FINOUT: Fn(T) -> T {

    let n = 99;

    for i in 0..n+1 {
      let t = T::from(i).unwrap() / T::from(n).unwrap();
      println!("{}", t.to_f32().unwrap());

      assert_float_eq::<T, T>(fin(t), lit::<T>(1.0) - (fout(lit::<T>(1.0) - t)));

      if t < lit(0.5) {
        assert_float_eq(finout(t), fin(t*lit::<T>(2.0)) / lit::<T>(2.0));
      } else {
        assert_float_eq(finout(t), fout((t-lit::<T>(0.5))*lit::<T>(2.0)) / lit::<T>(2.0) + lit::<T>(0.5));
      }
    }
  }

  tests!(quad);
  tests!(cubic);
  tests!(quart);
  tests!(quint);
  tests!(sine);
  tests!(circ);
  tests!(expo);
  tests!(elastic);
  tests!(back);
  tests!(bounce);
  #[test]
  fn test_linear() {
    test::<f32, _>(super::linear);
    test::<f64, _>(super::linear);
  }
}
