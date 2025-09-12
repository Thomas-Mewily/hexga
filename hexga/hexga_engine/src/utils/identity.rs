use super::*;


/* 
pub trait IIDentity<T> : Deref<Target=T> + DerefMut {}
impl<T,S> IIDentity<T> for S where S:  Deref<Target=T> + DerefMut{}

pub type Identity<T> = T;
*/

/// Act like a dirty flag. When deref_mut is applied, mark it dirty/consumed
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct IdentityDirty<T>
{
    value: T,
    dirty: bool,
}
impl<T> IdentityDirty<T>
{
    pub fn from_value(value: T) -> Self { Self::from_value_and_consumed(value, false) }
    pub fn from_value_and_consumed(value: T, consumed: bool) -> Self { Self { value, dirty: consumed }}

    pub fn consume(&mut self) -> &mut Self { self.dirty() }
    pub fn unconsume(&mut self) -> &mut Self { self.undirty() }
    pub fn is_consumed(&self) -> bool { self.is_dirty() }
    pub fn set_consumed(&mut self, consumed: bool) -> &mut Self { self.set_dirty(consumed) }

    pub fn dirty(&mut self) -> &mut Self { self.set_dirty(true) }
    pub fn undirty(&mut self) -> &mut Self { self.set_dirty(false) }
    pub fn is_dirty(&self) -> bool { self.dirty }
    pub fn set_dirty(&mut self, dirty: bool) -> &mut Self { self.dirty = dirty; self }

    pub fn into_value_and_dirty(self) -> (T, bool) { (self.value, self.dirty) }
    pub fn into_value_and_consumed(self) -> (T, bool) { (self.value, self.dirty) }
}
impl<T> Deref for IdentityDirty<T>
{
    type Target=T;
    fn deref(&self) -> &Self::Target { &self.value }
}
impl<T> DerefMut for IdentityDirty<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target 
    { 
        self.consume();
        &mut self.value 
    }
}
