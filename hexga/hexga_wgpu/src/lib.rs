#![feature(once_cell_try_insert)]

#![allow(unused)]
use std::fmt::Debug;
use std::marker::PhantomData;
use hexga::prelude::*;
use std::sync::{Arc, Mutex};
use hexga::bit;
use wgpu::util::DeviceExt;
use std::sync::OnceLock;
use std::ops::{Deref,DerefMut,Bound, RangeBounds};


pub use wgpu;

mod gpu;
pub use gpu::*;

mod vertex;
pub use vertex::*;

mod result;
pub use result::*;

mod buffer;
pub use buffer::*;

mod instance;
pub use instance::*;

mod surface;
pub use surface::*;

mod gpu_vec;
pub use gpu_vec::*;

pub mod prelude
{
    pub use super::{
        Gpu,GpuResult,GpuError,
        ToGpuBuffer,ToGpuVec,
        GpuBufferByte,GpuBufferNew,GpuBufferRead,GpuBufferAsWgpuSlice,GpuAsUntypedSlice
    };
}