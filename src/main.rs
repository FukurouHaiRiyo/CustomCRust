mod C;

use C::c_func::{atoi, is_digit, substr, strcat, NameOf, sublist, sort};
use C::math::{abs, sin, floor, ceil, natural_log};
use C::listcat;

fn main() {
    /* 
    // test for atoi
    println!("{}", atoi(" -123"));

    // test for abs
    println!("Abs test: {}", abs(-2));

    // test for itoa
    // println!("{:?}", itoa(-123));

    // test for is_digit
    println!("is_digit test: {}", is_digit('1'));

    // test for substr
    let s = "1234567890";
    let result = substr(s, 4, 8);
    println!("substr test: {}", result);

    // test for strcat
    let s1 = strcat(&[&"Garden", &" of ", &"Eden", &'.']);
    let s2 = strcat(&[&s1, &" Will"]);
    println!("strcat test: {}, {}", s1, s2);

    // test for name_of()
    println!("name_of test: {}", (&s2).name_of());

    // test for sublist
    let list = vec![1, 2, 3, 4, 5];
    println!("sublist test: {:?}", sublist(&list, 2, 4));

    // test for listcat
    let list1 = vec![6, 7, 8];
    let reuslt = listcat!(list, list1);
    println!("listcat test: {:?}", result);

    // test for sin 
    let angle: f64 = 1.0;
    println!("sin({}) = {}", angle, sin(angle));
    */

    // test for sort
    /* let mut numbers = vec![10, 6, 3, 2, 7, 5, 8, 9, 1, 4];
    println!("Array before sorting: {:?}", numbers);
    
    sort(&mut numbers);
    println!("Array after sorting: {:?}", numbers);*/ 

    // test for floor
    let numbers = vec![3.7, -3.7, 4.0, -4.0, 0.0, -0.9];
    for number in numbers {
        println!("The floor of {} is {} and the ceil is {} and the log is {}", number, floor(number), ceil(number), natural_log(number));
    }
}
