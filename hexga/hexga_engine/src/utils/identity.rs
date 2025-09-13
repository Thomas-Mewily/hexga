use super::*;


/* 
pub trait IIDentity<T> : Deref<Target=T> + DerefMut {}
impl<T,S> IIDentity<T> for S where S:  Deref<Target=T> + DerefMut{}

pub type Identity<T> = T;
*/

/// Act like a dirty flag. When deref_mut is applied, mark it dirty/consumed
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct IdentityUsed<T>
{
    value: T,
    used: bool,
}
impl<T> IdentityUsed<T>
{
    pub fn from_value(value: T) -> Self { Self::from_value_and_used(value, false) }
    pub fn from_value_and_used(value: T, consumed: bool) -> Self { Self { value, used: consumed }}

    pub fn use_it(&mut self) -> &mut Self { self.set_used(true) }
    pub fn unused_it(&mut self) -> &mut Self { self.set_used(false) }
    pub fn is_used(&self) -> bool { self.used }
    pub fn set_used(&mut self, used: bool) -> &mut Self { self.used = used; self }

    /* 
    pub fn dirty(&mut self) -> &mut Self { self.set_dirty(true) }
    pub fn undirty(&mut self) -> &mut Self { self.set_dirty(false) }
    pub fn is_dirty(&self) -> bool { self.dirty }
    pub fn set_dirty(&mut self, dirty: bool) -> &mut Self { self.dirty = dirty; self }

    pub fn into_value_and_dirty(self) -> (T, bool) { (self.value, self.dirty) }
    */
    pub fn into_value_and_used(self) -> (T, bool) { (self.value, self.used) }
}
impl<T> Deref for IdentityUsed<T>
{
    type Target=T;
    fn deref(&self) -> &Self::Target { &self.value }
}
impl<T> DerefMut for IdentityUsed<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target 
    { 
        self.use_it();
        &mut self.value 
    }
}
