#![feature(once_cell_try_insert)]

#![allow(unused)]
use std::fmt::Debug;
use std::marker::PhantomData;
use hexga::prelude::*;
use std::sync::{Arc, Mutex};
use hexga::bit;
use wgpu::util::DeviceExt;
use std::sync::OnceLock;


pub use wgpu;

mod context;
pub use context::*;

mod common_wrapper;
pub use common_wrapper::*;

mod result;
pub use result::*;

mod buffer;
pub use buffer::*;

mod gpu;
pub use gpu::*;

mod instance;
pub use instance::*;

mod surface;
pub use surface::*;

mod gpu_vec;
pub use gpu_vec::*;

pub mod prelude
{
    pub use super::{Gpu,GpuResult,GpuError};
}