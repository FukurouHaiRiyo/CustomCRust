mod C;
mod ciphers;

use C::c_func::{itoa, is_digit, substr, strcat, NameOf, sublist, sort};
use C::math::{abs, sin, floor, ceil, natural_log, real_part};
use C::listcat;
use ciphers::aes;
extern crate num_complex;
use num_complex::Complex;


fn main() {
    let key128: [u8; 16] = [0; 16]; // 16 bytes, all set to 0
    let aes_key_128 = aes::AesKey::AesKey128(key128);

    println!("{:?}", aes::aes_encrypt(b"Andrei", aes_key_128));
}
