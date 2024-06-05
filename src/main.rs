mod print;
mod math;

use print::p::*;
use math::c_func::{atoi, itoa, is_digit};
use math::abs::abs;

fn main() {
    println!("{}", atoi(" -123"));
    println!("{}", abs(-2));
    // println!("{:?}", itoa(-123));
    println!("{}", is_digit('1'));
}