//! A minimal set of feature for every project

use std::collections::{LinkedList, VecDeque,HashMap,HashSet,BTreeMap,BTreeSet};
use std::ffi::{OsStr, OsString};
use std::{ops::Index, slice::SliceIndex};
use std::hash::{BuildHasher, Hash};
use std::borrow::Borrow;

pub use hexga_number as number;

pub use hexga_map_on::*;
pub use default_is_triple_underscore::*;
pub use number::*;

mod utils;
pub use utils::*;

mod length;
pub use length::*;

mod get_index;
pub use get_index::*;

mod capacity;
pub use capacity::*;

mod clearable;
pub use clearable::*;

mod sequence;
pub use sequence::*;

mod lookup;
pub use lookup::*;