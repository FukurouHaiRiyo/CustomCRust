mod C;

use C::c_func::{atoi, is_digit, substr, strcat, NameOf};
use C::abs::abs;

fn main() {
    // test for atoi
    println!("{}", atoi(" -123"));

    // test for abs
    println!("{}", abs(-2));

    // test for itoa
    // println!("{:?}", itoa(-123));

    // test for is_digit
    println!("{}", is_digit('1'));

    // test for substr
    let s = "1234567890";
    let result = substr(s, 4, 8);
    println!("Result: {}", result);

    // test for strcat
    let s1 = strcat(&[&"Garden", &" of ", &"Eden", &'.']);
    let s2 = strcat(&[&s1, &" Will"]);
    println!("{}, {}", s1, s2);

    // test for name_of()
    println!("{}", (&s2).name_of());
}
