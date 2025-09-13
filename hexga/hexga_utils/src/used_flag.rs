use super::*;


/// Dirty flag trait
pub trait IUsedFlag
{
    fn is_used(&self) -> bool;
    fn set_used(&mut self, used: bool) -> &mut Self;
    fn use_it(&mut self) -> &mut Self { self.set_used(true) }
    fn unuse_it(&mut self) -> &mut Self { self.set_used(false) }
}

/// Mark value as dirty when mutated (using [DerefMut])
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub struct UsedFlag<T>
{
    value: T,
    used : bool,
}
impl<T> From<T> for UsedFlag<T> { fn from(value: T) -> Self { Self::new(value) } }
impl<T> UsedFlag<T>
{
    pub fn new(value: T) -> Self { Self::with_used(value, false) }
    pub fn with_used(value: T, used: bool) -> Self { Self { value, used }}

    pub fn into_value(self) -> T { self.value }
    pub fn into_value_and_used(self) -> (T,bool) { (self.value, self.used) }
}
impl<T> Deref for UsedFlag<T>
{
    type Target=T;
    fn deref(&self) -> &Self::Target { &self.value }
}
impl<T> DerefMut for UsedFlag<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.use_it(); &mut self.value }
}
impl<T> IUsedFlag for UsedFlag<T>
{
    fn is_used(&self) -> bool { self.used }
    fn set_used(&mut self, used: bool) -> &mut Self { self.used = used; self }
}