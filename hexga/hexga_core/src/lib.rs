#![feature(get_disjoint_mut_helpers)]
#![feature(unsafe_cell_access)]
#![feature(mapped_lock_guards)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::collections::TryReserveError;

use core::hash::BuildHasher;
use core::slice::GetDisjointMutIndex;
use core::{borrow::Borrow, hash::Hash, ops::Index, slice::SliceIndex};

#[cfg(feature = "std")]
use std::{
    collections::*,
    //hash::RandomState,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor, ser::SerializeStruct};

macro_rules! re_export_item_from_std_or_alloc {
    ($($segments:ident)::+) => {
        #[cfg(feature = "std")]
        pub use std::$($segments)::+;

        #[cfg(not(feature = "std"))]
        pub use ::alloc::$($segments)::+;
    };
}

macro_rules! re_export_items_from_std_or_alloc {
    ($($name:ident),+ $(,)?) => {
        $(
            #[allow(ambiguous_glob_reexports)]
            #[cfg(feature = "std")]
            pub use std::$name::*;
            #[allow(ambiguous_glob_reexports)]
            #[cfg(not(feature = "std"))]
            pub use ::core::$name::*;
        )+
    };
}

macro_rules! re_export_mod_from_std_or_alloc {
    /*
    // with rename
    ($name:ident as $alias:ident $(, $rest:tt)*) => {
        pub mod $alias {
            re_export_items_from_std_or_alloc!($name);
        }
        re_export_mod_from_std_or_alloc!($($rest),*);
    };
    */

    // without rename
    ($name:ident $(, $rest:tt)*) => {
        pub mod $name {
            re_export_items_from_std_or_alloc!($name);
        }
        re_export_mod_from_std_or_alloc!($($rest),*);
    };

    () => {};
}


macro_rules! re_export_item_from_std_or_core {
    ($($segments:ident)::+) => {
        #[cfg(feature = "std")]
        pub use std::$($segments)::+;

        #[cfg(not(feature = "std"))]
        pub use ::core::$($segments)::+;
    };
}

#[allow(unused)]
macro_rules! re_export_items_from_std_or_core {
    ($($name:ident),+ $(,)?) => {
        $(
            #[allow(ambiguous_glob_reexports)]
            #[cfg(feature = "std")]
            pub use std::$name::*;
            #[allow(ambiguous_glob_reexports)]
            #[cfg(not(feature = "std"))]
            pub use ::core::$name::*;
        )+
    };
}

#[allow(unused)]
macro_rules! re_export_mod_from_std_or_core {
    /*
    // with rename
    ($name:ident as $alias:ident $(, $rest:tt)*) => {
        pub mod $alias {
            re_export_items_from_std_or_core!($name);
        }
        re_export_mod_from_std_or_core!($($rest),*);
    };
    */

    // without rename
    ($name:ident $(, $rest:tt)*) => {
        pub mod $name {
            re_export_items_from_std_or_core!($name);
        }
        re_export_mod_from_std_or_core!($($rest),*);
    };

    () => {};
}

pub mod accessor;
pub mod allocation;
pub mod asynchrone;
pub mod boxed;
pub mod builder;
#[cfg(feature = "std")]
pub mod cell;
pub mod cfg;
pub mod ops;
pub mod collections;
pub mod default;
pub mod format;
pub mod guard;
pub mod handle;
pub mod iter;
pub mod option;
pub mod rc;
pub mod sync;
#[macro_use]
pub mod macros;
pub mod convert;
pub mod marker;
pub mod primitives;
pub mod ptr;
pub mod result;
pub mod run;
#[cfg(feature = "std")]
pub mod singleton;
pub mod utils;
pub mod wrapper;
pub use hexga_bit as bit;
pub use hexga_map_on as map_on;

re_export_mod_from_std_or_alloc!(
    any, array, ascii, clone, cmp, error, fmt, future, hash, hint, io, mem, net, num,
    panic, pin, task, // time <- Time don't work on WASM
    str, slice, // random
    borrow
);

#[cfg(feature = "std")]
re_export_mod_from_std_or_alloc!(
    thread, fs, vec, string, path
);

use prelude::*;
pub mod prelude
{
    pub use super::{hexga_prelude::*, std_prelude::*};
}

#[doc(hidden)]
/// Hexga specific prelude without the std.
///
/// You are probably looking for the `prelude` module.
pub mod hexga_prelude
{
    #[rustfmt::skip]
    #[allow(unused_imports)]
    pub use super::{
        allocation::prelude::*,
        accessor::*,
        asynchrone::*,
        boxed::prelude::*,
        builder::*,
        ops::prelude::*,
        collections::prelude::*,
        default::prelude::*,
        format::*,
        guard::*,
        handle::*,
        option::prelude::*,
        iter::prelude::*,
        macros::prelude::*,
        marker::prelude::*,
        primitives::prelude::*,
        run::*,
        utils::*,
        wrapper::*,
        result::prelude,
        convert::*,
        bit::prelude::*,
        map_on::prelude::*,
        ops::*,
    };

    pub(crate) use super::{primitives::*, ptr::*};

    #[cfg(feature = "std")]
    pub use super::singleton::prelude::*;
}

#[doc(hidden)]
/// Std specific prelude.
///
/// You are probably looking for the `prelude` module.
pub mod std_prelude
{
    // todo: use derive_more for Deref, DerefMut ? https://crates.io/crates/derive_more
    pub use core::{
        convert::{AsMut, AsRef},
        ops::{Deref, DerefMut},
    };

    re_export_items_from_std_or_alloc!(prelude);
}
