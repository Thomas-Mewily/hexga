🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.
It is subject to **breaking changes** in future releases.
Use it at your own risk, and keep in mind that the API may change in future versions.

# Hexga Log

Hexga's logging facade and configurable process-wide logger. The default
threshold is `Warn` in debug builds and `Error` in release builds.

```rust
# #[cfg(feature = "std")]
# {
use hexga_log::{Config, LevelFilter, warn};

Config::default()
    .filter_level(LevelFilter::Warn)
    .filter_module("my_app", LevelFilter::Debug)
    .filter_module("wgpu_hal", LevelFilter::Error)
    .try_init()
    .expect("install logger");

warn!(target: "my_app", "running with custom logging");
# }
```

Call `try_init` before starting the event loop to supply your own configuration.
For default logging, use `init_if_needed()`. Add `.parse_env()` to a configuration
to honor `RUST_LOG` directives such as `warn,my_app=debug,wgpu_hal=off`.
Target rules match prefixes, with the longest match taking precedence.
The same API is available from `hexga_core::log`.

The default native backend writes to stdout. The `log_std` feature explicitly
selects stdout and works without `env_logger`. Enable `log_env` to opt into the
native `env_logger` backend or `log_web` to use the browser console on WebAssembly.
Cargo features are additive. If `log_std` and another backend feature are both
enabled, output goes to stdout and Cargo still includes the opted-in dependencies.
Disabling default features leaves the logging macros and facade types available
for `no_std` consumers.

## Main Hexga crate

For a quick start with several Hexga crates together, see the [`hexga` crate](https://crates.io/crates/hexga).
