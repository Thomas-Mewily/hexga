#![feature(get_disjoint_mut_helpers)]
#![feature(unsafe_cell_access)]
#![feature(mapped_lock_guards)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::collections::TryReserveError;

use core::{borrow::Borrow, hash::Hash, ops::Index, slice::SliceIndex};
use core::slice::GetDisjointMutIndex;

#[cfg(feature = "std")]
use std::hash::BuildHasher;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};


pub mod accessor;
pub mod asynchrone;
pub mod builder;
#[cfg(feature = "std")]
pub mod cell;
pub mod cfg;
pub mod collections;
pub mod default;
pub mod format;
pub mod guard;
pub mod handle;
pub mod iter;
pub mod rc;
pub mod sync;
pub mod macros;
pub mod utils;
pub mod run;
pub mod wrapper;
pub mod result;
pub mod convert;
#[cfg(feature = "std")]
pub mod singleton;
pub use hexga_bit as bit;
pub use hexga_map_on as map_on;

use prelude::*;
pub mod prelude
{
    pub use super::{hexga_prelude::*,std_prelude::*};
}

#[doc(hidden)]
/// Hexga specific prelude without the std.
///
/// You are probably looking for the `prelude` module.
pub mod hexga_prelude
{
    #[cfg(feature = "std")]
    pub use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, BinaryHeap};
    #[cfg(feature = "std")]
    pub(crate) use std::collections::LinkedList;

    #[cfg(not(feature = "std"))]
    pub use alloc::collections::{BTreeMap, BTreeSet, VecDeque, BinaryHeap, LinkedList};
    #[cfg(not(feature = "std"))]
    pub use alloc::{boxed::Box, string::String, vec::Vec};

    #[rustfmt::skip]
    #[allow(unused_imports)]
    pub use super::{
        accessor::*,
        asynchrone::*,
        builder::*,
        collections::*,
        default::*,
        format::*,
        guard::*,
        handle::*,
        iter::*,
        macros::prelude::*,
        run::*,
        utils::*,
        wrapper::*,
        result::*,
        convert::*,
        bit::prelude::*,
        map_on::prelude::*,
    };

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
    pub use core::{convert::{AsRef,AsMut}, ops::{Deref,DerefMut}};
}