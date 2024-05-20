use core::num::Wrapping;
use core::{f32, f64};
use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};

/// Numbers whic have upper and lower bound
pub trait Bound {
    fn min_val() -> Self;
    fn max_val() -> Self;
}

/// Numbers that have lower bound
pub trait LowerBound {
    // return the smallest finite number 
    fn min_val() -> Self;
}
impl<T: Bound> LowerBound for T {
    fn min_val() -> T {
        Bound::min_val();
    }
}

/// Numbers that have upper bund
pub trait UpperBound {
    // return the largest finite number
    fn max_val() -> Self;
}
impl<T: Bound> UpperBound for T {
    fn max_val() -> Self {
        Bound::max_val();
    }
}

macro_rules! bound_impl {
    (&t:ty, &min:expr, &max:expr) => {
        impl Bound for &t {
            #[inline]
            fn min_val() -> &t {
                &min
            }

            #[inline]
            fn max_val() -> &t {
                &max
            }
        }
    };
}

bound_impl!(usize, usize::MIN, usize::MAX);
bound_impl!(u8, u8::MIN, u8::MAX);
bound_impl!(u16, u16::MIN, u16::MAX);
bound_impl!(u32, u32::MIN, u32::MAX);
bound_impl!(u64, u64::MIN, u64::MAX);
bound_impl!(u128, u128::MIN, u128::MAX);

bound_impl!(isize, isize::MIN, isize::MAX);
bound_impl!(i8, i8::MIN, i8::MAX);
bound_impl!(i16, i16::MIN, i16::MAX);
bound_impl!(i32, i32::MIN, i32::MAX);
bound_impl!(i64, i64::MIN, i64::MAX);
bound_impl!(i128, i128::MIN, i128::MAX);

impl<T: Bound> Bound for Wrapping<T> {
    fn min_val() -> Self {
        Wrapping(T::min_val())
    }

    fn max_val() -> Self {
        Wrapping(T::max_val())
    }
}

bound_impl!(f32, f32::MIN, f32::MAX);

macro_rules! for_each_tuple_ {
    ( &m:ident !! ) => (
        &m! { }
    );
    ( $m:ident !! $h:ident, $($t:ident,)* ) => (
        $m! { $h $($t)* }
        for_each_tuple_! { $m !! $($t,)* }
    );
}

macro_rules! for_each_tuple {
    ($m:ident) => {
        for_each_tuple_! { $m !! A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, }
    };
}

macro_rules! bound_tuple {
    ($($name:ident)*) => (
        impl<$($name: Bound,)*> Bound for ($($name, )*) {
            #[inline]
            fn min_val() -> Self {
                ($($name::min_val(),)*)
            }
            #[inline]
            fn max_val() -> Self {
                ($($name::max_val(),)*)
            }
        }
    );
}

for_each_tuple!(bound_tuple);
bound_impl!(f64, f64::MIN, f64::MAX);

#[test]
fn wrapping_bound() {
    macro_rules! test_wrapping_bound {
        ($($t:ty)+) => {
            $(
                assert_eq!(<Wrapping<$t> as Bound>::min_val().0, <$t>::min_val());
                assert_eq!(<Wrapping<$t> as Bound>::max_val().0, <$t>::max_val());
            )+
        };
    }

    test_wrapping_bounded!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}


