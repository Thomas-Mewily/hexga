🚧 **Warning: Experimental Crate!** 🚧

# Hexga Log

Hexga Log provides configurable logging with target filters and scoped prefixes.

The logger initializes on first use with `Debug` in debug builds and `Error` in release builds.

```rust
use hexga_log::{debug, error, warn};

debug!("Loading settings");
warn!("Settings file missing, using defaults");
error!("Could not open the save file");
```

- `log_env` (default) uses `env_logger` on native targets and the browser console on browser WebAssembly.
- `std` writes to stdout when used with `default-features = false`.

## Example

To change the default level, configure the logger before logging.

```rust
use hexga_log::{Config, LogLevel, info};

Config::default().filter_level(LogLevel::Info).try_init().unwrap();
info!("Application started");
```

## Custom logger

Implement `LoggableDyn`, `Enable`, `SetEnable`, and `Logger` for your logger type.
Put your output logic in `log_str`, which accepts a `LogFragment<'_>`, then install a `Send + 'static` instance
with `hexga_log::set_logger(my_logger)`.

## Main Hexga crate

For a quick start with several Hexga crates together, see the [`hexga` crate](https://crates.io/crates/hexga).
