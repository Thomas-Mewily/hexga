use super::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread::LocalKey;

pub mod prelude
{
    pub use super::Ctx;
}

thread_local! {
    pub(crate) static CONTEXT: RefCell<Option<Context>> = RefCell::new(None);
}

ctx_singleton!(
    Ctx,
    Context,
    { 
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
    },
    { 
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
);


impl SingletonInit for Ctx
{
    fn replace(instance: Option<<Self as SingletonRef>::Target>) {
        match instance
        {
            Some(ctx) => 
            {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    use std::io::Write;

                    env_logger::Builder::from_env(
                        env_logger::Env::default().default_filter_or("debug")
                    )
                    .filter_module("wgpu_core", ::log::LevelFilter::Warn)
                    .filter_module("wgpu_hal", ::log::LevelFilter::Warn)
                    .filter_module("naga", ::log::LevelFilter::Warn)
                    .format(|buf, record| {
                        writeln!(buf, "{}", record.args())
                    })
                    .init();

                    std::panic::set_hook(Box::new(|info| {
                        Ctx::destroy();
                        eprintln!("panic occurred: {info}");
                    }));
                }
                #[cfg(target_arch = "wasm32")]
                {
                    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                    console_log::init_with_level(::log::Level::Debug).expect("Couldn't initialize logger");
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