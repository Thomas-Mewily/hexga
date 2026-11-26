//! Run with `cargo run -p hexga_log --example scoped_logging`.

use hexga_log::{LogConfig, Log, LogLevel, LoggableExtension, LoggableLazyExtension, ScopedWith, SetEnableExtension, info};

fn main() -> Result<(), hexga_log::InitError>
{
    LogConfig::default().filter_level(LogLevel::Info).try_init()?;

    info!("Ready");
    Log.info("some info");
    Log.disable();
    Log.lazy_info(|| "info but lazy");
    Log.enable();

    Log.scope("request 42", |_| {
        info!("Accepted");
        Log.scope("database", |_| {
            info!("Saved");
        });
        info!("Completed");
    });
    info!("Idle");

    Ok(())
}
