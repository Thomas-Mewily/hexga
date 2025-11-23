#![feature(get_disjoint_mut_helpers)]
#![feature(unsafe_cell_access)]

use std::collections::{LinkedList, VecDeque,HashMap,HashSet,BTreeMap,BTreeSet};
use std::{ops::Index, slice::SliceIndex};
use core::slice::GetDisjointMutIndex;
use std::hash::{BuildHasher, Hash};
use std::borrow::Borrow;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};


pub mod default;
pub mod format;
pub mod iter;
pub mod utils;
pub mod collections;
//pub mod accessor;
pub mod builder;
pub mod traits;
pub mod asynchrone;
pub mod guard;
pub mod wrapper;
pub mod cfg;
pub mod rc;
pub mod sync;
pub mod cell;

use prelude::*;
pub mod prelude
{
    pub use super::default::*;
    pub use super::format::*;
    pub use super::iter::*;
    pub use super::utils::*;
    pub use super::collections::*;
    //pub use super::accessor::*;
    pub use super::builder::*;
    pub use super::traits::prelude::*;
    pub use super::asynchrone::*;
    pub use super::guard::*;
    pub use super::wrapper::*;
    pub use std::collections::{HashMap,HashSet};
}
