use super::*;


static mut CONTEXT : Option<Context> = None;
#[allow(static_mut_refs)]
pub(crate) fn ctx_mut() -> &'static mut Context { unsafe { CONTEXT.as_mut().expect("Ctx not initialized") } }
#[allow(static_mut_refs)]
pub(crate) fn ctx() -> &'static Context { unsafe { CONTEXT.as_ref().expect("Ctx not initialized") } }

#[allow(static_mut_refs)]
pub(crate) fn ctx_mut_or_init() -> &'static mut Context { init_ctx_if_needed(); ctx_mut() }
#[allow(static_mut_refs)]
pub(crate) fn ctx_or_init() -> &'static Context { init_ctx_if_needed(); ctx() }

#[allow(static_mut_refs)]
pub(crate) fn init_ctx_if_needed()
{
    unsafe
    {
        if CONTEXT.is_none()
        {
            CONTEXT = Some(Context::new());
        }
        std::panic::set_hook(Box::new(|info| {
            CONTEXT = None;
            eprintln!("Panic occurred: {info}");
        }));
    }
}

pub(crate) fn reset_ctx()
{
    unsafe { CONTEXT = None };
}

static INIT_LOGGER: std::sync::Once = std::sync::Once::new();

pub fn init_logger_if_needed() {
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
