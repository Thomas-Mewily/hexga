use std::ops::{Index, IndexMut};

mod array;
pub use array::*;

mod extension;
pub use extension::*;

pub mod prelude
{
    pub use super::*;
}