use super::*;

impl<T: Debug> Debug for Circular<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.get_items()).finish()
    }
}