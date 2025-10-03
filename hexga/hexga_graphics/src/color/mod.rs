use super::*;


pub mod prelude
{
    pub use super::
    {
        Color, ColorFloat, ColorU8, ColorU16,
        rgba::{Rgba, RgbaU8, RgbaU16, RgbaFloat, RgbaOf, rgba, rgb},
        hsla::{Hsla, HslaFloat, HslaOf, hsla, hsl},
        icolor::{IColor, ArrayToColor,ToColor},
    };
}

pub type Color      = Rgba;
pub type ColorFloat = RgbaFloat;
pub type ColorU8    = RgbaU8;
pub type ColorU16   = RgbaU16;


pub mod rgba;
pub use rgba::*;

pub mod hsla;
pub use hsla::*;

mod icolor;
pub use icolor::*;