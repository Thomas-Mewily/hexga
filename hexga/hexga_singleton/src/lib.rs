//! Define some Singleton traits and macro.
//!
//! ```
//! use hexga_singleton::prelude::*;
//!
//! pub struct CurrentUser
//! {
//!     pub name : String,
//! }
//!
//! singleton_thread_local!(pub User, CurrentUser, CURRENT_USER);
//!
//! // Custom logic to init / deinit the singleton
//! impl SingletonInit for User
//! {
//!     fn replace(value: Option<<Self as SingletonRef>::Target>) {
//!         CURRENT_USER.replace(value);
//!     }
//! }
//!
//!
//! fn main()
//! {
//!     assert!(User::is_not_init());
//!
//!     // init
//!     User::replace(Some( CurrentUser { name: "Foo".to_owned() }));
//!     assert!(User::is_init());
//!
//!     // Singleton access
//!     let _the_name = &User.name;
//!
//!     // de init
//!     User::replace(None);
//!     assert!(User::is_not_init());
//! }
//! ```

use std::ops::{Deref, DerefMut};

pub mod prelude
{
    pub use super::*;
}


pub trait Singleton : SingletonRef + SingletonMut {}
impl<T> Singleton for T where T: SingletonRef+SingletonMut {}

pub trait SingletonRef: Deref<Target = <Self as SingletonRef>::Target>
{
    type Target :'static;

    /// SAFETY: The lifetime is `'instantaneous`, not `static`, don't store it please
    fn try_as_ref() -> Option<&'static <Self as SingletonRef>::Target>;
    fn as_ref() -> &'static <Self as SingletonRef>::Target { Self::try_as_ref().unwrap() }

    fn is_init() -> bool { Self::try_as_ref().is_some() }
    fn is_not_init() -> bool { !Self::is_init() }
}

pub trait SingletonMut: SingletonRef + DerefMut
{
    /// SAFETY: The lifetime is `'instantaneous`, not `static`, don't store it please
    fn try_as_mut() -> Option<&'static mut <Self as SingletonRef>::Target>;
    fn as_mut() -> &'static mut <Self as SingletonRef>::Target { Self::try_as_mut().unwrap() }
}

pub trait SingletonInit: SingletonRef + SingletonMut
{
    fn replace(value: Option<<Self as SingletonRef>::Target>);

    /// Replace the value if already init
    fn init(value: <Self as SingletonRef>::Target)
    {
        Self::replace(Some(value));
    }

    /// Replace the value if already init
    fn init_default() where <Self as SingletonRef>::Target: Default
    {
        if Self::try_as_mut().is_none()
        {
            Self::replace(Some(Default::default()))
        }
    }

    fn destroy()
    {
        Self::replace(None)
    }
}


#[macro_export]
macro_rules! singleton_thread_local {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $target:ty, $constant_static_name:ident) => {
        thread_local! {
            pub(crate) static $constant_static_name: std::cell::RefCell<Option<$target>> = std::cell::RefCell::new(None);
        }

        $crate::singleton_access!($(#[$attr])* $vis $wrapper, $target,
            {
                $constant_static_name.with(|ctx_cell| {
                    if let Some(rc_ctx) = ctx_cell.borrow().as_ref() {
                        let ctx_ptr: *const $target = rc_ctx;
                        unsafe { Some(&*ctx_ptr) }
                    } else {
                        None
                    }
                })
            },
            {
                $constant_static_name.with(|ctx_cell| {
                    if let Some(rc_ctx) = ctx_cell.borrow_mut().as_mut() {
                        let ctx_ptr: *mut $target = rc_ctx;
                        unsafe { Some(&mut *ctx_ptr) }
                    } else {
                        None
                    }
                })
            }
        );
    };
}


#[macro_export]
macro_rules! singleton_access {
    (
        $(#[$attr:meta])* $vis:vis $wrapper:ident,
        $target:ty, $try_as_ref:block, $try_as_mut:block
    ) => {
        $(#[$attr])*
        $vis struct $wrapper;

        impl SingletonRef for $wrapper {
            type Target = $target;

            fn try_as_ref() -> Option<&'static <Self as SingletonRef>::Target> {
                $try_as_ref
            }
        }

        impl ::std::ops::Deref for $wrapper {
            type Target = $target;
            fn deref(&self) -> &Self::Target { Self::as_ref() }
        }

        impl SingletonMut for $wrapper {
            fn try_as_mut() -> Option<&'static mut <Self as SingletonRef>::Target> { $try_as_mut }
        }

        impl ::std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                Self::as_mut()
            }
        }
    };
}