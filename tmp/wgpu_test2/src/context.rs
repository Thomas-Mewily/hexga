use super::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread::LocalKey;


pub struct Context
{
    pub(crate) wgpu : WgpuContext,
}

pub(crate) struct WgpuContext
{
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}


thread_local! {
    pub(crate) static CONTEXT: RefCell<Option<Rc<Context>>> = RefCell::new(None);
}

pub struct Ctx;

impl Deref for Ctx
{
    type Target=Context;
    fn deref(&self) -> &Self::Target { self.as_ref() }
}
impl DerefMut for Ctx
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut() }
}

impl AsMut<Context> for Ctx
{
    fn as_mut(&mut self) -> &mut Context {
        unsafe { self.try_as_mut().expect("missing context") }
    }
}
impl AsRef<Context> for Ctx
{
    fn as_ref(&self) -> &Context {
        unsafe { self.try_as_ref().expect("missing context") }
    }
}

impl Ctx
{
    pub unsafe fn try_as_ref(&self) -> Option<&'static Context>
    {
        CONTEXT.with(|ctx_cell| {
            // Borrow the RefCell, get the Rc<Context> if present
            if let Some(rc_ctx) = ctx_cell.borrow().as_ref() {
                // Extend the lifetime to 'static (unsafe, but valid if CONTEXT truly is static)
                let ctx_ptr: *const Context = Rc::as_ptr(rc_ctx) as *const Context;
                unsafe { Some(&*ctx_ptr) }
            } else {
                None
            }
        })
    }

    pub unsafe fn try_as_mut(&mut self) -> Option<&'static mut Context>
    {
        CONTEXT.with(|ctx_cell| {
            // Borrow the RefCell, get the Rc<Context> if present
            if let Some(rc_ctx) = ctx_cell.borrow_mut().as_mut() {
                // Extend the lifetime to 'static (unsafe, but valid if CONTEXT truly is static)
                let ctx_ptr: *mut Context = Rc::as_ptr(rc_ctx) as *mut Context;
                unsafe { Some(&mut *ctx_ptr) }
            } else {
                None
            }
        })
    }
}

