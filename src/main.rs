mod C;

use C::c_func::{itoa, is_digit, substr, strcat, NameOf, sublist, sort};
use C::math::{abs, sin, floor, ceil, natural_log, real_part};
use C::listcat;
extern crate num_complex;
use num_complex::Complex;

use clap::{App, Subcommand};


fn main() {
    let matches = App::new("CLI Rust")
                    .version("1.0").
                    .author("Fufu <andreipanait00@gmail.com>")
                    .about("C function and other utility functions written in Rust that can be run in CLI.")
                    .args_from_usage("")
}
