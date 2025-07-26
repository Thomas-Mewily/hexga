use super::*;


static mut CONTEXT : Option<Context> = None;
#[allow(static_mut_refs)]
pub(crate) fn ctx_mut() -> &'static mut Context { unsafe { CONTEXT.as_mut().expect("Ctx not initialized") } }
#[allow(static_mut_refs)]
pub(crate) fn ctx() -> &'static Context { unsafe { CONTEXT.as_ref().expect("Ctx not initialized") } }
#[allow(static_mut_refs)]
pub(crate) fn init_ctx(mut ctx : Option<Context>) -> Option<Context>
{
    std::mem::swap(&mut ctx, unsafe { &mut CONTEXT });
    ctx
}

static INIT_LOGGER: std::sync::Once = std::sync::Once::new();

pub fn init_logger() {
    INIT_LOGGER.call_once(|| {
        #[cfg(target_arch = "wasm32")]
        {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Error).expect("Couldn't initialize logger");
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            env_logger::Builder::from_env(
                env_logger::Env::default().default_filter_or("error")
            ).init();
        }
    });
}
