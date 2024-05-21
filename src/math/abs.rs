use crate::math::num_traits::identities::Zero;

// Return the abolute value of a number
pub fn abs<T>(num: T) -> T where T: std::ops::Neg<Output = T> + PartialOrd + Copy + Zero, {
    if num < T::zero() {
        return -num;
    }

    num
}