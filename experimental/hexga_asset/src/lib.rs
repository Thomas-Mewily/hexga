#![allow(unused_imports)]
use hexga_utils::dirty::prelude::*;
use hexga_save::prelude::*;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug,Display,Formatter,Result as FmtResult};

mod reload;
pub use reload::*;

mod file;
pub use file::*;


pub mod prelude
{
    pub use super::FileData;
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::Reload;
}