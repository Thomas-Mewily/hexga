use super::*;

pub mod prelude
{
    pub use super::{Dirty,DirtyFlag};
}

pub trait Dirty
{
    fn is_dirty(&self) -> bool;
    fn set_dirty(&mut self, used: bool) -> &mut Self;
    fn mark_dirty(&mut self) -> &mut Self { self.set_dirty(true) }
    fn clear_dirty(&mut self) -> &mut Self { self.set_dirty(false) }
}

/// Mark value as dirty when mutated (using [`DerefMut`])
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub struct DirtyFlag<T>
{
    value: T,
    used : bool,
}
impl<T> From<T> for DirtyFlag<T> { fn from(value: T) -> Self { Self::new(value) } }
impl<T> DirtyFlag<T>
{
    pub fn new(value: T) -> Self { Self::with_used(value, false) }
    pub fn with_used(value: T, used: bool) -> Self { Self { value, used }}

    pub fn into_value(self) -> T { self.value }
    pub fn into_value_and_used(self) -> (T,bool) { (self.value, self.used) }
}
impl<T> Deref for DirtyFlag<T>
{
    type Target=T;
    fn deref(&self) -> &Self::Target { &self.value }
}
impl<T> DerefMut for DirtyFlag<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.mark_dirty(); &mut self.value }
}
impl<T> Dirty for DirtyFlag<T>
{
    fn is_dirty(&self) -> bool { self.used }
    fn set_dirty(&mut self, used: bool) -> &mut Self { self.used = used; self }
}


// Todo: DirtyHash, DirtyCounter...