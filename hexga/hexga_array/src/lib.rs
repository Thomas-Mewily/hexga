use std::ops::{Index, IndexMut};
use std::cmp::Ordering;

mod array;
pub use array::*;

mod extension;
pub use extension::*;

pub mod prelude
{
    pub use super::array::prelude::*;
    pub use super::extension::prelude::*;
}