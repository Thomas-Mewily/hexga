use super::*;

pub trait TryAsRef<T>
{
    type Error;
    fn try_as_ref(&self) -> Result<&T, Self::Error>;
}
impl<T, S> TryAsRef<T> for S
where
    S: AsRef<T>,
{
    type Error = Never;
    fn try_as_ref(&self) -> Result<&T, Self::Error> { Ok(self.as_ref()) }
}

/*
pub trait TryAsMut<T>
{
    type Error: Debug;
    fn try_as_mut(&mut self) -> Result<&mut T,Self::Error>;
}
impl<T,S> TryAsMut<T> for S where S: AsMut<T>
{
    type Error=Never;
    fn try_as_mut(&mut self) -> Result<&mut T,Self::Error> { Ok(self.as_mut()) }
}
*/
