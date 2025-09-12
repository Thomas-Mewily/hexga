use super::*;

// Todo: move this module his in own crate
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::LocalKey;
use wgpu::util::DeviceExt;


pub(crate) type GpuVertexBufferLayout<'a> = wgpu::VertexBufferLayout<'a>;

pub type GpuDevice = wgpu::Device;
pub type GpuVecUsages = wgpu::BufferUsages;

mod typedef;
pub use typedef::*;

mod context_gpu;
pub use context_gpu::*;

mod mesh;
pub use mesh::*;

mod vec;
pub use vec::*;

mod u8_slice;
pub use u8_slice::*;

mod vertex;
pub use vertex::*;

mod format;
pub use format::*;

mod camera;
pub use camera::*;

mod pen;
pub use pen::*;


thread_local! {
    pub(crate) static CONTEXT_GPU: RefCell<Option<ContextGpu>> = RefCell::new(None);
}

singleton!(
    Gpu,
    ContextGpu,
    { 
        CONTEXT_GPU.with(|ctx_cell| {
            // Borrow the RefCell, get the Rc<ContextGpu> if present
            if let Some(rc_ctx) = ctx_cell.borrow().as_ref() {
                // Extend the lifetime to 'static (unsafe, but valid if CONTEXTGPU truly is static)
                let ctx_ptr: *const ContextGpu = rc_ctx;
                unsafe { Some(&*ctx_ptr) }
            } else {
                None
            }
        })
    },
    { 
        CONTEXT_GPU.with(|ctx_cell| {
            // Borrow the RefCell, get the Rc<ContextGpu> if present
            if let Some(rc_ctx) = ctx_cell.borrow_mut().as_mut() {
                // Extend the lifetime to 'static (unsafe, but valid if CONTEXTGPU truly is static)
                let ctx_ptr: *mut ContextGpu = rc_ctx;
                unsafe { Some(&mut *ctx_ptr) }
            } else {
                None
            }
        })
    }
);

impl SingletonInit for Gpu
{
    fn replace(instance: Option<<Self as SingletonRef>::Target>) {
        CONTEXT_GPU.replace(instance);
    }
}