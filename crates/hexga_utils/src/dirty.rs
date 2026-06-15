use super::*;

pub mod prelude
{
    pub use super::Dirty;
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::IsDirty;
}

pub trait IsDirty
{
    fn is_dirty(&self) -> bool;
}
pub trait SetDirty: IsDirty
{
    fn set_dirty(&mut self, used: bool) -> &mut Self;
    fn mark_dirty(&mut self) -> &mut Self { self.set_dirty(true) }
    fn clear_dirty(&mut self) -> &mut Self { self.set_dirty(false) }
}

/// A Dirty flag that is automatically marked as dirty when mutated (using [`DerefMut`])
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub struct Dirty<T>
{
    value: T,
    used: bool,
}
impl<T> From<T> for Dirty<T>
{
    fn from(value: T) -> Self { Self::new(value) }
}
impl<T> Dirty<T>
{
    pub fn new_dirty(value: T) -> Self { Self::with_used(value, true) }
    pub fn new(value: T) -> Self { Self::with_used(value, false) }
    pub fn with_used(value: T, used: bool) -> Self { Self { value, used } }

    pub fn into_value(self) -> T { self.value }
    pub fn into_value_and_used(self) -> (T, bool) { (self.value, self.used) }
}
impl<T> Deref for Dirty<T>
{
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.value }
}
impl<T> DerefMut for Dirty<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        self.mark_dirty();
        &mut self.value
    }
}
impl<T> IsDirty for Dirty<T>
{
    fn is_dirty(&self) -> bool { self.used }
}
impl<T> SetDirty for Dirty<T>
{
    fn set_dirty(&mut self, used: bool) -> &mut Self
    {
        self.used = used;
        self
    }
}

// Todo: DirtyHash, DirtyCounter...
