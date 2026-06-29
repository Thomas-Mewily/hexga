use super::*;

pub mod prelude
{
    pub use super::Dirty;
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{IsDirty,SetDirty};
}

pub trait IsDirty
{
    fn is_dirty(&self) -> bool;
    fn is_not_dirty(&self) -> bool { !self.is_dirty() }
}
pub trait SetDirty: IsDirty
{
    fn set_dirty(&mut self, used: bool) -> &mut Self;
    fn mark_dirty(&mut self) -> &mut Self { self.set_dirty(true) }
    fn undirty(&mut self) -> &mut Self { self.set_dirty(false) }
}

/// A Dirty flag that is automatically marked as dirty when mutated (using [`DerefMut`])
#[derive(Clone, Copy, Eq, Debug, Default)]
pub struct Dirty<T>
{
    value: T,
    used: bool,
}
impl<T> PartialEq for Dirty<T>
    where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T> PartialOrd for Dirty<T>
    where T: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl<T> Ord for Dirty<T>
    where T: Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(other)
    }
}
impl<T> Hash for Dirty<T>
    where T: Hash
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
impl<T> From<T> for Dirty<T>
{
    fn from(value: T) -> Self { Self::new(value) }
}
impl<T> Dirty<T>
{
    pub fn new_dirty(value: T) -> Self { Self::with_dirty(value, true) }
    /// Create a new undirty value
    pub fn new(value: T) -> Self { Self::with_dirty(value, false) }
    pub fn with_dirty(value: T, dirty: bool) -> Self { Self { value, used: dirty } }

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
