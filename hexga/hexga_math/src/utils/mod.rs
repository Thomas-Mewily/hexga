//! Stuff inside this module need to move somewhere else...
use super::*;

pub mod array;
pub mod map_on;
pub mod prefix;
pub mod time;
pub mod unit;
pub mod lerp;
mod composite;

pub mod prelude
{
    pub use super::array::prelude::*;
    pub use super::lerp::prelude::*;
    pub use super::map_on::prelude::*;
    pub use super::prefix::prelude::*;
    pub use super::time::prelude::*;
    pub use super::unit::prelude::*;
    pub use super::composite::prelude::*;
}