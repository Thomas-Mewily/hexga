//! A 'minimal' set of feature for every project

#![feature(get_disjoint_mut_helpers)]

use std::collections::{LinkedList, VecDeque,HashMap,HashSet,BTreeMap,BTreeSet};
use std::ffi::{OsStr, OsString};
use std::{ops::Index, slice::SliceIndex};
use core::slice::GetDisjointMutIndex;
use std::hash::{BuildHasher, Hash};
use std::borrow::Borrow;

pub use hexga_number as number;
pub use hexga_map_on as map_on;

mod utils;
pub(crate) use utils::*;

pub mod prelude;

pub mod collections;
//pub(crate) use collections::*;