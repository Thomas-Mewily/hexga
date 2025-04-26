//! A minimal set of feature for every project

use std::{collections::HashMap, ops::Index, slice::SliceIndex};
use std::hash::{BuildHasher, Hash};

pub use hexga_map_on::*;
pub use default_is_triple_underscore::*;

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
