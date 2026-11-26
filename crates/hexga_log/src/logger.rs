use crate::{DEFAULT_LEVEL, Enable, Level, LogFragment, LogLevel, LoggableDyn, Logger, Metadata, Record, SetEnable};
use std::{
    collections::HashMap,
    fmt,
    sync::{LazyLock, Mutex, MutexGuard, OnceLock, PoisonError},
};

/// Startup settings for the built-in logger.
#[derive(Clone, Debug)]
pub struct LogConfig
{
    enabled: bool,
    level: LogLevel,
    targets: HashMap<String, LogLevel>,
}

impl Default for LogConfig
{
    fn default() -> Self
    {
        Self {
            enabled: true,
            level: DEFAULT_LEVEL,
            targets: HashMap::new(),
        }
    }
}

impl LogConfig
{
    /// Overrides the build's default threshold.
    pub fn filter_level(mut self, level: impl Into<LogLevel>) -> Self
    {
        self.level = level.into();
        self
    }

    /// Sets a target prefix's threshold. The longest matching prefix wins.
    /// `LogLevel::None` disables the matching target.
    pub fn filter_module(mut self, target: impl Into<String>, level: LogLevel) -> Self
    {
        self.targets.insert(target.into(), level);
        self
    }

    pub fn parse_env(self) -> Self
    {
        match std::env::var("RUST_LOG")
        {
            Ok(filters) => self.parse_filters(&filters),
            Err(_) => self,
        }
    }

    /// Applies comma separated level and target rules from a `RUST_LOG` string.
    pub fn parse_filters(mut self, filters: &str) -> Self
    {
        for directive in filters.split(',').map(str::trim).filter(|directive| !directive.is_empty())
        {
            if let Some((target, level)) = directive.split_once('=')
            {
                let target = target.trim();
                let level = level.trim();
                if !target.is_empty()
                {
                    if level.eq_ignore_ascii_case("off")
                    {
                        self = self.filter_module(target, LogLevel::None);
                    }
                    else if let Ok(level) = level.parse::<Level>()
                    {
                        self = self.filter_module(target, level.into());
                    }
                }
            }
            else if directive.eq_ignore_ascii_case("off")
            {
                self.enabled = false;
            }
            else if let Ok(level) = directive.parse::<Level>()
            {
                self.enabled = true;
                self.level = level.into();
            }
            else if !directive.contains('/') && !directive.contains(char::is_whitespace)
            {
                self = self.filter_module(directive, LogLevel::Trace);
            }
        }
        self
    }

    /// Installs these settings if the Hexga logger has yet to initialize.
    pub fn try_init(self) -> Result<(), InitError>
    {
        ACTIVE.set(Mutex::new(Active::configured(self))).map_err(|_| InitError)?;
        register_facade();
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitError;

impl fmt::Display for InitError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("the Hexga logger is already initialized") }
}

impl std::error::Error for InitError {}

static ACTIVE: OnceLock<Mutex<Active>> = OnceLock::new();
static FALLBACK: OnceLock<LogConfig> = OnceLock::new();

struct Active
{
    logger: Box<dyn Logger + Send>,
    targets: HashMap<String, LogLevel>,
}

impl Active
{
    fn configured(config: LogConfig) -> Self
    {
        Self {
            logger: Box::new(DefaultLogger {
                enabled: config.enabled,
                level: config.level,
                output: LazyLock::new(output::Output::new),
            }),
            targets: config.targets,
        }
    }

    fn allows(&self, lvl: LogLevel, target: &str) -> bool
    {
        if lvl == LogLevel::None
        {
            return false;
        }

        let target_level = self
            .targets
            .iter()
            .filter(|(prefix, _)| target.starts_with(prefix.as_str()))
            .max_by_key(|(prefix, _)| prefix.len());
        match target_level
        {
            Some((_, level)) => self.logger.is_enable() && *level != LogLevel::None && lvl <= *level,
            None => self.logger.should_log(lvl),
        }
    }
}

fn active() -> MutexGuard<'static, Active>
{
    register_facade();
    ACTIVE
        .get_or_init(|| {
            let config = FALLBACK.get().cloned().unwrap_or_else(|| LogConfig::default().parse_env());
            Mutex::new(Active::configured(config))
        })
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

/// A zero-sized handle to the process-wide logger.
#[derive(Clone, Copy, Debug, Default)]
pub struct Log;

impl LoggableDyn for Log
{
    fn log_str(&mut self, lvl: LogLevel, msg: LogFragment<'_>)
    {
        if lvl != LogLevel::None
        {
            let msg = prefix_message(msg);
            active().logger.log_str(lvl, msg);
        }
    }
}

impl Enable for Log
{
    fn is_enable(&self) -> bool { active().logger.is_enable() }
}

impl SetEnable for Log
{
    fn set_enable(&mut self, enable: bool) { active().logger.set_enable(enable); }
}

impl Logger for Log
{
    fn level(&self) -> LogLevel { active().logger.level() }

    fn set_level(&mut self, lvl: LogLevel) { active().logger.set_level(lvl); }

    fn should_log(&self, lvl: LogLevel) -> bool { lvl != LogLevel::None && active().logger.should_log(lvl) }
}

/// Registers the logging facade. The default logger is created on demand.
pub fn init_if_needed() { register_facade(); }

/// Supplies fallback settings for the first use of the logger.
pub fn init_if_needed_with(configure: impl FnOnce() -> LogConfig)
{
    if ACTIVE.get().is_none()
    {
        FALLBACK.get_or_init(configure);
    }
    register_facade();
}

/// Replaces the active logger with an application-provided sink.
pub fn set_logger(logger: impl Logger + Send + 'static)
{
    let replacement = Mutex::new(Active {
        logger: Box::new(logger),
        targets: HashMap::new(),
    });
    if let Err(replacement) = ACTIVE.set(replacement)
    {
        let replacement = replacement.into_inner().unwrap_or_else(PoisonError::into_inner);
        let old = std::mem::replace(&mut *active(), replacement);
        drop(old);
    }
    register_facade();
}

pub fn set_enabled(enable: bool) { Log.set_enable(enable); }

/// Sets the current threshold.
pub fn set_level(lvl: LogLevel) { Log.set_level(lvl); }

#[doc(hidden)]
pub fn log_lazy(lvl: impl Into<LogLevel>, target: &str, f: impl FnOnce() -> String)
{
    let lvl = lvl.into();
    if !active().allows(lvl, target)
    {
        return;
    }

    // Formatting can run user code, so release the logger lock first.
    let msg = prefix_message(LogFragment::Owned(f()));

    let mut active = active();
    if active.allows(lvl, target)
    {
        active.logger.log_str(lvl, msg);
    }
}

fn prefix_message(msg: LogFragment<'_>) -> LogFragment<'_>
{
    match crate::prefix::current_prefix()
    {
        Some(prefix) => LogFragment::Owned(format!("[{prefix}] {msg}")),
        None => msg,
    }
}

struct Facade;

fn register_facade()
{
    static FACADE: Facade = Facade;
    static REGISTERED: OnceLock<()> = OnceLock::new();
    REGISTERED.get_or_init(|| {
        if log::set_logger(&FACADE).is_ok()
        {
            log::set_max_level(log::LevelFilter::Trace);
        }
    });
}

impl log::Log for Facade
{
    fn enabled(&self, metadata: &Metadata<'_>) -> bool { active().allows(metadata.level().into(), metadata.target()) }

    fn log(&self, record: &Record<'_>) { log_lazy(record.level(), record.target(), || record.args().to_string()); }

    fn flush(&self) {}
}

struct DefaultLogger
{
    enabled: bool,
    level: LogLevel,
    output: LazyLock<output::Output>,
}

impl LoggableDyn for DefaultLogger
{
    fn log_str(&mut self, lvl: LogLevel, msg: LogFragment<'_>)
    {
        if lvl != LogLevel::None
        {
            self.output.write(msg.as_ref(), lvl);
        }
    }
}

impl Enable for DefaultLogger
{
    fn is_enable(&self) -> bool { self.enabled }
}

impl SetEnable for DefaultLogger
{
    fn set_enable(&mut self, enable: bool) { self.enabled = enable; }
}

impl Logger for DefaultLogger
{
    fn level(&self) -> LogLevel { self.level }
    fn set_level(&mut self, lvl: LogLevel) { self.level = lvl; }
}

#[cfg(all(feature = "log_env", not(all(target_arch = "wasm32", target_os = "unknown"))))]
mod output
{
    use crate::{LogLevel, Record};
    use log::Log;
    use std::io::Write;

    pub(super) struct Output(env_logger::Logger);

    impl Output
    {
        pub(super) fn new() -> Self
        {
            Self(
                env_logger::Builder::new()
                    .filter_level(log::LevelFilter::Trace)
                    .format(|buf, record| writeln!(buf, "{}", record.args()))
                    .build(),
            )
        }

        pub(super) fn write(&self, msg: &str, lvl: LogLevel)
        {
            if let Ok(level) = lvl.try_into()
            {
                self.0.log(&Record::builder().level(level).args(format_args!("{msg}")).build());
            }
        }
    }
}

#[cfg(all(feature = "log_env", target_arch = "wasm32", target_os = "unknown"))]
mod output
{
    use crate::LogLevel;
    use web_sys::console;

    pub(super) struct Output;

    impl Output
    {
        pub(super) fn new() -> Self { Self }

        pub(super) fn write(&self, msg: &str, lvl: LogLevel)
        {
            let msg = msg.into();
            match lvl
            {
                LogLevel::Error => console::error_1(&msg),
                LogLevel::Warn => console::warn_1(&msg),
                LogLevel::Info => console::info_1(&msg),
                LogLevel::Debug | LogLevel::Trace => console::debug_1(&msg),
                LogLevel::None =>
                {}
            }
        }
    }
}

#[cfg(not(feature = "log_env"))]
mod output
{
    use crate::LogLevel;
    use std::io::{Write, stdout};

    pub(super) struct Output;

    impl Output
    {
        pub(super) fn new() -> Self { Self }

        pub(super) fn write(&self, msg: &str, _lvl: LogLevel) { let _ = writeln!(stdout().lock(), "{msg}"); }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::{EnableExtension, LoggableExtension, LoggableLazy, LoggableLazyExtension, ScopedWith, SetEnableExtension};
    use std::sync::Arc;

    type Messages = Arc<Mutex<Vec<(String, LogLevel)>>>;

    struct Capture
    {
        enabled: bool,
        level: LogLevel,
        messages: Messages,
    }

    impl LoggableDyn for Capture
    {
        fn log_str(&mut self, lvl: LogLevel, msg: LogFragment<'_>) { self.messages.lock().unwrap().push((msg.into_owned(), lvl)); }
    }

    impl Enable for Capture
    {
        fn is_enable(&self) -> bool { self.enabled }
    }

    impl SetEnable for Capture
    {
        fn set_enable(&mut self, enable: bool) { self.enabled = enable; }
    }

    impl Logger for Capture
    {
        fn level(&self) -> LogLevel { self.level }
        fn set_level(&mut self, lvl: LogLevel) { self.level = lvl; }
    }

    fn should_not_format() -> &'static str
    {
        panic!("a filtered log evaluated its message");
    }

    struct Unformatted;

    impl fmt::Display for Unformatted
    {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            panic!("a filtered log formatted its message");
        }
    }

    #[test]
    fn default_level_matches_the_build()
    {
        let expected = if cfg!(debug_assertions)
        {
            LogLevel::Debug
        }
        else
        {
            LogLevel::Error
        };
        assert_eq!(LogConfig::default().level, expected);
        let logger = Active::configured(LogConfig::default());
        assert!(logger.logger.is_enable());
        assert_eq!(logger.logger.level(), expected);
    }

    #[test]
    fn level_override_and_enabled_state_are_independent()
    {
        let config = LogConfig::default().parse_filters("off");
        assert_eq!(config.level, DEFAULT_LEVEL);
        let logger = Active::configured(config.filter_level(Level::Info));
        assert!(!logger.logger.is_enable());
        assert_eq!(logger.logger.level(), LogLevel::Info);
    }

    #[test]
    fn longest_matching_filter_wins()
    {
        let logger = Active::configured(LogConfig::default().parse_filters("error, app=info, app=debug, app::noisy=off, app::worker"));
        assert!(!logger.allows(LogLevel::Warn, "other"));
        assert!(logger.allows(LogLevel::Debug, "app"));
        assert!(!logger.allows(LogLevel::Error, "app::noisy::task"));
        assert!(logger.allows(LogLevel::Trace, "app::worker"));

        let unfiltered = Active::configured(LogConfig::default().filter_level(LogLevel::None));
        assert!(unfiltered.allows(LogLevel::Trace, "app"));
        assert!(!unfiltered.allows(LogLevel::None, "app"));
    }

    #[test]
    fn initializes_on_first_use_and_checks_custom_logger_before_formatting()
    {
        init_if_needed_with(|| LogConfig::default().filter_level(LogLevel::Warn));
        assert!(ACTIVE.get().is_none());
        crate::trace!("{}", should_not_format());
        assert!(ACTIVE.get().is_some());
        assert_eq!(LogConfig::default().try_init(), Err(InitError));

        let messages = Messages::default();
        set_logger(Capture {
            enabled: true,
            level: LogLevel::Warn,
            messages: messages.clone(),
        });
        crate::debug!("{}", should_not_format());
        set_enabled(false);
        crate::error!("{}", should_not_format());
        set_enabled(true);

        set_level(LogLevel::Debug);
        Log.scope(42, |_| crate::debug!("hexga"));
        set_level(LogLevel::Warn);
        crate::debug!("{}", should_not_format());
        log::warn!("facade");

        let mut handle = Log;
        assert_eq!(std::mem::size_of_val(&handle), 0);
        assert!(handle.is_enable());
        assert_eq!(handle.level(), LogLevel::Warn);
        handle.disable();
        assert!(Log.is_disable());
        handle.error(Unformatted);
        handle.lazy_error(should_not_format);
        handle.enable();
        handle.set_level(LogLevel::Info);
        assert_eq!(Log.level(), LogLevel::Info);
        handle.debug(Unformatted);
        handle.lazy_debug(should_not_format);
        Log.scope("proxy", |_| handle.lazy_info(|| 42));
        handle.info(7);
        let borrowed = String::from("borrowed");
        handle.lazy_log_str(LogLevel::Info, || LogFragment::Borrowed(&borrowed));
        handle.lazy_log(LogLevel::None, should_not_format);
        handle.log_str(LogLevel::None, LogFragment::Borrowed("ignored"));
        handle.set_level(LogLevel::Warn);
        assert_eq!(Log.level(), LogLevel::Warn);

        let mut local = Capture {
            enabled: false,
            level: LogLevel::Info,
            messages: messages.clone(),
        };
        local.lazy_log_str(LogLevel::Error, || should_not_format().into());
        local.set_enable(true);
        local.lazy_log_str(LogLevel::Debug, || should_not_format().into());
        local.lazy_log_str(LogLevel::Info, || "local".into());

        assert_eq!(
            *messages.lock().unwrap(),
            [
                ("[42] hexga".into(), LogLevel::Debug),
                ("facade".into(), LogLevel::Warn),
                ("[proxy] 42".into(), LogLevel::Info),
                ("7".into(), LogLevel::Info),
                ("borrowed".into(), LogLevel::Info),
                ("local".into(), LogLevel::Info),
            ]
        );
    }
}
