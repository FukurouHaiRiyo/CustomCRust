// Return the abolute value of a number
pub fn abs<T>(num: T) -> T where T: std::ops::Neg<Output = T> + PartialOrd + Copy + num_traits::Zero, {
    if num < T::zero() {
        return -num;
    }

    num
}