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

pub use wgpu;

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
    pub(crate) use super::experimental::*;
    pub use super::traits::*;
    pub use super::{
        Gpu, GpuBackend, GpuBufferUsage, GpuBufferUsageFlags, GpuContext, GpuError,
        GpuInstance, GpuParam, GpuPowerPreference, GpuResult
    };
    pub use super::{buffer::prelude::*, render::prelude::*};
}

pub mod traits
{
    pub use super::{buffer::prelude::*, render::prelude::*};
}
