use super::*;

pub struct GetCircular<'a, const N: usize,T> {
    queue: &'a  Circular<N, T>,
    index: usize,
}

pub struct MutCircular<'a, const N: usize,T> {
    queue: &'a mut  Circular<N, T>,
    index: usize,
}

impl<'a, const N: usize,T> Iterator for GetCircular<'a, N, T> {
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

impl<'a, const N: usize,T> Iterator for MutCircular<'a, N, T> {
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

impl<const N: usize, T> Circular<N, T> {
    pub fn get_items(&self) -> GetCircular<N, T> {
        GetCircular {
            queue: self,
            index: 0,
        }
    }
    pub fn mut_items(&mut self) -> MutCircular<N, T> {
        MutCircular {
            queue: self,
            index: 0,
        }
    }
}