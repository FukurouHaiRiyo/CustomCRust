use std::io::{self, Write};

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

pub fn printf(format_str: &str, args: &[&dyn std::fmt::Debug]) {
    let formatted_string = format(format_str, args);
    println!("{}", formatted_string);
}

pub fn scanf(format: &str) -> Result<Vec<String>, &'static str> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| "Failed to read line")?;
    let input = input.trim();

    // Split the format string by whitespace
    let format_parts: Vec<&str> = format.split_whitespace().collect();

    // Split the input string by whitespace
    let input_parts: Vec<&str> = input.split_whitespace().collect();

    if format_parts.len() != input_parts.len() {
        return Err("Input does not match format");
    }


    let mut result = Vec::new();
    for part in input_parts {
        result.push(part.to_string());
    }

    Ok(result);
}
