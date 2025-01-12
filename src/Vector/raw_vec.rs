use std::alloc::{ self, Layout };
use std::ptr::NonNull;
use std::mem;

/// A low-level, untyped memory buffer used by `Vec`.
/// Handles memory allocation, deallocation, and resizing operations.
pub struct RawVec<T> {
    pub ptr: NonNull<T>, // pointer to the allocated memory block
    pub cap: usize, // capacity of the allocated memory block
}

// Ensure thread safety for `RawVec` if `T` is `Send` or `Sync`
unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}


impl <T> RawVec<T> {
    /// Creates a new, empty `RawVec`.
    /// The capacity is set to 0, except for zero-sized types where capacity is `!0`.
    pub fn new() -> Self {
        let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };

        RawVec {
            ptr: NonNull::dangling(), // use a dangling pointer for initial state
            cap,
        }
    }

    /// Grows the capacity of the 'RawVec'
    /// If the current capacity is 0, it allocates memory with an initial capacity of 1
    /// Otherwise, the capacity is doubled.
    pub fn grow(&mut self) {
        assert!(mem::size_of::<T>() != 0, "capacity overflow");

        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap()) // Initial allocation
        } else {
            let new_cap = 2 * self.cap; // Double the current capacity
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Ensure the allocation size is withing allowable bounds
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        // Allocate or reallocate memory fro the new capacity
        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) } // First allocation
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout,size()) } // resize allocation
        };

        // Update the pointer and capacity
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };;
        self.cap = new_cap;
    }
}

impl<T> Drop fro RawVect<T> {
    /// Deallocates the memory used by the 'RawVec'.
    /// This is only performed if the capacity if non-zero and the element type is not zero-sized.
    fn drop(&mut self) {
        if self.cap != 0 && mem::size_of::<T> != 0 {
            unsafe { 
                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8, // Convert to raw byte pointer
                    Layout::array::<T>(self.cap).unwrap(), // Use the layout for deallocation
                );
            }
        }
    }
} 
