#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError, debug, error, info, log, trace, warn};

pub mod prelude
{
    pub use super::{debug, error, info, trace, warn};
}

pub mod traits {}

#[cfg(feature = "std")]
mod logger;
#[cfg(feature = "std")]
pub use logger::{Config, init_if_needed, init_if_needed_with};

/// Default threshold for records with no matching target filter.
pub const DEFAULT_LEVEL: LevelFilter = if cfg!(debug_assertions)
{
    LevelFilter::Warn
}
else
{
    LevelFilter::Error
};
