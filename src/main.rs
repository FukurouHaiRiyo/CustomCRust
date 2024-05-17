mod print;
mod math;


fn main() {
    let mut line = String::new();
    println!(
        "Enter your name: "
    );
    std::io::stdin().read_line(&mut line).unwrap();
    let name = line.trim();

    print::p::printf("Your name: %s", &[&name]);

    println!("{}", math::abs::abs(-2));

    // let s = "     -12345abcd";

    // let result = atoi(s);
    // println!("The result is: {}", result); // Output should be: -12345
}