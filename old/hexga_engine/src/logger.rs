use crate::*;


static INIT_LOGGER: std::sync::Once = std::sync::Once::new();

pub fn init_logger_if_needed() {
    INIT_LOGGER.call_once(|| {
        #[cfg(target_arch = "wasm32")]
        {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            env_logger::Builder::from_env(
                env_logger::Env::default().default_filter_or("error")
            ).init();
        }
    });
}
