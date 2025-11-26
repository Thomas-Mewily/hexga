#![feature(mapped_lock_guards)]
#![feature(lazy_get)]
//! Define Single and Multi threaded Singleton.
use hexga_core::prelude::*;

use std::{cell::*,sync::*,ops::{Deref,DerefMut}};

pub use hexga_core::guard;
pub use hexga_core::cell;

mod singleton;
pub use singleton::*;

mod read_write;
pub use read_write::*;

// #[doc(hidden)]
// pub use hexga_core::guard::{ReadGuard,WriteGuard};

pub mod prelude
{
    pub use super::
    {
        SingletonSingleThread,SingletonMultiThread,SingletonRead,SingletonWrite,
        singleton_single_thread,singleton_single_thread_access,
        singleton_multi_thread,//singleton_multi_thread_access,
    };
    pub use super::guard::*;
}