#![feature(unsafe_cell_access)]
//! Define Single and Multi threaded Singleton.
//!
//! Inspired by [RusPiRo Singleton crate](https://github.com/RusPiRo/ruspiro-singleton)
//!
//! ```
//! use hexga_singleton::prelude::*;
//!
//! pub struct CurrentUser {
//!     pub name: String,
//! }
//!
//! static USER : SingletonSingleThread<CurrentUser> = SingletonSingleThread::new(|| CurrentUser { name: "Foo".to_owned() });
//!
//! hexga_singleton::singleton_single_thread_access!(pub User, CurrentUser, USER);
//!
//!
//! fn main() {
//!     assert_eq!(User.name, "Foo");
//!     User.name = "Bar".to_owned();
//!     assert_eq!(User.name, "Bar");
//! }
//! ```

use hexga_core::prelude::*;

use std::{cell::*,sync::*,ops::{Deref,DerefMut}};
pub use hexga_core::guard;

mod singleton;
pub use singleton::*;

#[doc(hidden)]
pub use hexga_core::guard::{ReadGuard,WriteGuard};

pub mod prelude
{
    pub use super::{SingletonSingleThread,SingletonMultiThread};
    pub use super::guard::*;
}

/*
#[macro_export]
macro_rules! singleton_single_thread_access {
    ($(#[$attr:meta])* $vis:vis $empty_struct_name: ident, $target_type: ident, $singleton_name: ident) => {
        $(#[$attr])* $vis struct $empty_struct_name;

        impl ::std::ops::Deref for $empty_struct_name
        {
            type Target = $target_type;
            #[inline(always)]
            #[track_caller]
            fn deref(&self) -> &Self::Target
            {
                $crate::guard::ReadGuard::read($singleton_name)
            }
        }
        impl ::std::ops::DerefMut for $empty_struct_name
        {
            #[inline(always)]
            #[track_caller]
            fn deref_mut(&mut self) -> &mut Self::Target
            {
                $crate::guard::WriteGuard::write($singleton_name)
            }
        }
    };


    ($(#[$attr:meta])* $vis:vis $empty_struct_name: ident, $target_type: ident, $try_borrow:block, $try_borrow_mut:block) => {
        $(#[$attr])* $vis struct $empty_struct_name;
        impl $crate::TryBorrow for $empty_struct_name
        {
            type Borrow=$target_type;
            type Error=();

            fn try_borrow(self) -> Result<Self::Borrow, Self::Error> {
                $try_borrow
            }
        }
        impl $crate::TryBorrowMut for $empty_struct_name
        {
            type Borrow=$target_type;
            type Error=();

            fn try_borrow_mut(self) -> Result<Self::Borrow, Self::Error> {
                $try_borrow_mut
            }
        }

        impl ::std::ops::Deref for $empty_struct_name
        {
            type Target = $target_type;
            #[inline(always)]
            #[track_caller]
            fn deref(&self) -> &Self::Target
            {
                $crate::TryBorrow::borrow(self)
            }
        }
        impl ::std::ops::DerefMut for $empty_struct_name
        {
            #[inline(always)]
            #[track_caller]
            fn deref_mut(&mut self) -> &mut Self::Target
            {
                $crate::TryBorrow_mut::borrow_mut(self)
            }
        }
    };
}
*/