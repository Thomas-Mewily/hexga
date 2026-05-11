use super::*;

pub fn identity<T>(value: T) -> T { value }
pub struct Identity<T>
{
    pub value: T,
}
impl<T> Identity<T>
{
    pub const fn new(value: T) -> Self { Self { value } } 
}

impl<T> AsRef<T> for Identity<T>
{
    fn as_ref(&self) -> &T {
        &self.value
    }
}
impl<T> AsMut<T> for Identity<T>
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
impl<T> Has<T> for Identity<T>
    where T: Clone
{
    fn retrieve(&self) -> T {
        self.value.clone()
    }
}
impl<T> HasRef<T> for Identity<T>
{
    fn retrive_ref(&self) -> &T {
        &self.value
    }
}
impl<T> HasMut<T> for Identity<T>
{
    fn retrive_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> From<T> for Identity<T>
{
    fn from(value: T) -> Self {
        Self { value }
    }
}
impl<T> Deref for Identity<T>
{
    type Target=T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T> DerefMut for Identity<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
impl<T> Guarded<T> for Identity<T>
{
    type Guard<'a> = &'a T where Self: 'a;
    fn get<'a>(&'a self) -> Self::Guard<'a> {
        self
    }
}