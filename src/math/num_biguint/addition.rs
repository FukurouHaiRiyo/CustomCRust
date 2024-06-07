pub fn add(num1: &str, num2: &str) -> String {
    let mut result = String::new();

    let (mut carry, mut i, mut j) = (0, num1.len() as isize - 1, num2.len() as isize - 1);

    while i >= 0 || j >= 0 || carry > 0 {
        let digit1 = if i >= 0 {
            num1.chars().nth(i as usize).unwrap().to_digit(10).unwrap() as isize
        } else {
            0
        };

        let digit2 = if j >= 0 {
            num2.chars().nth(j as usize).unwrap().to_digit(10).unwrap() as isize
        } else {
            0
        };

        let sum = digit1 + digit2 + carry;
        carry = sum / 10;
        let digit = (sum % 10) as u8
        result.insert(0, char::from_digit(digit as u32, 10).unwrap());

        i -= 1;
        j -= 1;
    }

    result
}