use std::fmt::Debug;
use std::ptr::NonNull;

pub mod iters;
mod display;

/// A first-in-first-out queue with fixed size.
pub struct Circular<const N: usize, T> {
    pointer: NonNull<T>,
    capacity: usize,
    current: usize,
}


impl<const N: usize, T: Default> Circular<N, T> {
    /// Creates a new FIFO with the given capacity.
    pub fn new(capacity: usize) -> Self {
        assert_ne!(capacity, 0, "Cannot create an empty queue");
        // SAFETY: We are allocating memory for the queue and initializing it with default values.
        unsafe {
            let pointer = std::alloc::alloc(std::alloc::Layout::array::<T>(capacity).expect("Invalid layout"));
            let pointer = NonNull::new_unchecked(pointer as *mut T);
            for i in 0..capacity {
                std::ptr::write(pointer.as_ptr().add(i), T::default());
            }
            Self {
                pointer,
                capacity,
                current: 0,
            }
        }
    }
}

impl<const N: usize, T> Circular<N, T> {
    /// Pushes a new element to the queue, it will overwrite the oldest element.
    pub fn push(&mut self, element: T) {
        // SAFETY: We are writing to a valid memory location.
        unsafe {
            let index = self.pointer.as_ptr().add(self.current);
            self.current = (self.current + 1) % self.capacity;
            std::ptr::write(index, element);
        }
    }
    /// Returns the capacity of the queue.
    ///
    /// # Safety
    ///
    /// This function is safe to call.
    ///
    /// # Examples
    ///
    /// ```
    /// use heatflow::Circular;
    ///
    ///
    /// ```
    unsafe fn get_ptr(&self, index: usize) -> *mut T {
        assert!(index < self.capacity, "{index} is out of bounds, capacity only {capacity}.", index = index, capacity = self.capacity);
        let m_index = (self.current + index) % self.capacity;
        self.pointer.as_ptr().add(m_index)
    }
    pub fn get(&self, index: usize) -> &T {
        // SAFETY: We are reading from a valid memory location.
        unsafe {
            &*self.get_ptr(index)
        }
    }
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        // SAFETY: We are reading from a valid memory location.
        unsafe {
            &mut *self.get_ptr(index)
        }
    }
    pub fn newest(&self) -> &T {
        self.get(self.capacity - 1)
    }
    pub fn oldest(&self) -> &T {
        self.get(0)
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}



