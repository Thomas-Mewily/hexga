use crate::*;

pub mod prelude
{
    pub use super::{SingletonRef,SingletonMut,SingletonInit};
}

pub trait Singleton : SingletonRef + SingletonMut {}
impl<T> Singleton for T where T: SingletonRef+SingletonMut {}

pub trait SingletonRef: Deref<Target = <Self as SingletonRef>::Target> + AsRef<<Self as SingletonRef>::Target>
{
    type Target :'static;

    /// SAFETY: The lifetime is `'instantaneous`, not `static`, don't store it please
    fn try_as_ref() -> Option<&'static <Self as SingletonRef>::Target>;
    fn is_init() -> bool { Self::try_as_ref().is_some() }
}

pub trait SingletonMut: SingletonRef + DerefMut + AsMut<<Self as SingletonRef>::Target>
{
    /// SAFETY: The lifetime is `'instantaneous`, not `static`, don't store it please
    fn try_as_mut() -> Option<&'static mut <Self as SingletonRef>::Target>;
}

pub trait SingletonInit: SingletonRef + SingletonMut
{
    fn replace(instance: Option<<Self as SingletonRef>::Target>);

    /// Do nothings if it is already init
    fn init() where <Self as SingletonRef>::Target: Default 
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
}