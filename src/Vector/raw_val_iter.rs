/// An iterator that moves out of a raw memory buffer of elements 
pub struct RawValIter<T> {
    start: *const T; // pointer to the start of the buffer
    end: *const T; // pointer to the end of the buffer
}

impl<T> RawValIter<T> {
    /// Creates a new `RawValIter` from a raw pointer and length.
    /// # Safety
    /// The caller must ensure the raw memory is valid for the given range.
    pub unsafe fn new(start: *const T, len: usize) -> Self {
        RawValIter {
            start,
            end: start.add(len), // Compute the end pointer
        }
    }
}

impl<T> Iterator for RawValIter<T> {
    type Item = T;

    /// Advances the iterator and returns the next value.
    /// # Safety
    /// The caller must ensure elements are not accessed after being consumed.
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None; // no more elements
        }

        unsafe {
            let value = ptr::read(self.start); // read the value from the current position
            self.start = self.start.add(1); // Move the start pointer forward
            Some(value)
        }
    }

    /// return the number of elements remaining in the iterator
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.end as usize - self.start as usize) / std::mem::size_of::<T>();
        (remaining, Some(remaining))
    }
}

impl<T> Drop for RawValIter<T> {
    /// Drops the iterator and deallocates any remaining elements.
    /// # Safety
    /// Ensures all elements are properly dropped before freeing memory.
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}
