#![feature(unsafe_cell_access)]
//! Define Single and Multi threaded Singleton.
//!
//! Inspired by [RusPiRo Singleton crate](https://github.com/RusPiRo/ruspiro-singleton)
use hexga_core::prelude::*;
use std::{cell::*,sync::*,ops::{Deref,DerefMut}};


mod single_thread;
pub use single_thread::*;

mod singleton;
pub use singleton::*;

pub mod prelude
{
    pub use super::{SingletonSingleThread,SingletonMultiThread};
}

#[macro_export]
macro_rules! singleton_single_thread_deref_to {
    ($singleton_name: ident : $target_type: ident => $empty_struct_name: ident)=> {
        pub struct $empty_struct_name;
        impl ::std::ops::Deref for $empty_struct_name
        {
            type Target = $target_type;
            #[inline(always)]
            #[track_caller]
            fn deref(&self) -> &Self::Target
            {
                $singleton_name.instance()
            }
        }
        impl ::std::ops::DerefMut for $empty_struct_name
        {
            #[inline(always)]
            #[track_caller]
            fn deref_mut(&mut self) -> &mut Self::Target
            {
                $singleton_name.instance_mut()
            }
        }
    };
}