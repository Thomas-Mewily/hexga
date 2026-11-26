pub use hexga_core::log::*;

/// Installs the event loop's default target filters unless logging is already configured.
pub fn init_if_needed()
{
    init_if_needed_with(|| {
        Config::default()
            .filter_module("wgpu_core", LevelFilter::Error)
            .filter_module("wgpu_hal", LevelFilter::Error)
            .filter_module("naga", LevelFilter::Error)
            .parse_env()
    });
}
