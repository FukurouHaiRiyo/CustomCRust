use core::mem::size_of;
use core::num::Wrapping;
use core::{f32, f64};
use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};

/* 
    A generic trait for converting a value to a number.
    A value can be represented by the target type when it lies within
    the range of scalars supported by the target type.
    For example, a negative integer cannot be represented by an unsigned
    integer type, and an `i64` with a very high magnitude might not be
    convertible to an `i32`.
    On the other hand, conversions with possible precision loss or truncation
    are admitted, like an `f32` with a decimal part to an integer type, or
    even a large `f64` saturating to `f32` infinity. 
*/

pub trait ToPrimitive {
    /*
        Converts the value of self to an 'isize'. If the value cannot be 
        represented by an 'isize', then 'None' is returned/
    */
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        self.to_i64().as_ref.and_then(ToPrimitive::to_isize)
    }

    /*
        Converts the value of self to an 'i8'. If the value cannot be 
        represented by an 'i8', then 'None' is returned.
    */
    #[inline]
    fn to_i8(&self) -> Option<i8>{
        self.to_i64().as_ref().and_then(ToPrimitive::to_i8)
    }

    /*
        Converts the value of self to an 'i16'. If the value cannot be 
        represented by an 'i16', then 'None' is returned.
    */
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.to_i64().as_ref().and_then(ToPrimitive::to_i16)
    }

    /*
        Converts the value of self to an 'i32'. If the value cannot be 
        represented by an 'i32', then 'None' is returned.
    */
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.to_i64().as_ref().and_then(ToPrimitive::to_i32)
    }

    /*
        Converts the value of self to an 'i64'. If the value cannot be 
        represented by an 'i64', then 'None' is returned.
    */
    fn to_i64(&self) -> Option<i64>;

    /*
        Converts the value of self to an 'i128'. If the value cannot be 
        represented by an 'i128' ('i64' under the default implementation), then 'None' is returned.

        The default implementation converts through `to_i64()`. Types implementing
        this trait should override this method if they can represent a greater range.
    */
    #[inline]
    fn to_i128(&self) -> Option<i128> {
        self.to_i64().map(From::from)
    }

    /*
        Converts the value of self to an 'usize'. If the value cannot be reprezented
        by an 'usize', then 'None' will be returned.
    */
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        self.to_u64().as_ref().and_then(ToPrimitive::to_usize)
    }

    
}
