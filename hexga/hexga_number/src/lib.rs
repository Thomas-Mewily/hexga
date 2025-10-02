#![allow(unused_imports)]

use hexga_map_on as map_on;
use std::cmp::Ordering;
use std::ops::*;
use std::iter::{Sum,Product};
use std::num::{Saturating, Wrapping};

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