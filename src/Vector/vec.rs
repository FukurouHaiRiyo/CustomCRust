use std::ops::{ Deref, DerefMut };
use std::ptr;

/// A growable, heap-allocated vector that provides dynamic resizing and element storage
/// This implementation is built on top of 'RawVec', which handles memory allocation.
pub struct Vec<T> {
    buf: super::raw_vec::RawVec<T>, // Low-level memory buffer
    len: usize, // Current number of elements in the vector
}

impl<T> Vec<T> {
    /// Creates a new, empty 'Vec' with zero capacity
    pub fn new() -> Self {
        Vec {
            buf: super::raw_vec::RawVec::new(), // Initialize with an empty buffer
            len: 0,
        }
    }

    /// Adds an element to the end of the vector
    /// If the current capacity is insufficient, it grows the buffer
    pub fn push(&mut self, elem: T) {
        if self.len == self.buf.cap {
            self.buf.grow(); // Increase capacity when full
        }

        unsafe {
            // Write the element into the uninitialized memory
            ptr::write(self.buf.ptr.as_ptr().add(self.len), elem);
        }
        self.len += 1; // Increment the length
    }

    /// Removes and returns the last element of the vector
    /// Return 'None' if the vector is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == { 
            return None; // No elements to remove
        }

        self.len -= 1; // Decrement the length
        unsafe {
            // Read and return the value from the last position
            Some(ptr::read(self.buf.ptr.as_ptr().add(self.len)))
        }
    }

    /// Return the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the capacity of the vector (number of elements it can hold without resizing)
    pub fn capacity(&self) -> usize {
        self.buf.cap
    }

    /// Return true if the vector contains no elements
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Provides a mutable reference to an element at a given index
    /// # Panics
    /// Panics if the index is out of bounds
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.len, "Index out of bounds");
        unsafe { &mut *self.buf.ptr.as_ptr().add(index) }
    }

    /// Clears the vector, dropping all elements.
    /// The capacity remains unchanged.
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> Drop for Vec<T> {
    /// Drops the vector and deallocates all associated memory.
    /// All elements are dropped in reverse order before deallocating the buffer.
    fn drop(&mut self) {
        while let Some(_) = self.pop() {} // Drop each element
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    /// Dereferences the vector to a slice.
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.buf.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    /// Provides mutable dereference to a slice.
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.buf.ptr.as_ptr(), self.len) }
    }
}
