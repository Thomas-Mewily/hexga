use crate::{DEFAULT_LEVEL, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::sync::OnceLock;

/// Startup configuration for the process-wide logger.
#[derive(Clone, Debug)]
pub struct Config
{
    level: LevelFilter,
    targets: Vec<(String, LevelFilter)>,
}

impl Default for Config
{
    fn default() -> Self
    {
        Self {
            level: DEFAULT_LEVEL,
            targets: Vec::new(),
        }
    }
}

impl Config
{
    /// Sets the threshold for targets with no matching rule.
    pub fn filter_level(mut self, level: LevelFilter) -> Self
    {
        self.level = level;
        self
    }

    /// Sets the threshold for a target prefix. The longest matching prefix wins.
    pub fn filter_module(mut self, target: impl Into<String>, level: LevelFilter) -> Self
    {
        let target = target.into();
        if let Some((_, existing)) = self.targets.iter_mut().find(|(prefix, _)| prefix == &target)
        {
            *existing = level;
        }
        else
        {
            self.targets.push((target, level));
        }
        self
    }

    /// Applies comma-separated levels and target filters from `RUST_LOG`.
    pub fn parse_env(self) -> Self
    {
        match std::env::var("RUST_LOG")
        {
            Ok(filters) => self.parse_filters(&filters),
            Err(_) => self,
        }
    }

    /// Applies directives such as `warn,my_app=debug,wgpu_hal=off`.
    /// A bare target enables all levels for that target. Invalid directives are ignored.
    pub fn parse_filters(mut self, filters: &str) -> Self
    {
        for directive in filters.split(',').map(str::trim).filter(|directive| !directive.is_empty())
        {
            if let Some((target, level)) = directive.split_once('=')
            {
                let target = target.trim();
                if !target.is_empty()
                    && let Ok(level) = level.trim().parse()
                {
                    self = self.filter_module(target, level);
                }
            }
            else if let Ok(level) = directive.parse()
            {
                self = self.filter_level(level);
            }
            else if !directive.contains('/') && !directive.contains(char::is_whitespace)
            {
                self = self.filter_module(directive, LevelFilter::Trace);
            }
        }
        self
    }

    /// Installs this configuration. Returns an error if a logger already exists.
    pub fn try_init(self) -> Result<(), SetLoggerError>
    {
        let max_level = self.targets.iter().map(|(_, level)| *level).fold(self.level, core::cmp::max);
        log::set_boxed_logger(Box::new(Logger::new(self)))?;
        log::set_max_level(max_level);
        Ok(())
    }

    fn level_for(&self, target: &str) -> LevelFilter
    {
        self.targets
            .iter()
            .filter(|(prefix, _)| target.starts_with(prefix))
            .max_by_key(|(prefix, _)| prefix.len())
            .map_or(self.level, |(_, level)| *level)
    }
}

/// Installs the default configuration with any `RUST_LOG` overrides.
pub fn init_if_needed() { init_if_needed_with(|| Config::default().parse_env()); }

/// Installs a fallback configuration once. An already installed logger takes precedence.
pub fn init_if_needed_with(configure: impl FnOnce() -> Config)
{
    static INIT: OnceLock<()> = OnceLock::new();
    INIT.get_or_init(|| {
        let _ = configure().try_init();
    });
}

struct Logger
{
    config: Config,
    output: output::Output,
}

impl Logger
{
    fn new(config: Config) -> Self
    {
        Self {
            config,
            output: output::Output::new(),
        }
    }
}

impl Log for Logger
{
    fn enabled(&self, metadata: &Metadata<'_>) -> bool { metadata.level() <= self.config.level_for(metadata.target()) }

    fn log(&self, record: &Record<'_>)
    {
        if self.enabled(record.metadata())
        {
            self.output.write(record);
        }
    }

    fn flush(&self) { self.output.flush(); }
}

#[cfg(all(feature = "log_env", not(feature = "log_std"), not(target_arch = "wasm32")))]
mod output
{
    use super::{LevelFilter, Log, Record};
    use std::io::Write;

    pub(super) struct Output(env_logger::Logger);

    impl Output
    {
        pub(super) fn new() -> Self
        {
            Self(
                env_logger::Builder::new()
                    .filter_level(LevelFilter::Trace)
                    .format(|buf, record| writeln!(buf, "{}", record.args()))
                    .build(),
            )
        }

        pub(super) fn write(&self, record: &Record<'_>) { self.0.log(record); }

        pub(super) fn flush(&self) { self.0.flush(); }
    }
}

#[cfg(all(feature = "log_web", not(feature = "log_std"), target_arch = "wasm32"))]
mod output
{
    use super::Record;
    use crate::Level;
    use web_sys::console;

    pub(super) struct Output;

    impl Output
    {
        pub(super) fn new() -> Self { Self }

        pub(super) fn write(&self, record: &Record<'_>)
        {
            let message = record.args().to_string().into();
            match record.level()
            {
                Level::Error => console::error_1(&message),
                Level::Warn => console::warn_1(&message),
                Level::Info => console::info_1(&message),
                Level::Debug | Level::Trace => console::debug_1(&message),
            }
        }

        pub(super) fn flush(&self) {}
    }
}

#[cfg(not(any(
    all(feature = "log_env", not(feature = "log_std"), not(target_arch = "wasm32")),
    all(feature = "log_web", not(feature = "log_std"), target_arch = "wasm32"),
)))]
mod output
{
    use super::Record;
    use std::io::{Write, stdout};

    pub(super) struct Output;

    impl Output
    {
        pub(super) fn new() -> Self { Self }

        pub(super) fn write(&self, record: &Record<'_>) { let _ = writeln!(stdout().lock(), "{}", record.args()); }

        pub(super) fn flush(&self) { let _ = stdout().lock().flush(); }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn defaults_to_warn_in_debug_and_error_in_release()
    {
        let expected = if cfg!(debug_assertions)
        {
            LevelFilter::Warn
        }
        else
        {
            LevelFilter::Error
        };
        assert_eq!(Config::default().level_for("app"), expected);
    }

    #[test]
    fn more_specific_filters_override_defaults_and_can_be_replaced()
    {
        let config = Config::default()
            .filter_module("app", LevelFilter::Info)
            .parse_filters("error, app=debug, app::noisy=off, app::worker, app=invalid");

        assert_eq!(config.level_for("other"), LevelFilter::Error);
        assert_eq!(config.level_for("app"), LevelFilter::Debug);
        assert_eq!(config.level_for("app::noisy::task"), LevelFilter::Off);
        assert_eq!(config.level_for("app::worker"), LevelFilter::Trace);
    }

    #[test]
    fn fallback_keeps_an_explicitly_installed_logger()
    {
        Config::default()
            .filter_level(LevelFilter::Error)
            .filter_module("app", LevelFilter::Trace)
            .try_init()
            .unwrap();
        init_if_needed_with(|| Config::default().filter_level(LevelFilter::Off));

        let record = Metadata::builder().target("app::worker").level(crate::Level::Trace).build();
        assert!(log::logger().enabled(&record));
        assert_eq!(log::max_level(), LevelFilter::Trace);
    }
}
