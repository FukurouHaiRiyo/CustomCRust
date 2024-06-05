mod print;
mod math;

use print::p::*;
use math::c_func::atoi;

fn main() {
    println!("{}", atoi(" -123"));
}