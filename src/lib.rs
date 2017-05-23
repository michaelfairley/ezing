//! Simple easing functions
//!
//! All functions have the signature `fn(f32) -> f32`.

#![cfg_attr(test, feature(concat_idents))]

#[cfg(test)]
#[macro_use]
extern crate approx;

use std::f32::consts::{PI,FRAC_PI_2};

// Quadratic

#[inline]
pub fn quad_in(t: f32) -> f32 {
  t * t
}

#[inline]
pub fn quad_out(t: f32) -> f32 {
  -t * (t - 2.0)
}

#[inline]
pub fn quad_inout(t: f32) -> f32 {
  if t < 0.5 {
    2.0 * t * t
  } else {
    (-2.0 * t * t) + (4.0 * t) - 1.0
  }
}

// Cubic

#[inline]
pub fn cubic_in(t: f32) -> f32 {
  t * t * t
}

#[inline]
pub fn cubic_out(t: f32) -> f32 {
  let f = t - 1.0;
  f * f * f + 1.0
}

#[inline]
pub fn cubic_inout(t: f32) -> f32 {
  if t < 0.5 {
    4.0 * t * t * t
  } else {
    let f = (2.0 * t) - 2.0;
    0.5 * f * f * f + 1.0
  }
}

// Quartic

#[inline]
pub fn quart_in(t: f32) -> f32 {
  t * t * t * t
}

#[inline]
pub fn quart_out(t: f32) -> f32 {
  let f = t - 1.0;
  f * f * f * (1.0 - t) + 1.0
}

#[inline]
pub fn quart_inout(t: f32) -> f32 {
  if t < 0.5 {
    8.0 * t * t * t * t
  } else {
    let f = t - 1.0;
    -8.0 * f * f * f * f + 1.0
  }
}

// Quintic

#[inline]
pub fn quint_in(t: f32) -> f32 {
  t * t * t * t * t
}

#[inline]
pub fn quint_out(t: f32) -> f32 {
  let f = t - 1.0;
  f * f * f * f * f + 1.0
}

#[inline]
pub fn quint_inout(t: f32) -> f32 {
  if t < 0.5 {
    16.0 * t * t * t * t * t
  } else {
    let f = (2.0 * t) - 2.0;
    0.5 * f * f * f * f * f + 1.0
  }
}

// Sine

#[inline]
pub fn sine_in(t: f32) -> f32 {
  ((t - 1.0) * FRAC_PI_2).sin() + 1.0
}

#[inline]
pub fn sine_out(t: f32) -> f32 {
  (t * FRAC_PI_2).sin()
}

#[inline]
pub fn sine_inout(t: f32) -> f32 {
  0.5 * (1.0 - (t * PI).cos())
}

// Circular

#[inline]
pub fn circ_in(t: f32) -> f32 {
  1.0 - (1.0 - t * t).sqrt()
}

#[inline]
pub fn circ_out(t: f32) -> f32 {
  ((2.0 - t) * t).sqrt()
}

#[inline]
pub fn circ_inout(t: f32) -> f32 {
  if t < 0.5 {
    0.5 * (1.0 - (1.0 - 4.0 * t * t).sqrt())
  } else {
    0.5 * ((-(2.0 * t - 3.0) * (2.0 * t - 1.0)).sqrt() + 1.0)
  }
}

// Exponential

#[inline]
pub fn expo_in(t: f32) -> f32 {
  if t == 0.0 {
    0.0
  } else {
    (2.0f32).powf(10.0 * (t - 1.0))
  }
}


#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
#[inline]
pub fn expo_out(t: f32) -> f32 {
  if t == 1.0 {
    1.0
  } else {
    1.0 - (2.0f32).powf(-10.0 * t)
  }
}

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
#[inline]
pub fn expo_inout(t: f32) -> f32 {
  if t == 0.0 {
    0.0
  } else if t == 1.0 {
    1.0
  } else if t < 0.5 {
    0.5 * (2.0f32).powf(20.0 * t - 10.0)
  } else {
    -0.5 * (2.0f32).powf(-20.0 * t + 10.0) + 1.0
  }
}

// Elastic

#[inline]
pub fn elastic_in(t: f32) -> f32 {
  (13.0 * FRAC_PI_2 * t).sin() * (2.0f32).powf(10.0 * (t - 1.0))
}

#[inline]
pub fn elastic_out(t: f32) -> f32 {
  (-13.0 * FRAC_PI_2 * (t + 1.0)).sin() * (2.0f32).powf(-10.0 * t) + 1.0
}

#[inline]
pub fn elastic_inout(t: f32) -> f32 {
  if t < 0.5 {
    0.5 * (13.0 * FRAC_PI_2 * 2.0 * t).sin() * (2.0f32).powf(10.0 * (2.0 * t - 1.0))
  } else {
    0.5 * ((-13.0 * FRAC_PI_2 * 2.0 * t).sin() * (2.0f32).powf(-10.0 * (2.0 * t - 1.0)) + 2.0)
  }
}

// Back

#[inline]
pub fn back_in(t: f32) -> f32 {
  t * t * t - t * (t * PI).sin()
}

#[inline]
pub fn back_out(t: f32) -> f32 {
  let f = 1.0 - t;
  1.0 - f * f * f + f * (f * PI).sin()
}

#[inline]
pub fn back_inout(t: f32) -> f32 {
  if t < 0.5 {
    let f = 2.0 * t;
    0.5 * (f * f * f - f * (f * PI).sin())
  } else {
    let f = 2.0 - 2.0 * t;
    0.5 * (1.0 - (f * f * f - f * (f * PI).sin())) + 0.5
  }
}

// Bounce

#[inline]
pub fn bounce_in(t: f32) -> f32 {
  1.0 - bounce_out(1.0 - t)
}

#[inline]
pub fn bounce_out(t: f32) -> f32 {
  if t < 4.0 / 11.0 {
    121.0 / 16.0 * t * t
  } else if t < 8.0 / 11.0 {
    363.0 / 40.0 * t * t - 99.0 / 10.0 * t+ 17.0 / 5.0
  } else if t < 9.0 / 10.0 {
    4356.0 / 361.0 * t * t - 35442.0 / 1805.0 * t + 16061.0 / 1805.0
  } else {
    54.0 / 5.0 * t * t - 513.0 / 25.0 * t + 268.0 / 25.0
  }
}

#[inline]
pub fn bounce_inout(t: f32) -> f32 {
  if t < 0.5 {
    0.5 * bounce_in(t * 2.0)
  } else {
    0.5 * bounce_out(t * 2.0 - 1.0) + 0.5
  }
}


#[cfg(test)]
mod tests {
  macro_rules! tests {
    ($name:ident) => {
      mod $name{
        use super::*;
        use super::super::*;

        #[test] fn in_() { test(concat_idents!($name, _in)); }
        #[test] fn out() { test(concat_idents!($name, _out)); }
        #[test] fn inout() { test(concat_idents!($name, _inout)); }
        #[test] fn trio() { test_trio(concat_idents!($name, _in),
                                      concat_idents!($name, _out),
                                      concat_idents!($name, _inout)); }
      }
    }
  }

  fn test<F>(f: F)
    where F: Fn(f32) -> f32 {
    assert_relative_eq!(f(0.0), 0.0);
    assert_relative_eq!(f(1.0), 1.0);
  }

  fn test_trio<FIN, FOUT, FINOUT>(fin: FIN, fout: FOUT, finout: FINOUT)
    where FIN: Fn(f32) -> f32,
          FOUT: Fn(f32) -> f32,
          FINOUT: Fn(f32) -> f32 {

    let n = 99;

    for i in 0..n+1 {
      let t = i as f32 / n as f32;
      println!("{}", t);

      assert_relative_eq!(fin(t), 1.0 - (fout(1.0 - t)), epsilon = 0.00001);

      if t < 0.5 {
        assert_relative_eq!(finout(t), fin(t*2.0) / 2.0);
      } else {
        assert_relative_eq!(finout(t), fout((t-0.5)*2.0) / 2.0 + 0.5);
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
}
