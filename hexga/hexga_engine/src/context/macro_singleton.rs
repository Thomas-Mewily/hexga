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
                Self::try_as_ref().expect("Ctx not initialized")
            }
        }

        impl SingletonMut for $wrapper {
            fn try_as_mut() -> Option<&'static mut <Self as SingletonRef>::Target> {
                $try_as_mut
            }
        }

        impl ::std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                Self::try_as_mut().expect("Ctx not initialized")
            }
        }
    };
}
pub(crate) use ctx_singleton;