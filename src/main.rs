mod print;
mod math;

use print::p::*;
use math::c_func::{atoi, itoa, is_digit};
use math::abs::abs;
use math::num_biguint::addition;

fn main() {
    println!("{}", atoi(" -123"));
    println!("{}", abs(-2));
    // println!("{:?}", itoa(-123));
    println!("{}", is_digit('1'));

    let result = addition::add("123", "123");
    println!("{}", result);
}
