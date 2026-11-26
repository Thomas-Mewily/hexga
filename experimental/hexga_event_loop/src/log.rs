use super::*;

pub use ::log::{debug, error, info, warn};

pub mod prelude {
    pub use super::{debug, error, info, warn};
}

static LOGGER_INIT: LazyLock<()> = LazyLock::new(|| {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use ::log::LevelFilter;
        use std::io::Write;

        const WGPU_LOG_LEVEL: LevelFilter = LevelFilter::Error;

        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
            .filter_module("wgpu_core", WGPU_LOG_LEVEL)
            .filter_module("wgpu_hal", WGPU_LOG_LEVEL)
            .filter_module("wgpu_hal::dx12", WGPU_LOG_LEVEL)
            .filter_module("wgpu_hal::gles", WGPU_LOG_LEVEL)
            .filter_module("wgpu_hal::metal", WGPU_LOG_LEVEL)
            .filter_module("wgpu_hal::noop", WGPU_LOG_LEVEL)
            .filter_module("wgpu_hal::vulkan", WGPU_LOG_LEVEL)
            .filter_module("naga", WGPU_LOG_LEVEL)
            .format(|buf, record| writeln!(buf, "{}", record.args()))
            .init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(::log::Level::Debug).expect("Couldn't initialize logger");
    }
});

pub fn init_if_needed() {
    LazyLock::force(&LOGGER_INIT);
}
