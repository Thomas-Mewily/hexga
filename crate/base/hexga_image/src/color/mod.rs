use super::*;

pub mod prelude
{
    pub use super::{
        Color, ColorF32, ColorF64, ColorFloat, ColorU8, ColorU16,
        hsla::{Hsla, HslaF32, HslaF64, HslaFloat, HslaOf, hsl, hsla},
        icolor::{ArrayToColor, IColor, ToColor},
        rgba::{Rgba, RgbaF32, RgbaF64, RgbaFloat, RgbaOf, RgbaU8, RgbaU16, rgb, rgba},
    };
}

pub type Color = Rgba;
pub type ColorFloat = RgbaFloat;
pub type ColorF32 = RgbaF32;
pub type ColorF64 = RgbaF64;
pub type ColorU8 = RgbaU8;
pub type ColorU16 = RgbaU16;

pub mod rgba;
pub use rgba::*;

pub mod hsla;
pub use hsla::*;

mod icolor;
pub use icolor::*;
