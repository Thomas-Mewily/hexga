#![allow(unused_imports)]
use hexga_utils::dirty::prelude::*;
use hexga_io::prelude::*;
use hexga_encoding::prelude::*;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use hexga_core::prelude::*;

mod reload;
pub use reload::*;

/*
mod file;
pub use file::*;

mod asset_manager;
pub use asset_manager::*;

mod asset;
pub use asset::*;
*/

pub mod prelude
{
    //pub use super::FileData;
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::Reload;
}