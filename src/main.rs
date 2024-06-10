mod C;

use C::c_func::{atoi, is_digit, substr, strcat, NameOf, sublist};
use C::math::abs;

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

    // test for sublist
    let list = vec![1, 2, 3, 4, 5];
    println!("{:?}", sublist(&list, 2, 4));
}
