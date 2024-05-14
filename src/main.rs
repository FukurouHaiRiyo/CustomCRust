fn format(format_str: &str, args: &[&dyn std::fmt::Debug]) -> String {
    let mut result = String::new();
    let mut iter = format_str.chars().peekable();

    while let Some(c) = iter.next() {
        if c == '%' {
            if let Some(next_char) = iter.peek() {
                match *next_char {
                    'c' => {
                        if let Some(arg) = args.get(0) {
                            result.push_str(&format!("{:?}", arg));
                        } else {
                            panic!("Insufficient arguments for format specifier %c");
                        }
                    }

                    's' => {
                        if let Some(arg) = args.get(0) {
                            result.push_str(&format!("{:?}", arg));
                        } else {
                            panic!("Insufficient arguments for format specifier %s");
                        }
                    }

                    'p' => {
                        if let Some(arg) = args.get(0) {
                            result.push_str(&format!("{:p}", arg));
                        } else {
                            panic!("Insufficient arguments for format specifier %p");
                        }
                    }

                    'd' | 'i' => {
                        if let Some(arg) = args.get(0) {
                            result.push_str(&format!("{:?}", arg));
                        } else {
                            panic!("Insufficient arguments for format specifier %d or %i");
                        }
                    }

                    'u' => {
                        if let Some(arg) = args.get(0) {
                            result.push_str(&format!("{:?}", arg))
                        } else {
                            panic!("Insufficient arguments for format specifier %u");
                        }
                    }

                    'x' => {
                        if let Some(arg) = args.get(0) {
                            result.push_str(&format!("{:?}", arg));
                        } else {
                            panic!("Insufficient arguments for format specifier %x or %X");
                        }
                    }

                    '%' => {
                        result.push('%');
                    }

                    _ => panic!("Unsupported format specifier: %{}", next_char),
                }
                iter.next();
            }
        } else {
            result.push(c);
        }
    } 

    result
}

fn printf(format_str: &str, args: &[&dyn std::fmt::Debug]) {
    let formatted_string = format(format_str, args);
    println!("{}", formatted_string);
}

fn atoi(s: &str) -> i32 {
    // Convert the input string to a byte array for easier processing
    let bytes = s.as_bytes();
    let mut i = 0;
    let len = bytes.len();

    // skip leading whitepaces
    while i < len && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    // check for sign
    let mut sign = 1;
    if i < len {
        if bytes[i] == b'-' {
            sign = -1;
            i += 1;
        } else if bytes[i] == b'+' {
            i += 1;
        }
    }

    // convert digits to integer
    let mut result: i32 = 0;
    while i < len {
        if bytes[i].is_ascii_digit() {
            result = result.saturating_mul(10).saturating_add((bytes[i] - b'0') as i32);
            i += 1;
        } else {
            break;
        }
    }

    // apply the sing
    result * sign
}

fn main() {
    let mut line = String::new();
    println!(
        "Enter your name: "
    );
    std::io::stdin().read_line(&mut line).unwrap();
    let name = line.trim();

    printf("Your name: %s", &[&name]);

    let s = "     -12345abcd";

    let result = atoi(s);
    println!("The result is: {}", result); // Output should be: -12345
}