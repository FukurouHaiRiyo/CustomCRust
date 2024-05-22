use super::{BigUint, IntDigits};
use crate::big_digit::{self, BigDigit};
use crate::UsizePromotion;

use core::iter::Sum;
use core::ops::{Add, AddAssign};
use num_traits::CheckedAdd;

// add with carry
#[cfg(target_arch="x86_64")]
#[inline]
fn adc(carry: u8, a: u64, b: u64, out: &mut u64) -> u8 {
    // Safety: There are absolutely no safety concerns with calling `_addcarry_u64`.
    // It's just unsafe for API consistency with other intrinsics.
    unsafe { core::arch::x86_64::_addcarry_u64(carry, a, b, out) }
}

#[cfg(target_arch = "x86")]
#[inline]
fn adc(carry: u8, a: u32, b: u32, out: &mut u32) -> u8 {
    // Safety: There are absolutely no safety concerns with calling `_addcarry_u32`.
    // It's just unsafe for API consistency with other intrinsics.
    unsafe { core::arch::x86::_addcarry_u32(carry, a, b, out) }
}


