use std::alloc::{alloc, Layout};
use std::ptr;

fn isspace(c: char) -> bool {
    c == '\t' || c == '\n' || c == '\x0b' || c == '\x0c' || c == '\r' || c == ' '
}

fn isalnum(c: char) -> bool {
    (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9')
}

pub fn atoi(s: &str) -> i32 {
    let mut chars = s.chars().peekable(); // Using peekable to handle leading whitespaces
    let mut sign = 1;
    let mut result = 0;

    while let Some(&c) = chars.peek() {
        if isspace(c) {
            chars.next(); // Consume the whitespace character
            continue;
        }
        if c == '-' || c == '+' {
            if c == '-' {
                sign = -1;
            }
            chars.next(); // Consume the sign character
            continue;
        }
        if c.is_digit(10) {
            result = result * 10 + c.to_digit(10).unwrap() as i32;
            chars.next(); // Consume the digit character
        } else {
            break;
        }
    }

    sign * result
}

pub fn bzero(s: &mut [u8], mut n: usize) {
    while n > 0 {
        n -= 1;
        s[n] = 0;
    }
}

fn alloc(count: usize, size: usize) -> *mut u8 {
    let (count, size) = if count == 0 || size == 0 {
        (1, 1)
    } else {
        (count, size)
    };

    let total_size = count * size;
    let layout = Layout::from_size_align(total_size, 1).expect("Failed to create layout");

    let ret = unsafe {alloc(layout)};
    if ret.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        ptr::write_bytes(ret, 0, total_size);
    }

    ret
}
