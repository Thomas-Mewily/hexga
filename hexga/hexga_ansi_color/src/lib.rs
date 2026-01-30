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
impl AnsiColorKind
{
    pub const ALL : &'static [Self] =
    {
        pub use AnsiColorKind::*;
        &[Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, Grey]
    };
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum AnsiColorLayer
{
    #[default]
    Foreground,
    Background,
}
impl AnsiColorLayer
{
    pub const ALL : &'static [Self] =
    {
        pub use AnsiColorLayer::*;
        &[Foreground, Background]
    };
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct AnsiColor
{
    pub color : AnsiColorKind,
    pub layer : AnsiColorLayer,
}
impl From<AnsiColorKind> for AnsiColor {
    fn from(color: AnsiColorKind) -> Self { Self { color, layer: AnsiColorLayer::default() } }
}
impl From<(AnsiColorKind,AnsiColorLayer)> for AnsiColor {
    fn from((color, layer): (AnsiColorKind,AnsiColorLayer)) -> Self { Self { color, layer } }
}

pub type AnsiColorStr = &'static str;

impl AnsiColor
{
    #[rustfmt::skip] pub const BLACK             : AnsiColorStr = "\x1b[30m";
    #[rustfmt::skip] pub const RED               : AnsiColorStr = "\x1b[31m";
    #[rustfmt::skip] pub const GREEN             : AnsiColorStr = "\x1b[32m";
    #[rustfmt::skip] pub const YELLOW            : AnsiColorStr = "\x1b[33m";
    #[rustfmt::skip] pub const BLUE              : AnsiColorStr = "\x1b[34m";
    #[rustfmt::skip] pub const MAGENTA           : AnsiColorStr = "\x1b[35m";
    #[rustfmt::skip] pub const CYAN              : AnsiColorStr = "\x1b[36m";
    #[rustfmt::skip] pub const WHITE             : AnsiColorStr = "\x1b[37m";
    #[rustfmt::skip] pub const GREY              : AnsiColorStr = "\x1b[90m";

    // Same but suffixed by FOREGROUND
    #[rustfmt::skip] pub const BLACK_FOREGROUND  : AnsiColorStr = "\x1b[30m";
    #[rustfmt::skip] pub const RED_FOREGROUND    : AnsiColorStr = "\x1b[31m";
    #[rustfmt::skip] pub const GREEN_FOREGROUND  : AnsiColorStr = "\x1b[32m";
    #[rustfmt::skip] pub const YELLOW_FOREGROUND : AnsiColorStr = "\x1b[33m";
    #[rustfmt::skip] pub const BLUE_FOREGROUND   : AnsiColorStr = "\x1b[34m";
    #[rustfmt::skip] pub const MAGENTA_FOREGROUND: AnsiColorStr = "\x1b[35m";
    #[rustfmt::skip] pub const CYAN_FOREGROUND   : AnsiColorStr = "\x1b[36m";
    #[rustfmt::skip] pub const WHITE_FOREGROUND  : AnsiColorStr = "\x1b[37m";
    #[rustfmt::skip] pub const GREY_FOREGROUND   : AnsiColorStr = "\x1b[90m";

    #[rustfmt::skip] pub const BLACK_BACKGROUND  : AnsiColorStr = "\x1b[40m";
    #[rustfmt::skip] pub const RED_BACKGROUND    : AnsiColorStr = "\x1b[41m";
    #[rustfmt::skip] pub const GREEN_BACKGROUND  : AnsiColorStr = "\x1b[42m";
    #[rustfmt::skip] pub const YELLOW_BACKGROUND : AnsiColorStr = "\x1b[43m";
    #[rustfmt::skip] pub const BLUE_BACKGROUND   : AnsiColorStr = "\x1b[44m";
    #[rustfmt::skip] pub const MAGENTA_BACKGROUND: AnsiColorStr = "\x1b[45m";
    #[rustfmt::skip] pub const CYAN_BACKGROUND   : AnsiColorStr = "\x1b[46m";
    #[rustfmt::skip] pub const WHITE_BACKGROUND  : AnsiColorStr = "\x1b[47m";
    #[rustfmt::skip] pub const GREY_BACKGROUND   : AnsiColorStr = "\x1b[100m";

    #[rustfmt::skip] pub const BLACK_ON_WHITE    : AnsiColorStr = "\x1b[30m\x1b[47m";
    #[rustfmt::skip] pub const WHITE_ON_BLACK    : AnsiColorStr = "\x1b[37m\x1b[40m";
    #[rustfmt::skip] pub const RESET             : AnsiColorStr = Self::WHITE_ON_BLACK;

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
    pub fn str(&self) -> AnsiColorStr
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
    fn from(value: AnsiColor) -> Self { value.str() }
}

impl Display for AnsiColor
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_str(self.str())
    }
}