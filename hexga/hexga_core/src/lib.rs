#![feature(get_disjoint_mut_helpers)]

use std::collections::{LinkedList, VecDeque,HashMap,HashSet,BTreeMap,BTreeSet};
use std::{ops::Index, slice::SliceIndex};
use core::slice::GetDisjointMutIndex;
use std::hash::{BuildHasher, Hash};
use std::borrow::Borrow;

pub mod default;
pub mod format;
pub mod iter;
pub mod utils;
pub mod collections;
pub mod accessor;

use prelude::*;
pub mod prelude
{
    pub use super::default::*;
    pub use super::format::*;
    pub use super::iter::*;
    pub use super::utils::*;
    pub use super::collections::*;
    pub use super::accessor::*;
}