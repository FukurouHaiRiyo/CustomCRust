mod print;
mod math;


fn main() {
    // let mut line = String::new();
    // println!(
    //     "Enter your name: "
    // );
    // std::io::stdin().read_line(&mut line).unwrap();
    // let name = line.trim();

    // print::p::printf("Your name: %s", &[&name]);

    println!("{}", math::abs::abs(-2));

    println!("{}", math::aliquot_sum::aliquot_sum(6));

    println!("{:?}", math::amicable_nums::amicable_nums(1000));
}