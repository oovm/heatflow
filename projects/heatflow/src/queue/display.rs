use super::*;

impl<const N: usize, T: Debug> Debug for Circular<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.get_items()).finish()
    }
}