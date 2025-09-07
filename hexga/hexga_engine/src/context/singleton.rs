use super::*;


pub mod prelude
{
    pub(crate) use super::ctx_singleton;
}

macro_rules! ctx_singleton {
    ($wrapper:ident, $target:ty, $try_as_ref:block, $try_as_mut:block) => {
        pub struct $wrapper;

        impl SingletonRef for $wrapper {
            type Target = $target;

            fn try_as_ref() -> Option<&'static <Self as SingletonRef>::Target> {
                $try_as_ref
            }
        }

        impl ::std::ops::Deref for $wrapper {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                Self::try_as_ref().expect(concat!(stringify!($wrapper), " not initialized"))
            }
        }

        impl AsRef<$target> for $wrapper {
            fn as_ref(&self) -> &$target {
                self.deref()
            }
        }

        impl SingletonMut for $wrapper {
            fn try_as_mut() -> Option<&'static mut <Self as SingletonRef>::Target> {
                $try_as_mut
            }
        }

        impl ::std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_mut()
            }
        }

        impl AsMut<$target> for $wrapper {
            fn as_mut(&mut self) -> &mut $target {
                Self::try_as_mut().expect(concat!(stringify!($wrapper), " not initialized (mut)"))
            }
        }
    };
}
pub(crate) use ctx_singleton;