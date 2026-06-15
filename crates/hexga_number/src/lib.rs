#![allow(unused_imports)]

use hexga_map_on as map_on;
use std::cmp::Ordering;
use std::iter::{Product, Sum};
use std::num::{Saturating, Wrapping};
use std::ops::*;

pub use map_on::*;

mod constant;
pub use constant::*;

mod increment;
pub use increment::*;

mod reflection;
pub use reflection::*;

pub mod prelude
{
    pub use super::*;
}
