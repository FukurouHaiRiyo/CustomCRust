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
