use crate::C::num_traits::identities::Zero;
extern crate num_complex;
use num_complex::Complex;

// Return the abolute value of a number
pub fn abs<T>(num: T) -> T where T: std::ops::Neg<Output = T> + PartialOrd + Copy + Zero, {
    if num < T::zero() {
        return -num;
    }

    num
}

pub fn sin(x: f64) -> f64 {
    x.sin()
}

pub fn cos(x: f64) -> f64 {
    x.cos()
}

pub fn tan(x: f64) -> f64 {
    x.tan()
}

pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

pub fn pow(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

pub fn floor(x: f64) -> f64 {
    if x >= 0.0 {
        x.trunc() // trunc() cuts off the fractional part, which is the same as floor for non-negative numbers
    } else {
        // for non negative numbers, we need to check if it has a fractional part
        let trunc = x.trunc();
        if trunc == x {
            // if truncating didn't change the value, it's already an integer
            trunc 
        } else {
            // Otherwise, we subtract 1 to get the next lower integer
            trunc - 1.0
        }
    }
}

pub fn ceil(x: f64) -> f64 {
    if x >= 0.0 {
        x.trunc() // trunc() cuts off the fractional part, which is the same as floor for non-negative numbers
    } else {
        // for non negative numbers, we need to check if it has a fractional part
        let trunc = x.trunc();
        if trunc == x {
            // if truncating didn't change the value, it's already an integer
            trunc 
        } else {
            // Otherwise, we subtract 1 to get the next lower integer
            trunc + 1.0
        }
    }
}

pub fn natural_log(x: f64) -> f64 {
    if x > 0.0 {
        x.ln()
    } else {
        f64::NAN // logarithm is not defined for non-postive numbers
    }
}

pub fn real_part<T>(complex: &Complex<T>) -> &T {
    &complex.re
}
 