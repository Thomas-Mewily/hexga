use super::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread::LocalKey;

thread_local! {
    pub(crate) static CONTEXT: RefCell<Option<Context>> = RefCell::new(None);
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
        Self::try_as_mut().expect("missing context")
    }
}
impl AsRef<Context> for Ctx
{
    fn as_ref(&self) -> &Context {
        Self::try_as_ref().expect("missing context")
    }
}

impl SingletonRef for Ctx
{
    type Target = Context;

    fn try_as_ref() -> Option<&'static <Self as SingletonRef>::Target> {
        CONTEXT.with(|ctx_cell| {
            // Borrow the RefCell, get the Rc<Context> if present
            if let Some(rc_ctx) = ctx_cell.borrow().as_ref() {
                // Extend the lifetime to 'static (unsafe, but valid if CONTEXT truly is static)
                let ctx_ptr: *const Context = rc_ctx;
                unsafe { Some(&*ctx_ptr) }
            } else {
                None
            }
        })
    }
}

impl SingletonMut for Ctx
{
    fn try_as_mut() -> Option<&'static mut Context> {
        CONTEXT.with(|ctx_cell| {
            // Borrow the RefCell, get the Rc<Context> if present
            if let Some(rc_ctx) = ctx_cell.borrow_mut().as_mut() {
                // Extend the lifetime to 'static (unsafe, but valid if CONTEXT truly is static)
                let ctx_ptr: *mut Context = rc_ctx;
                unsafe { Some(&mut *ctx_ptr) }
            } else {
                None
            }
        })
    }
}
impl SingletonInit for Ctx
{
    fn replace(instance: Option<<Self as SingletonRef>::Target>) {
        match instance
        {
            Some(ctx) => 
            {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    env_logger::init();
                    std::panic::set_hook(Box::new(|info| {
                        Ctx::destroy();
                        eprintln!("panic occurred: {info}");
                    }));
                }
                #[cfg(target_arch = "wasm32")]
                {
                    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                    console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
                }
                CONTEXT.replace(Some(ctx));
                // The Gpu is initialized in a special async way... 
            },
            None => 
            {
                CONTEXT.replace(None);
                Gpu::destroy();
            },
        }
    }
}