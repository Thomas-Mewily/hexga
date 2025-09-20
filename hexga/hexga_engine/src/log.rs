pub use log::{info, warn, error, debug};


pub(crate) fn init_logger()
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
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(::log::Level::Debug).expect("Couldn't initialize logger");
    }
}
