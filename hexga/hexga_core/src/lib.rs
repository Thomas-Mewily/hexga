#![feature(get_disjoint_mut_helpers)]
#![feature(unsafe_cell_access)]
#![feature(mapped_lock_guards)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use allocation::collections::TryReserveError;

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

// re-export without rename
macro_rules! re_export_item_from_std {
    ($($segments:ident)::+) => {
        #[cfg(feature = "std")]
        pub use std::$($segments)::+;

        #[cfg(not(feature = "std"))]
        pub use ::alloc::$($segments)::+;
    };
}

/*
// re-export with rename
macro_rules! re_export_item_from_std_as {
    ($($segments:ident)::+ as $alias:ident) => {
        #[cfg(feature = "std")]
        pub use std::$($segments)::+ as $alias;

        #[cfg(not(feature = "std"))]
        pub use ::alloc::$($segments)::+ as $alias;
    };
}
*/

macro_rules! re_export_items_from_std {
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

macro_rules! re_export_mod_from_std {
    // with rename
    ($name:ident as $alias:ident $(, $rest:tt)*) => {
        pub mod $alias {
            re_export_items_from_std!($name);
        }
        re_export_mod_from_std!($($rest),*);
    };

    // without rename
    ($name:ident $(, $rest:tt)*) => {
        pub mod $name {
            re_export_items_from_std!($name);
        }
        re_export_mod_from_std!($($rest),*);
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

re_export_mod_from_std!(
    any, array, ascii, borrow, clone, cmp, error, fmt, future, hash, hint, io, mem, net, num,
    panic, path, pin, task, // time <- Time don't work on WASM
    str, string, slice // random
);

#[cfg(feature = "std")]
re_export_mod_from_std!(
    thread, fs, vec
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

    re_export_items_from_std!(prelude);
}
