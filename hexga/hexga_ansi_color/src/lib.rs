//! A minimal package for printing some Ansi Color
//!
//! Provides optional support for [`Serde`](https://docs.rs/serde/latest/serde/) (serialization / deserialization) when the "serde" feature is enabled.
//!
//! ```rust
//! use hexga_ansi_color::*;
//!
//! println!("{}I'm green{}", AnsiColor::GREEN, AnsiColor::RESET);
//! println!("{}I'm red{}", AnsiColor::new_foreground(AnsiColorKind::Red), AnsiColor::RESET);
//! println!("{}White on magenta background{}", AnsiColor::new(AnsiColorKind::Magenta, AnsiColorLayer::Background), AnsiColor::RESET);
//! ```
//!
//! Based on previous crate [`minimal_ansi_color`](https://crates.io/crates/minimal_ansi_color), but integrated to hexga.

use std::fmt::{Debug, Display};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
#[cfg(feature = "hexga_io")]
use hexga_io::prelude::*;


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AnsiColorKind
{
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Grey,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AnsiColorLayer
{
    Foreground,
    Background,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct AnsiColor
{
    pub color : AnsiColorKind,
    pub layer : AnsiColorLayer,
}

pub type AnsiColorStr = &'static str;

impl AnsiColor
{
    pub const BLACK             : AnsiColorStr = "\x1b[30m";
    pub const RED               : AnsiColorStr = "\x1b[31m";
    pub const GREEN             : AnsiColorStr = "\x1b[32m";
    pub const YELLOW            : AnsiColorStr = "\x1b[33m";
    pub const BLUE              : AnsiColorStr = "\x1b[34m";
    pub const MAGENTA           : AnsiColorStr = "\x1b[35m";
    pub const CYAN              : AnsiColorStr = "\x1b[36m";
    pub const WHITE             : AnsiColorStr = "\x1b[37m";
    pub const GREY              : AnsiColorStr = "\x1b[90m";

    // Same but suffixed by FOREGROUND
    pub const BLACK_FOREGROUND  : AnsiColorStr = "\x1b[30m";
    pub const RED_FOREGROUND    : AnsiColorStr = "\x1b[31m";
    pub const GREEN_FOREGROUND  : AnsiColorStr = "\x1b[32m";
    pub const YELLOW_FOREGROUND : AnsiColorStr = "\x1b[33m";
    pub const BLUE_FOREGROUND   : AnsiColorStr = "\x1b[34m";
    pub const MAGENTA_FOREGROUND: AnsiColorStr = "\x1b[35m";
    pub const CYAN_FOREGROUND   : AnsiColorStr = "\x1b[36m";
    pub const WHITE_FOREGROUND  : AnsiColorStr = "\x1b[37m";
    pub const GREY_FOREGROUND   : AnsiColorStr = "\x1b[90m";

    pub const BLACK_BACKGROUND  : AnsiColorStr = "\x1b[40m";
    pub const RED_BACKGROUND    : AnsiColorStr = "\x1b[41m";
    pub const GREEN_BACKGROUND  : AnsiColorStr = "\x1b[42m";
    pub const YELLOW_BACKGROUND : AnsiColorStr = "\x1b[43m";
    pub const BLUE_BACKGROUND   : AnsiColorStr = "\x1b[44m";
    pub const MAGENTA_BACKGROUND: AnsiColorStr = "\x1b[45m";
    pub const CYAN_BACKGROUND   : AnsiColorStr = "\x1b[46m";
    pub const WHITE_BACKGROUND  : AnsiColorStr = "\x1b[47m";
    pub const GREY_BACKGROUND   : AnsiColorStr = "\x1b[100m";

    pub const BLACK_ON_WHITE : AnsiColorStr   = "\x1b[30m\x1b[47m";
    pub const RESET: AnsiColorStr = "\x1b[37m\x1b[40m";

    pub fn new(color : AnsiColorKind, layer : AnsiColorLayer) -> Self { Self { color, layer }}
    pub fn new_foreground(color : AnsiColorKind) -> Self { Self::new(color, AnsiColorLayer::Foreground) }
    pub fn new_background(color : AnsiColorKind) -> Self { Self::new(color, AnsiColorLayer::Background) }

    pub fn color(&self) -> AnsiColorKind  { self.color }
    pub fn set_color(&mut self, color : AnsiColorKind) -> &mut Self  { self.color = color; self }
    pub fn with_color(mut self, color : AnsiColorKind) -> Self  { self.set_color(color); self }

    pub fn layer(&self) -> AnsiColorLayer  { self.layer }
    pub fn set_layer(&mut self, layer : AnsiColorLayer) -> &mut Self  { self.layer = layer; self }
    pub fn with_layer(mut self, layer : AnsiColorLayer) -> Self  { self.set_layer(layer); self }

    /// Get the ansi color code
    pub fn get_str(&self) -> AnsiColorStr
    {
        match self.layer
        {
            AnsiColorLayer::Foreground =>
            {
                match self.color
                {
                    AnsiColorKind::Black   => Self::BLACK_FOREGROUND,
                    AnsiColorKind::Red     => Self::RED_FOREGROUND,
                    AnsiColorKind::Green   => Self::GREEN_FOREGROUND,
                    AnsiColorKind::Yellow  => Self::YELLOW_FOREGROUND,
                    AnsiColorKind::Blue    => Self::BLUE_FOREGROUND,
                    AnsiColorKind::Magenta => Self::MAGENTA_FOREGROUND,
                    AnsiColorKind::Cyan    => Self::CYAN_FOREGROUND,
                    AnsiColorKind::White   => Self::WHITE_FOREGROUND,
                    AnsiColorKind::Grey    => Self::GREY_FOREGROUND,
                }
            }
            AnsiColorLayer::Background =>
            {
                match self.color
                {
                    AnsiColorKind::Black   => Self::BLACK_BACKGROUND,
                    AnsiColorKind::Red     => Self::RED_BACKGROUND,
                    AnsiColorKind::Green   => Self::GREEN_BACKGROUND,
                    AnsiColorKind::Yellow  => Self::YELLOW_BACKGROUND,
                    AnsiColorKind::Blue    => Self::BLUE_BACKGROUND,
                    AnsiColorKind::Magenta => Self::MAGENTA_BACKGROUND,
                    AnsiColorKind::Cyan    => Self::CYAN_BACKGROUND,
                    AnsiColorKind::White   => Self::WHITE_BACKGROUND,
                    AnsiColorKind::Grey    => Self::GREY_BACKGROUND,
                }
            }
        }
    }
}

impl From<AnsiColor> for AnsiColorStr
{
    fn from(value: AnsiColor) -> Self { value.get_str() }
}

impl Display for AnsiColor
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_str(self.get_str())
    }
}