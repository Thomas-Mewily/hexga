use super::*;


pub mod prelude
{
    pub use super::
    {
        Color, ColorU8, ColorU16, ColorFloat, ColorF32, ColorF64,
        rgba::{Rgba, RgbaU8, RgbaU16, RgbaFloat, RgbaF32, RgbaF64, RgbaOf, rgba, rgb},
        hsla::{Hsla, HslaFloat, HslaOf, HslaF32, HslaF64, hsla, hsl},
        icolor::{IColor, ArrayToColor,ToColor},
    };
}

pub type Color      = Rgba;
pub type ColorFloat = RgbaFloat;
pub type ColorF32   = RgbaF32;
pub type ColorF64   = RgbaF64;
pub type ColorU8    = RgbaU8;
pub type ColorU16   = RgbaU16;


pub mod rgba;
pub use rgba::*;

pub mod hsla;
pub use hsla::*;

mod icolor;
pub use icolor::*;