#![feature(once_cell_try_insert)]
#![allow(unused)]
use hexga::bit;
use hexga::prelude::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Bound, Deref, DerefMut, RangeBounds};
use std::sync::OnceLock;
use std::sync::{Arc, Mutex};
use wgpu::util::DeviceExt;

pub mod experimental;

mod buffer;
pub use buffer::*;

use prelude::*;
pub mod prelude
{
    pub(crate) use super::experimental::prelude::*;
}

pub mod traits
{

}