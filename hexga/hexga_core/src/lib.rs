#![feature(get_disjoint_mut_helpers)]

use std::collections::{LinkedList, VecDeque,HashMap,HashSet,BTreeMap,BTreeSet};
use std::ffi::{OsStr, OsString};
use std::{ops::Index, slice::SliceIndex};
use core::slice::GetDisjointMutIndex;
use std::hash::{BuildHasher, Hash};
use std::borrow::Borrow;

pub mod default;
pub mod format;
pub mod iter;
pub mod utils;
pub mod collections;
use prelude::*;

pub mod prelude
{
    pub use crate::default::prelude::*;
    pub use crate::format::prelude::*;
    pub use crate::iter::prelude::*;
    pub use crate::utils::prelude::*;
    pub use crate::collections::prelude::*;
}