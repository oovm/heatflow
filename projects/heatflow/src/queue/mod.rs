use std::ptr::{NonNull};
use crate::Error;

pub mod iters;

/// A first-in-first-out queue with fixed size.
pub struct Circular<T> {
    data: Box<[T]>,
    current: usize,
}


impl<T: Default> Circular<T> {
    /// Creates a new FIFO with the given capacity.
    pub fn new(capacity: usize) -> Self {
        assert_ne!(capacity, 0, "Cannot create an empty queue");
        // SAFETY: We are allocating memory for the queue and initializing it with default values.
        unsafe {
            let mut data = Vec::with_capacity(capacity);
            data.set_len(capacity);
            for i in 0..capacity {
                std::ptr::write(data.as_mut_ptr().add(i), T::default());
            }
            Self {
                data: data.into_boxed_slice(),
                current: 0,
            }
        }
    }
}

impl<T> Circular<T> {
    /// Pushes a new element to the queue, it will overwrite the oldest element.
    pub fn push(&mut self, element: T) {
        // SAFETY: We are writing to a valid memory location.
        unsafe {
            let index = self.pointer.as_ptr().add(self.current);
            self.current = (self.current + 1) % self.capacity;
            std::ptr::write(index, element);
        }
    }
    pub fn get(&self, index: usize) -> &T {
        // SAFETY: We are reading from a valid memory location.
        // when index >= self.capacity, the modulo operation will make it smaller than self.capacity
        unsafe {
            let m_index = self.pointer.as_ptr().add(index % self.capacity);
            &std::ptr::read(m_index)
        }
    }
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        // SAFETY: We are reading from a valid memory location.
        // when index >= self.capacity, the modulo operation will make it smaller than self.capacity
        unsafe {
            let m_index = self.pointer.as_ptr().add(index % self.capacity);
            &mut std::ptr::read(m_index)
        }
    }

    pub fn newest(&self) -> &T {
        self.get(0)
    }
    pub fn oldest(&self) -> &T {
        self.get(self.capacity - 1)
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}