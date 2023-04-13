use super::*;

pub struct GetCircular<'a, T> {
    queue: &'a Circular<T>,
    index: usize,
}

pub struct MutCircular<'a, T> {
    queue: &'a mut Circular<T>,
    index: usize,
}

impl<'a, T> Iterator for GetCircular<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.queue.capacity() {
            return None;
        }
        let result = self.queue.get(self.index);
        self.index += 1;
        Some(result)
    }
}

impl<'a, T> Iterator for MutCircular<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.queue.capacity() {
            return None;
        }
        unsafe {
            let result = self.queue.get_ptr(self.index);
            self.index += 1;
            Some(&mut *result)
        }
    }
}

impl<T> Circular<T> {
    pub fn get_items(&self) -> GetCircular<T> {
        GetCircular {
            queue: self,
            index: 0,
        }
    }
    pub fn mut_items(&mut self) -> MutCircular<T> {
        MutCircular {
            queue: self,
            index: 0,
        }
    }
}