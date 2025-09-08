use super::*;
use std::iter::{FusedIterator,Map};
use std::ops::{Range, RangeInclusive};
use std::hint::unreachable_unchecked;


mod default;
pub use default::*;

mod step;
pub use step::*;

mod sample;
pub use sample::*;

pub mod prelude
{
    pub use super::default::prelude::*;
    pub use super::step::prelude::*;
    pub use super::sample::prelude::*;
}