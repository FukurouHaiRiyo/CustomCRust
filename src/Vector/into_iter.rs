use super::vec::Vec;

/// An iterator that consumes 'Vec' and yields its elements
pub struct IntoIter<T> {
    buf: super::raw_vec::RawVec<T>, // Memory buffer from the original Vec
    start: *const T, // pointer to the start of the buffer
    end: *const T, // Pointer to the end of the buffer (one past the last element)
}