use super::*;


pub mod prelude
{
    pub use super::
    {
        Color,ColorU8,
        rgba::{Rgba,RgbaU8,RgbaFloat,RgbaOf},
        hsla::{Hsla,HslaFloat,HslaOf},
        icolor::{IColor,ColorArrayExtension},
    };
}

pub type Color      = Rgba;
pub type ColorU8    = RgbaU8;


pub mod rgba;
pub use rgba::*;

pub mod hsla;
pub use hsla::*;

mod icolor;
pub use icolor::*;