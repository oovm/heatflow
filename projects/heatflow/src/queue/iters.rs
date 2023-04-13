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
        if self.index >= self.queue.capacity {
            None
        } else {
            let result = self.queue.get(self.index);
            self.index += 1;
            Some(result)
        }
    }
}

impl<'a, T> Iterator for MutCircular<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.queue.capacity {
            None
        } else {
            let result = self.queue.get_mut(self.index);
            self.index += 1;
            Some(result)
        }
    }
}

