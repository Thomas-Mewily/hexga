use super::*;
use std::hint::unreachable_unchecked;
use std::iter::{FusedIterator, Map};
use std::ops::{Range, RangeInclusive};

mod default;
pub use default::*;

mod step;
pub use step::*;

mod sample;
pub use sample::*;

/*
pub mod prelude
{
    pub use super::traits::*;
}
pub mod traits
{
    pub use super::{RangeDefault,RangeStepExtension, RangeStepIter, RangeDefaultStepExtension, RangeDefaultStepInclusiveExtension};
}
*/