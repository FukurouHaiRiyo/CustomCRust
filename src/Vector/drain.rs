use super::vec::Vec;

/// A drain iterator for 'Vec'
/// Removes elements from a Vector while iterating over them 
pub struct Drain<'a, T: 'a> {
    vec: &'a mut Vec<T>, // reference to the original vector
    start: usize, // starting index of the drain range
    end: usize, // ending index of the drain range
    index: usize, // current position in the range
}

impl<'a, T> Drain<'a, T> {
    /// creates a new 'Drain' iterator for a given range in a 'Vec'
    pub fn new(vec: &'a mut Vec<T>, start: usize, end: usize) -> Self {
        assert!(start <= end && end <= vec.len(), "Invalid range");
        Drain {
            vec,
            start,
            end,
            index: start,
        }
    }
}

impl<'a, T>Iterator for Drain<'a, T> {
    type Item = T;

    /// Advances the iterator and removes the next element from the vector
    /// # Safety
    /// Modifies the original vector in place, ensuring elements are removed correctly
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            return None; // no more elements to drain
        }

        unsafe {
            let elem = ptr::read(self.vec.get_mut(self.index)); // read the value
            self.index += 1; // move to the next position
            Some(elem)
        }
    }

    /// returns the number of elements remaining in the drain range
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end - self.index;
        (remaining, Some(remaining))
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    /// Drops the iterator and removes all remaining elements in the range.
    fn drop(&mut self) {
        while let Some(_) = self.next() {} // Drain remaining elements
        let remaining_len = self.vec.len() - (self.end - self.start);
        unsafe {
            self.vec.len = remaining_len; // Update the length of the vector
        }
    }
}
