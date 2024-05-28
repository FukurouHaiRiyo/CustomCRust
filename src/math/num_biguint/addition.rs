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
    unsafe { 
        core::arch::x86_64::_addcarry_u64(carry, a, b, out) 
    }
}

#[cfg(target_arch = "x86")]
#[inline]
fn adc(carry: u8, a: u32, b: u32, out: &mut u32) -> u8 {
    // Safety: There are absolutely no safety concerns with calling `_addcarry_u32`.
    // It's just unsafe for API consistency with other intrinsics.
    unsafe { 
        core::arch::x86::_addcarry_u32(carry, a, b, out) 
    }
}

// fallback for environments where we don't have an addcarry intrinsic
// (copied from the standard library's `carrying_add`)
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
fn adc(carry: u8, lhs: BigDigit, rhs: BigDigit, out: &mut BigDigit) -> u8 {
    let (a, b) = lhs.overflowing_add(rhs);
    let (c, d) = a.overflowing_add(carry as BigDigit);
    *out = c;
    u8::from(b || d)
}

/// Two argument addition of raw slices, `a += b`, returning the carry.
///
/// This is used when the data `Vec` might need to resize to push a non-zero carry, so we perform
/// the addition first hoping that it will fit.
///
/// The caller _must_ ensure that `a` is at least as long as `b`.
#[inline] 
pub(super) fn __add2(a: &mut [BigDigit], b: &[BigDigit]) -> BigDigit {
    debug_assert!(a.len() >= b.len());

    let mut carry = 0;
    let (a_lo, a_hi) = a.split_at_mut(b.len());


    for (a, b) in a_lo.iter_mut().zip(b) {
        carry = adc(carry, *a, *b, a);
    }

    if carry != 0 {
        for a in a_hi {
            carry = adc(carry, *a, *b, a);
            if carry == 0{
                break;
            }
        }
    }

    carry as BigDigit
}


