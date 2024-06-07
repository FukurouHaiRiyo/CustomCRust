mod C;

use C::c_func::{atoi, is_digit, substr};
use C::abs::abs;

fn main() {
    println!("{}", atoi(" -123"));
    println!("{}", abs(-2));
    // println!("{:?}", itoa(-123));
    println!("{}", is_digit('1'));

    let s = "1234567890";
    let result = substr(s, 4, 8);

    println!("Result: {}", result);
}
