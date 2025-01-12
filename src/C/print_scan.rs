use std::io::{self, Write};

/// Trait for parsing input into various variable types.
pub trait ScanfInput {
    fn parse_input(&mut self, value: &str, type_name: &str);
}

/// Implementation of ScanfInput for integers.
impl ScanfInput for i32 {
    fn parse_input(&mut self, value: &str, _: &str) {
        *self = value.parse::<i32>().expect("Failed to parse integer");
    }
}

/// Implementation of ScanfInput for floats.
impl ScanfInput for f32 {
    fn parse_input(&mut self, value: &str, _: &str) {
        *self = value.parse::<f32>().expect("Failed to parse float");
    }
}

/// Implementation of ScanfInput for strings.
impl ScanfInput for String {
    fn parse_input(&mut self, value: &str, _: &str) {
        *self = value.to_string();
    }
}

/// Implementation of ScanfInput for characters.
impl ScanfInput for char {
    fn parse_input(&mut self, value: &str, _: &str) {
        *self = value.chars().next().expect("Failed to parse char");
    }
}


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

/// Prints a formatted string to the console.
pub fn printf(format_str: &str, args: &[&dyn std::fmt::Debug]) {
    let formatted_string = format(format_str, args);
    println!("{}", formatted_string);
}

/// Reads input from the user and parses it based on the format string.
pub fn scanf(format: &str, args: &mut [&mut dyn ScanfInput]) {
    let mut input = String::new();
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut input_iter = input.trim().split_whitespace();

    let mut format_iter = format.chars().peekable();
    let mut arg_index = 0;

    while let Some(c) = format_iter.next() {
        if c == '%' {
            if let Some(specifier) = format_iter.next() {
                if let Some(arg) = args.get_mut(arg_index) {
                    if let Some(value) = input_iter.next() {
                        match specifier {
                            'd' | 'i' => arg.parse_input(value, "integer"),
                            'f' => arg.parse_input(value, "float"),
                            's' => arg.parse_input(value, "string"),
                            'c' => arg.parse_input(value, "char"),
                            _ => panic!("Unsupported format specifier: %{}", specifier),
                        }
                        arg_index += 1;
                    } else {
                        panic!("Insufficient input for format specifier %{}", specifier);
                    }
                } else {
                    panic!("Insufficient arguments for format specifier %{}", specifier);
                }
            }
        }
    }
}
