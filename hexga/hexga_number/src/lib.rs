#![allow(unused_imports)]

pub(crate) use std::cmp::Ordering;
pub(crate) use std::ops::*;
pub(crate) use std::iter::{Sum,Product};
pub(crate) use std::num::{Saturating, Wrapping};


mod map_on;
pub use map_on::*;

mod constant;
pub use constant::*;

mod increase;
pub use increase::*;

mod number;
pub use number::*;