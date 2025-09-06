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

pub mod prelude
{
    pub use super::lerp::prelude::*;
    pub use super::time::prelude::*;
    pub use super::prefix::prelude::*;
    pub use super::map_on::prelude::*;
    pub use super::unit::prelude::*;
    pub use super::array::prelude::*;
}