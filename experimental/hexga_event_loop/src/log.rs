pub use hexga_core::log::*;

/// Installs the event loop's default target filters unless logging is already configured.
pub fn init_if_needed()
{
    init_if_needed_with(|| {
        LogConfig::default()
            .filter_module("wgpu_core", LogLevel::Error)
            .filter_module("wgpu_hal", LogLevel::Error)
            .filter_module("naga", LogLevel::Error)
            .parse_env()
    });
}
