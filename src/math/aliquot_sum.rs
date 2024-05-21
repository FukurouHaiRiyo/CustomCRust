/// Aliquot sum of a number is defined as the sum of the proper divisors of a number.\
/// meaning all the divisors of a number apart from the number itself.
///
/// ## Example:
/// The aliquot sum of 6 is (1 + 2 + 3) = 6, and that of 15 is (1 + 3 + 5) = 9
///
/// Wikipedia article on Aliquot Sum: <https://en.wikipedia.org/wiki/Aliquot_sum>

pub fn aliquot_sum(num: u64) -> u64 {
    if num == 0 {
        panic!("The input must be positive")
    }

    (1..= num / 2).filter(|&d| num % d == 0).sum()
}