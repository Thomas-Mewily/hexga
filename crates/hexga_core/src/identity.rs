use super::*;

pub mod prelude
{
    //pub use super::traits::*;
    pub use super::{identity, Identity};
}
pub mod traits {}


pub fn identity<T>(value: T) -> T { value }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct Identity<T>(pub T);
impl<T> Identity<T>
{
    pub const fn new(value: T) -> Self { Self(value) }
}

impl<T> AsRef<T> for Identity<T>
{
    fn as_ref(&self) -> &T { &self.0 }
}
impl<T> AsMut<T> for Identity<T>
{
    fn as_mut(&mut self) -> &mut T { &mut self.0 }
}
impl<T> Has<T> for Identity<T>
where
    T: Clone,
{
    fn retrieve(&self) -> T { self.0.clone() }
}
impl<T> HasRef<T> for Identity<T>
{
    fn retrive_ref(&self) -> &T { &self.0 }
}
impl<T> HasMut<T> for Identity<T>
{
    fn retrive_mut(&mut self) -> &mut T { &mut self.0 }
}

impl<T> From<T> for Identity<T>
{
    fn from(value: T) -> Self { Self(value) }
}
impl<T> Deref for Identity<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<T> DerefMut for Identity<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
impl<T> Guarded<T> for Identity<T>
{
    type Guard<'a>
        = &'a T
    where
        Self: 'a;
    fn get<'a>(&'a self) -> Self::Guard<'a> { self }

    type Error<'a>
        = ()
    where
        Self: 'a;

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { Ok(self) }
}
