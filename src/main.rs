mod print;
mod math;

use print::p::*;
use math::c_func::{atoi, itoa};
use math::abs::abs;

fn main() {
    println!("{}", atoi(" -123"));
    println!("{}", abs(-2));
    // println!("{:?}", itoa(-123));
}