use super::*;


pub trait Singleton : SingletonRef + SingletonMut {}
impl<T> Singleton for T where T: SingletonRef+SingletonMut {}

pub trait SingletonRef: Deref<Target = <Self as SingletonRef>::Target>
{
    type Target :'static;

    /// SAFETY: The lifetime is `'instantaneous`, not `static`, don't store it please
    fn try_as_ref() -> Option<&'static <Self as SingletonRef>::Target>;
    fn as_ref() -> &'static <Self as SingletonRef>::Target { Self::try_as_ref().unwrap() }

    fn is_init() -> bool { Self::try_as_ref().is_some() }
}

pub trait SingletonMut: SingletonRef + DerefMut
{
    /// SAFETY: The lifetime is `'instantaneous`, not `static`, don't store it please
    fn try_as_mut() -> Option<&'static mut <Self as SingletonRef>::Target>;
    fn as_mut() -> &'static mut <Self as SingletonRef>::Target { Self::try_as_mut().unwrap() }
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


#[macro_export]
macro_rules! singleton {
    ($wrapper:ident, $target:ty, $try_as_ref:block, $try_as_mut:block) => {
        pub struct $wrapper;

        impl SingletonRef for $wrapper {
            type Target = $target;

            fn try_as_ref() -> Option<&'static <Self as SingletonRef>::Target> {
                $try_as_ref
            }
        }

        impl ::std::ops::Deref for $wrapper 
        {
            type Target = $target;
            fn deref(&self) -> &Self::Target { Self::as_ref() }
        }

        impl SingletonMut for $wrapper 
        {
            fn try_as_mut() -> Option<&'static mut <Self as SingletonRef>::Target> { $try_as_mut }
        }

        impl ::std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                Self::as_mut()
            }
        }
    };
}
pub use singleton;