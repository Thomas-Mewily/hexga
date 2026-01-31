use super::*;
#[cfg(feature = "std")]
use std::{
    collections::*,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use core::ops::Range;

mod length;
pub use length::*;

mod get;
pub use get::*;

mod index;
pub use index::*;

mod capacity;
pub use capacity::*;

mod clearable;
pub use clearable::*;

mod remove;
pub use remove::*;

mod push;
pub use push::*;

mod insert;
pub use insert::*;

mod pop;
pub use pop::*;

/*
mod insert_pop;
pub use insert_pop::*;
*/

mod collection;
pub use collection::*;

/*
mod sequence;
pub use sequence::*;
*/

mod collect_to;
pub use collect_to::*;

/*
mod view;
pub use view::*;

mod view_mut;
pub use view_mut::*;
*/
