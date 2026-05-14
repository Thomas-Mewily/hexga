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

mod render;
pub use render::*;

mod result;
pub use result::*;

mod instance;
pub use instance::*;

mod gpu;
pub use gpu::*;

mod context;
pub use context::*;

use prelude::*;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::
    {
        GpuParam, GpuInstance, GpuBackend, GpuPowerPreference,

        Gpu,

        GpuContext,

        GpuResult,GpuError,

        GpuBuffer,GpuVec,
        GpuSlice,GpuSliceMut,
        GpuBufferUsage, GpuBufferUsageFlags,

        GpuSurface, GpuSurfaceConfigured,
    };
}

pub mod traits
{
    pub use super::{GpuBufferNew,GpuSliceable,GpuSliceableMut,GpuBufferElement};
}