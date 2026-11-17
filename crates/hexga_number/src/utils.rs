pub trait RefMutByValue
{
    type Item;
    fn by_value<F>(self, f: F) where F: FnOnce(Self::Item) -> Self::Item;
}
impl<T> RefMutByValue for &mut T {
    type Item = T;

    /// Replaces the pointee with `f(old_value)`, moving the old value into `f`.
    ///
    /// # Panics
    /// 
    /// `f` must not panic. If it does, behavior is **undefined**: the slot is
    /// left logically uninitialized between `ptr::read` and `ptr::write`, and
    /// unwinding will double-drop. No abort, no guard.
    fn by_value<F>(self, f: F)
    where
        F: FnOnce(Self::Item) -> Self::Item,
    {
        // SAFETY: `&mut T` gives exclusive access. `ptr::read` moves the value
        // out (one owner); `f` consumes it (FnOnce); `ptr::write` overwrites
        // without dropping the moved-out bits. The slot is uninitialized only
        // between read and write — sound iff `f` does not panic (see # Panics).
        unsafe {
            let old = std::ptr::read(self);
            let new = f(old);
            std::ptr::write(self, new);
        }
    }
}