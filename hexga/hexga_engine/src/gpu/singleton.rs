use super::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread::LocalKey;

thread_local! {
    pub(crate) static CONTEXT_GPU: RefCell<Option<ContextGpu>> = RefCell::new(None);
}

pub struct Gpu;

impl Singleton<ContextGpu> for Gpu
{
    fn try_as_ref() -> Option<&'static ContextGpu> {
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
    }

    fn try_as_mut() -> Option<&'static mut ContextGpu> 
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

    fn replace(instance: Option<ContextGpu>) {
        CONTEXT_GPU.replace(instance);
    }
}

impl Deref for Gpu
{
    type Target=ContextGpu;
    fn deref(&self) -> &Self::Target { self.as_ref() }
}
impl DerefMut for Gpu
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut() }
}

impl AsMut<ContextGpu> for Gpu
{
    fn as_mut(&mut self) -> &mut ContextGpu {
        Self::try_as_mut().expect("missing context gpu")
    }
}
impl AsRef<ContextGpu> for Gpu
{
    fn as_ref(&self) -> &ContextGpu {
        Self::try_as_ref().expect("missing context gpu")
    }
}