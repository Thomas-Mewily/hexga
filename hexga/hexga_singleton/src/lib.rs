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
//!     fn replace(value: Option<<Self as SingletonRef>::Target>) -> SingletonResult {
//!         CURRENT_USER.replace(value);
//!         Ok(())
//!     }
//! }
//!
//!
//! fn main()
//! {
//!     assert!(User::is_not_init());
//!
//!     // init
//!     User::replace(Some( CurrentUser { name: "Foo".to_owned() })).unwrap();
//!     assert!(User::is_init());
//!
//!     // Singleton access
//!     let name = &User.name;
//!
//!     // de init
//!     User::replace(None).unwrap();
//!     assert!(User::is_not_init());
//! }
//! ```

use std::ops::{Deref, DerefMut};

pub mod prelude
{
    pub use super::*;
}

pub type SingletonResult = Result<(), ()>;

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
    fn replace(value: Option<<Self as SingletonRef>::Target>) -> SingletonResult;

    /// Replace the value if already init
    fn init(value: <Self as SingletonRef>::Target) -> SingletonResult
    {
        Self::replace(::core::option::Option::Some(value))
    }

    /// Replace the value if already init
    fn init_default() -> SingletonResult where <Self as SingletonRef>::Target: Default
    {
        if Self::try_as_mut().is_none()
        {
            Self::replace(::core::option::Option::Some(Default::default()))
        }else
        {
            Ok(())
        }
    }

    fn destroy() -> SingletonResult
    {
        Self::replace(::core::option::Option::None)
    }
}


#[macro_export]
macro_rules! singleton_thread_local {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $target:ty, $constant_static_name:ident) => {

        $crate::singleton_declare_thread_local!(
            $(#[$attr])* $vis $wrapper, $target, $constant_static_name
        );

        impl $crate::SingletonInit for $wrapper {
            fn replace(value: ::core::option::Option<<Self as $crate::SingletonRef>::Target>) -> $crate::SingletonResult {
                $constant_static_name.with(|cell| {
                    *cell.borrow_mut() = value;
                });
                ::core::result::Result::Ok(())
            }
        }
    };
}


#[macro_export]
macro_rules! singleton_declare_thread_local {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $target:ty, $constant_static_name:ident) => {
        thread_local! {
            pub(crate) static $constant_static_name: std::cell::RefCell<Option<$target>> = std::cell::RefCell::new(::core::option::Option::None);
        }

        $crate::singleton_access!($(#[$attr])* $vis $wrapper, $target,
            {
                $constant_static_name.with(|ctx_cell| {
                    if let ::core::option::Option::Some(rc_ctx) = ctx_cell.borrow().as_ref() {
                        let ctx_ptr: *const $target = rc_ctx;
                        unsafe { ::core::option::Option::Some(&*ctx_ptr) }
                    } else {
                        ::core::option::Option::None
                    }
                })
            },
            {
                $constant_static_name.with(|ctx_cell| {
                    if let ::core::option::Option::Some(rc_ctx) = ctx_cell.borrow_mut().as_mut() {
                        let ctx_ptr: *mut $target = rc_ctx;
                        unsafe { ::core::option::Option::Some(&mut *ctx_ptr) }
                    } else {
                        ::core::option::Option::None
                    }
                })
            }
        );
    };
}



#[macro_export]
macro_rules! singleton_declare_multi_thread {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $target:ty, $constant_static_name:ident) => {
        static $constant_static_name: ::std::sync::OnceLock<::std::sync::Mutex<::core::option::Option<$target>>> = ::std::sync::OnceLock::new();

        $crate::singleton_access!(
            $(#[$attr])* $vis $wrapper, $target,
            {
                let cell: &'static ::std::sync::Mutex<::core::option::Option<$target>> =
                    $constant_static_name.get_or_init(|| ::std::sync::Mutex::new(::core::option::Option::None));

                match cell.lock() {
                    ::core::result::Result::Ok(guard) => {
                        if let ::core::option::Option::Some(ctx) = guard.as_ref() {
                            let ctx_ptr: *const $target = ctx;
                            // SAFETY: The lifetime is `'instantaneous`, not `'static`
                            unsafe { ::core::option::Option::Some(&*ctx_ptr) }
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    ::core::result::Result::Err(_poisoned) => {
                        ::core::option::Option::None
                    }
                }
            },
            {
                let cell: &'static ::std::sync::Mutex<::core::option::Option<$target>> =
                    $constant_static_name.get_or_init(|| ::std::sync::Mutex::new(::core::option::Option::None));

                match cell.lock() {
                    ::core::result::Result::Ok(mut guard) => {
                        if let ::core::option::Option::Some(ctx) = guard.as_mut() {
                            let ctx_ptr: *mut $target = ctx;
                            // SAFETY: The lifetime is `'instantaneous`, not `'static`
                            unsafe { ::core::option::Option::Some(&mut *ctx_ptr) }
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    ::core::result::Result::Err(_poisoned) => {
                        ::core::option::Option::None
                    }
                }
            }
        );
    };
}

#[macro_export]
macro_rules! singleton_multi_thread {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $target:ty, $constant_static_name:ident) => {

        $crate::singleton_declare_multi_thread!(
            $(#[$attr])* $vis $wrapper, $target, $constant_static_name
        );

        impl $crate::SingletonInit for $wrapper {
            fn replace(value: ::core::option::Option<<Self as $crate::SingletonRef>::Target>) -> $crate::SingletonResult {
                let cell: &'static ::std::sync::Mutex<::core::option::Option<$target>> =
                    $constant_static_name.get_or_init(|| ::std::sync::Mutex::new(::core::option::Option::None));

                match cell.lock() {
                    ::core::result::Result::Ok(mut guard) => {
                        *guard = value;
                        ::core::result::Result::Ok(())
                    }
                    ::core::result::Result::Err(_poisoned) => {
                        ::core::result::Result::Err(())
                    }
                }
            }
        }
    };
}


#[macro_export]
macro_rules! singleton_declare_multi_thread_poison_resistant {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $target:ty, $constant_static_name:ident) => {
        static $constant_static_name: ::std::sync::OnceLock<::std::sync::Mutex<::core::option::Option<$target>>> = ::std::sync::OnceLock::new();

        $crate::singleton_access!(
            $(#[$attr])* $vis $wrapper, $target,
            {
                let cell: &'static ::std::sync::Mutex<::core::option::Option<$target>> =
                    $constant_static_name.get_or_init(|| ::std::sync::Mutex::new(::core::option::Option::None));

                match cell.lock() {
                    ::core::result::Result::Ok(guard) => {
                        if let ::core::option::Option::Some(ctx) = guard.as_ref() {
                            let ctx_ptr: *const $target = ctx;
                            // SAFETY: The lifetime is `'instantaneous`, not `'static`
                            unsafe { ::core::option::Option::Some(&*ctx_ptr) }
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    ::core::result::Result::Err(poisoned) => {
                        let guard = poisoned.into_inner();
                        if let ::core::option::Option::Some(ctx) = guard.as_ref()
                        {
                            let ctx_ptr: *const $target = ctx;
                            unsafe { ::core::option::Option::Some(&*ctx_ptr) }
                        } else
                        {
                            ::core::option::Option::None
                        }
                    }
                }
            },
            {
                let cell: &'static ::std::sync::Mutex<::core::option::Option<$target>> =
                    $constant_static_name.get_or_init(|| ::std::sync::Mutex::new(::core::option::Option::None));

                match cell.lock() {
                    ::core::result::Result::Ok(mut guard) => {
                        if let ::core::option::Option::Some(ctx) = guard.as_mut() {
                            let ctx_ptr: *mut $target = ctx;
                            // SAFETY: The lifetime is `'instantaneous`, not `'static`
                            unsafe { ::core::option::Option::Some(&mut *ctx_ptr) }
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    ::core::result::Result::Err(poisoned) => {
                        let guard = poisoned.into_inner();
                        if let ::core::option::Option::Some(ctx) = guard.as_ref()
                        {
                            let ctx_ptr: *const $target = ctx;
                            unsafe { ::core::option::Option::Some(&*ctx_ptr) }
                        } else
                        {
                            ::core::option::Option::None
                        }
                    }
                }
            }
        );
    };
}

#[macro_export]
macro_rules! singleton_multi_thread_poison_resistant {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $target:ty, $constant_static_name:ident) => {

        $crate::singleton_declare_multi_thread!(
            $(#[$attr])* $vis $wrapper, $target, $constant_static_name
        );

        impl $crate::SingletonInit for $wrapper {
            fn replace(value: ::core::option::Option<<Self as $crate::SingletonRef>::Target>) -> $crate::SingletonResult {
                let cell: &'static ::std::sync::Mutex<::core::option::Option<$target>> =
                    $constant_static_name.get_or_init(|| ::std::sync::Mutex::new(::core::option::Option::None));

                match cell.lock() {
                    ::core::result::Result::Ok(mut guard) => {
                        *guard = value;
                        ::core::result::Result::Ok(())
                    }
                    ::core::result::Result::Err(poisoned) => {
                        let mut guard = poisoned.into_inner();
                        *guard = value;
                        ::core::result::Result::Ok(())
                    }
                }
            }
        }
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

        impl $crate::SingletonRef for $wrapper {
            type Target = $target;

            fn try_as_ref() -> ::core::option::Option<&'static <Self as $crate::SingletonRef>::Target> {
                $try_as_ref
            }
        }

        impl ::std::ops::Deref for $wrapper {
            type Target = $target;
            fn deref(&self) -> &Self::Target { Self::as_ref() }
        }

        impl $crate::SingletonMut for $wrapper {
            fn try_as_mut() -> ::core::option::Option<&'static mut <Self as $crate::SingletonRef>::Target> { $try_as_mut }
        }

        impl ::std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                Self::as_mut()
            }
        }
    };
}