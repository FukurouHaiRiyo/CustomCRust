use crate::C::math::abs;


use std::alloc::{alloc as all, Layout};
use std::ptr;

pub trait NameOf{
    fn name_of(&self) -> &'static str;
}

impl NameOf for i32 {
    fn name_of(&self) -> &'static str {
        "i32"
    }
}
impl NameOf for f64 {
    fn name_of(&self) -> &'static str {
        "f64"
    }
}
impl NameOf for String {
    fn name_of(&self) -> &'static str {
        "String"
    }
}
impl NameOf for char {
    fn name_of(&self) -> &'static str {
        "char"
    }
}

impl<T> NameOf for &T where T: NameOf {
    fn name_of(&self) -> &'static str {
        (*self).name_of()
    }
}

fn isspace(c: char) -> bool {
    c == '\t' || c == '\n' || c == '\x0b' || c == '\x0c' || c == '\r' || c == ' '
}

pub fn is_alnum(c: char) -> bool {
    #![allow(unused_imports)]
    (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9')
}

pub fn is_alpha(c: char) -> bool {
    #![allow(unused_imports)]
    (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
}

pub fn is_ascii(c: i32) -> bool {
    #![allow(unused_imports)]
    c >= 0 && c<= 127
}

pub fn is_digit(c: char) -> bool {
    #![allow(unused_imports)]
    c >= '0' && c <= '9'
}

fn is_print(c: i32) -> bool {
    #![allow(unused_imports)]
    c >= 32 && c <= 126
}

fn is_space(c: char) -> bool {
    #![allow(unused_imports)]
    (c >= '\u{0009}' && c <= '\u{000D}') || c == ' '
}

fn len_num(mut n: i32) -> usize {
    #![allow(unused_imports)]
    let mut len = 0;
    if n <= 0 {
        len += 1;
    }

    while n != 0 {
        n /= 10;
        len += 1;
    }

    len
}

pub fn substr(s: &str, from: usize, to: usize) -> String {
    #![allow(unused_imports)]
    if from > to || from < 1 {
        return String::new();
    }

    let from_index = from - 1;
    let to_index = if to > s.len() { s.len() } else { to };

    let mut result = String::new();

    if from_index < s.len() {
        result.push_str(&s[from_index..to_index]);
    }

    if to > s.len() {
        result.push_str(&" ".repeat(to - s.len()));
    }

    result
}

pub fn strcat(args: &[&dyn ToString]) -> String {
    #![allow(unused_imports)]
    let mut result = String::new();
    for arg in args {
        result.push_str(&arg.to_string());
    }

    result
}

pub fn itoa(n: i32) -> Option<String> {
    #![allow(unused_imports)]

    let len = len_num(n);
    let mut ret = unsafe { all(Layout::array::<u8>(len + 1).unwrap()) } as *mut u8;

    if ret.is_null() {
        return None;
    }

    let mut n = n;

    unsafe {
        ret = ret.offset(len as isize);
        *ret = b'\0';
        if n == 0 {
            *ret.offset(-1) = b'0';
        } else if n < 0 {
            *ret.offset(-1) = b'-';
            n = -n;
        }

        while n != 0 {
            ret = ret.offset(-1);
            *ret = (abs(n % 10) as u8) + b'0';
            n /= 10;
        }
    }

    let s = unsafe {
        String::from_utf8_unchecked(Vec::from_raw_parts(ret, len + 1, len + 1))
    };

    Some(s)
}

pub fn atoi(s: &str) -> i32 {
    #![allow(unused_imports)]
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
    #![allow(unused_imports)]
    while n > 0 {
        n -= 1;
        s[n] = 0;
    }
}

fn alloc(count: usize, size: usize) -> *mut u8 {
    #![allow(unused_imports)]
    let (count, size) = if count == 0 || size == 0 {
        (1, 1)
    } else {
        (count, size)
    };

    let total_size = count * size;
    let layout = Layout::from_size_align(total_size, 1).expect("Failed to create layout");

    let ret = unsafe {all(layout)};
    if ret.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        ptr::write_bytes(ret, 0, total_size);
    }

    ret
}

pub fn sublist<T: Clone + Default>(list: &[T], from: usize, to:usize) -> Vec<T> {
    if from > to || from < 1 {
        return Vec::new();
    }

    let from_index = from - 1; // adjust for 0-based index
    let to_index = if to > list.len() {
        list.len()
    } else {
        to
    };


    let mut result = Vec::new();

    if from_index < list.len() {
        result.extend_from_slice(&list[from_index..to_index]);
    }

    // Append placeholder items if to is greater than the length of the list
    if to > list.len() {
        result.extend((0..to - list.len()).map(|_| T::default())); // Using T::default() for placeholders
    }

    result
}
