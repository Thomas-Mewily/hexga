use super::*;
use std::iter::{FusedIterator,Map};
use std::ops::{Range, RangeInclusive};
use std::hint::unreachable_unchecked;


mod default;
mod step;
mod sample;

pub mod prelude
{
    pub use super::default::prelude::*;
    pub use super::step::prelude::*;
    pub use super::sample::prelude::*;
}