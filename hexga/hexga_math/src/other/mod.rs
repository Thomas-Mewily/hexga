//! Stuff inside this module need to move somewhere else...
use super::*;

mod lerp;
pub use lerp::*;

pub mod time;
pub(crate) use time::*;

pub mod prefix;
pub(crate) use prefix::*;

pub mod map_on;
pub(crate) use map_on::*;

mod unit;
pub use unit::*;

pub mod array;
use array::*;