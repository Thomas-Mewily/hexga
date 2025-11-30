#![feature(get_disjoint_mut_helpers)]
#![feature(unsafe_cell_access)]
#![feature(mapped_lock_guards)]

use std::{ops::Index, slice::SliceIndex};
use core::slice::GetDisjointMutIndex;
use std::hash::{BuildHasher, Hash};
use std::borrow::Borrow;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};


pub mod accessor;
pub mod asynchrone;
pub mod builder;
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
pub mod traits;
pub mod utils;
pub mod wrapper;
pub mod mem;

use prelude::*;
pub mod prelude
{
    pub use std::collections::{HashMap,HashSet,BTreeMap,BTreeSet,VecDeque};
    pub use super::accessor::*;
    pub use super::asynchrone::*;
    pub use super::builder::*;
    pub use super::collections::*;
    pub use super::default::*;
    pub use super::format::*;
    pub use super::guard::*;
    pub use super::handle::*;
    pub use super::iter::*;
    pub use super::traits::prelude::*;
    pub use super::utils::*;
    pub use super::wrapper::*;
    pub use super::mem::*;
}
