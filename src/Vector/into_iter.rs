use super::vec::Vec;

/// An iterator that consumes 'Vec' and yields its elements
pub struct IntoIter<T> {
    buf: super::raw_vec::RawVec<T>, // Memory buffer from the original Vec
    start: *const T, // pointer to the start of the buffer
    end: *const T, // Pointer to the end of the buffer (one past the last element)
}

impl<T> IntoIter<T> {
    /// Creates a new 'IntoIter' from a given 'Vec'
    pub fn new(vec: Vec<T>) -> Self {
        let len = vec.len(); // get the number of elements
        let buf = vec.buf; // take ownership of the buffer
        let start = buf.ptr.as_ptr();
        let end = unsafe { start.add(len) }; // compute the end pointer

        std::mem::forget(vec); // prevent the original Vec from being dropped

        IntoIter { buf, start, end }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    /// advances the iterator and returns the next value
    /// # safety
    /// Ensures elements are only accessed once and in order
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None; // No more elements
        }

        unsafe {
            let value = ptr::read(self.start); // read the value from the current possition
            self.start = self.start.add(1); // move the start pointer forward
            Some(value)
        }
    }

    /// Returns the number of elements remaining in the iterator
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.end as usize - self.start as usize) / std::mem::size_of::<T>();
        (remaining, Some(remaining))
    }
}

impl<T> Drop for IntoIter<T> {
    /// Drops the iterator and deallocates any remaining elements.
    /// # Safety
    /// Ensures all remaining elements are properly dropped before deallocating the buffer.
    fn drop(&mut self) {
        for _ in self {} // Consume all elements to ensure proper cleanup
        unsafe {
            self.buf.dealloc(); // Deallocate the buffer
        }
    }
}


