use crate::*;

pub mod prelude
{
    pub use super::Singleton;
}

pub trait Singleton<T>: Deref<Target = T> + DerefMut + AsRef<T> + AsMut<T> where T:'static
{
    /// SAFETY: The lifetime is `'instantaneous`, not `static`, don't store it please
    fn try_as_ref() -> Option<&'static T>;
    /// SAFETY: The lifetime is `'instantaneous`, not `static`, don't store it please
    fn try_as_mut() -> Option<&'static mut T>;
    fn replace(instance: Option<T>);

    /// Do nothings if it is already init
    fn init() where T:Default 
    {
        if Self::try_as_mut().is_none()
        {
            Self::replace(Some(___())) 
        } 
    }
    fn destroy()
    {
        Self::replace(None) 
    }

    fn is_init() -> bool { Self::try_as_ref().is_some() }
}

