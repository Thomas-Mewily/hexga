#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

extern crate alloc;

use alloc::{borrow::Cow, string::ToString};
use core::fmt::Display;

pub use ::log::{Level, Log as FacadeLog, Metadata, Record, SetLoggerError};
pub use hexga_core_utils::{Enable, EnableExtension, ScopedWith, SetEnable, SetEnableExtension};

pub mod prelude
{
    pub use super::traits::*;
    pub use super::{debug, error, info, trace, warn};
}

pub mod traits
{
    pub use super::{
        Enable, EnableExtension, Loggable, LoggableDyn, LoggableExtension, LoggableLazy, LoggableLazyExtension, Logger, ScopedWith, SetEnable,
        SetEnableExtension,
    };
}

#[cfg(feature = "std")]
mod logger;
#[cfg(feature = "std")]
pub use logger::{LogConfig, InitError, Log, init_if_needed, init_if_needed_with, set_enabled, set_level, set_logger};

#[cfg(feature = "std")]
mod prefix;

/// Severity ordered from the most severe to the most verbose.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel
{
    Error,
    Warn,
    Info,
    Debug,
    Trace,
    /// No importance, skipped, never logged
    None,
}

impl From<Level> for LogLevel
{
    fn from(level: Level) -> Self
    {
        match level
        {
            Level::Error => Self::Error,
            Level::Warn => Self::Warn,
            Level::Info => Self::Info,
            Level::Debug => Self::Debug,
            Level::Trace => Self::Trace,
        }
    }
}

impl TryFrom<LogLevel> for Level
{
    type Error = LogLevel;

    fn try_from(level: LogLevel) -> Result<Self, LogLevel>
    {
        Ok(match level
        {
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace => Self::Trace,
            LogLevel::None => return Err(level),
        })
    }
}

pub const DEFAULT_LEVEL: LogLevel = if cfg!(debug_assertions)
{
    LogLevel::Debug
}
else
{
    LogLevel::Error
};

/// Borrowed or owned text for a log message.
pub type LogFragment<'a> = Cow<'a, str>;

/// Object-safe sink for a formatted log message.
pub trait LoggableDyn
{
    fn log_str(&mut self, lvl: LogLevel, msg: LogFragment<'_>);
}

/// Accepts a displayable value and forwards its formatted message to the sink.
pub trait Loggable: LoggableDyn
{
    fn log(&mut self, msg: impl Display, lvl: LogLevel);
}

impl<T: Logger + ?Sized> Loggable for T
{
    fn log(&mut self, msg: impl Display, lvl: LogLevel)
    {
        if lvl != LogLevel::None && self.should_log(lvl)
        {
            self.log_str(lvl, LogFragment::Owned(msg.to_string()));
        }
    }
}

/// Level-specific shortcuts that forward directly to `Loggable::log`.
pub trait LoggableExtension: Loggable
{
    fn error(&mut self, message: impl Display) { self.log(message, LogLevel::Error); }
    fn warn(&mut self, message: impl Display) { self.log(message, LogLevel::Warn); }
    fn info(&mut self, message: impl Display) { self.log(message, LogLevel::Info); }
    fn debug(&mut self, message: impl Display) { self.log(message, LogLevel::Debug); }
    fn trace(&mut self, message: impl Display) { self.log(message, LogLevel::Trace); }
}

impl<T: Loggable + ?Sized> LoggableExtension for T {}

/// A log sink with runtime filtering.
pub trait Logger: LoggableDyn + Enable + SetEnable
{
    fn level(&self) -> LogLevel;
    /// Sets the threshold. `LogLevel::None` permits all actual levels.
    fn set_level(&mut self, lvl: LogLevel);

    fn should_log(&self, lvl: LogLevel) -> bool { lvl != LogLevel::None && self.is_enable() && lvl <= self.level() }
}

/// Formats a message only when it passes the logger's filter.
pub trait LoggableLazy: Logger
{
    fn lazy_log_str<'a, F>(&mut self, lvl: LogLevel, f: F)
    where
        F: FnOnce() -> LogFragment<'a>,
    {
        if lvl != LogLevel::None && self.should_log(lvl)
        {
            self.log_str(lvl, f());
        }
    }

    fn lazy_log<F, D>(&mut self, lvl: LogLevel, f: F)
    where
        F: FnOnce() -> D,
        D: Display,
    {
        self.lazy_log_str(lvl, || LogFragment::Owned(f().to_string()));
    }
}

impl<T: Logger + ?Sized> LoggableLazy for T {}

/// Level-specific shortcuts that defer message creation and formatting.
pub trait LoggableLazyExtension: LoggableLazy
{
    fn lazy_error<S: Display>(&mut self, message: impl FnOnce() -> S) { self.lazy_log(LogLevel::Error, message); }
    fn lazy_warn<S: Display>(&mut self, message: impl FnOnce() -> S) { self.lazy_log(LogLevel::Warn, message); }
    fn lazy_info<S: Display>(&mut self, message: impl FnOnce() -> S) { self.lazy_log(LogLevel::Info, message); }
    fn lazy_debug<S: Display>(&mut self, message: impl FnOnce() -> S) { self.lazy_log(LogLevel::Debug, message); }
    fn lazy_trace<S: Display>(&mut self, message: impl FnOnce() -> S) { self.lazy_log(LogLevel::Trace, message); }
}

impl<T: LoggableLazy + ?Sized> LoggableLazyExtension for T {}

#[doc(hidden)]
pub mod __private
{
    pub use alloc::format;

    #[cfg(feature = "std")]
    pub use crate::logger::log_lazy;

    #[cfg(not(feature = "std"))]
    pub fn log_lazy(lvl: impl Into<crate::LogLevel>, target: &str, f: impl FnOnce() -> alloc::string::String)
    {
        let Ok(level) = crate::Level::try_from(lvl.into())
        else
        {
            return;
        };
        if ::log::log_enabled!(target: target, level)
        {
            ::log::log!(target: target, level, "{}", f());
        }
    }
}

/// Logs with a runtime level. Message arguments are only evaluated if enabled.
#[macro_export]
macro_rules! log {
    (target: $target:expr, $level:expr, $($args:tt)+) => {
        $crate::__private::log_lazy($level, $target, || $crate::__private::format!($($args)+))
    };
    ($level:expr, $($args:tt)+) => {
        $crate::log!(target: module_path!(), $level, $($args)+)
    };
}

#[macro_export]
macro_rules! error {
    (target: $target:expr, $($args:tt)+) => { $crate::log!(target: $target, $crate::LogLevel::Error, $($args)+) };
    ($($args:tt)+) => { $crate::log!($crate::LogLevel::Error, $($args)+) };
}

#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($args:tt)+) => { $crate::log!(target: $target, $crate::LogLevel::Warn, $($args)+) };
    ($($args:tt)+) => { $crate::log!($crate::LogLevel::Warn, $($args)+) };
}

#[macro_export]
macro_rules! info {
    (target: $target:expr, $($args:tt)+) => { $crate::log!(target: $target, $crate::LogLevel::Info, $($args)+) };
    ($($args:tt)+) => { $crate::log!($crate::LogLevel::Info, $($args)+) };
}

#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($args:tt)+) => { $crate::log!(target: $target, $crate::LogLevel::Debug, $($args)+) };
    ($($args:tt)+) => { $crate::log!($crate::LogLevel::Debug, $($args)+) };
}

#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($args:tt)+) => { $crate::log!(target: $target, $crate::LogLevel::Trace, $($args)+) };
    ($($args:tt)+) => { $crate::log!($crate::LogLevel::Trace, $($args)+) };
}
