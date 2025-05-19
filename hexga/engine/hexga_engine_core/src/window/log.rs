// Todo : use a dedicated crate ?
use crate::*;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!("[LOG]: {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!("{}[WARN]: {}{}", hexga::ansi_color::AnsiColor::YELLOW, format!($($arg)*), hexga::ansi_color::AnsiColor::RESET);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("{}[ERR]: {}{}", hexga::ansi_color::AnsiColor::RED, format!($($arg)*), hexga::ansi_color::AnsiColor::RESET);
    };
}

pub use crate::{log, warn, error};
