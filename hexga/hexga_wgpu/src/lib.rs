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

mod gpu_context;
pub use gpu_context::*;

mod vertex;
pub use vertex::*;

mod result;
pub use result::*;

mod gpu_buffer;
pub use gpu_buffer::*;

mod gpu_instance;
pub use gpu_instance::*;

mod gpu_surface;
pub use gpu_surface::*;

mod gpu_vec;
pub use gpu_vec::*;

mod gpu_texture;
pub use gpu_texture::*;

mod gpu_sampler;
pub use gpu_sampler::*;

mod gpu_bindgroup;
pub use gpu_bindgroup::*;

pub mod prelude
{
    pub use super::{
        Gpu,GpuResult,GpuError,
        ToGpuBuffer,ToGpuVec,
        GpuBufferByte,GpuBufferNew,GpuBufferRead,GpuBufferAsWgpuSlice,GpuAsUntypedSlice
    };
}