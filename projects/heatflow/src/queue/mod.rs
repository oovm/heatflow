use std::fmt::Debug;
use std::ptr::NonNull;

pub mod iters;

/// A first-in-first-out queue with fixed size.
pub struct Circular<T> {
    pointer: NonNull<T>,
    capacity: usize,
    current: usize,
}


impl<T: Default> Circular<T> {
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
    unsafe fn get_ptr(&self, index: usize) -> *mut T {
        let modulo = index % self.capacity;
        self.pointer.as_ptr().add(modulo)
    }
    pub fn get(&self, index: usize) -> &T {
        // SAFETY: We are reading from a valid memory location.
        // when index >= self.capacity, the modulo operation will make it smaller than self.capacity
        unsafe {
            &*self.get_ptr(index)
        }
    }
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        // SAFETY: We are reading from a valid memory location.
        // when index >= self.capacity, the modulo operation will make it smaller than self.capacity
        unsafe {
            &mut *self.get_ptr(index)
        }
    }
    pub fn newest(&self) -> &T {
        // SAFETY: We are reading from a valid memory location.
        unsafe {
            &*self.get_ptr(self.current)
        }
    }
    pub fn oldest(&self) -> &T {
        // SAFETY: We are reading from a valid memory location.
        unsafe {
            &*self.get_ptr(self.current + 1)
        }
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T: Debug> Debug for Circular<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.get_items()).finish()
    }
}

#[test]
pub fn test() {
    let mut queue = Circular::<u8>::new(5);
    assert_eq!(std::mem::size_of::<Circular<u8>>(), 24);

    println!("{:?}", queue);
    for i in 1..10 {
        queue.push(i);
        println!("{:?}", queue);
    }
    println!("oldest: {}", queue.oldest());
    println!("newest: {}", queue.newest());
}