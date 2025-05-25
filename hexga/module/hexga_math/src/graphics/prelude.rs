use crate::*;

pub use super::color_rgba::*;
//pub use color_hsla::*;
pub use super::icolor::*;

pub fn rgba(red : float, green : float, blue : float, alpha : float) -> ColorRGBA
{
    ColorRGBA::new(red, green, blue, alpha)
}

pub fn rgba_byte(red : u8, green : u8, blue : u8, alpha : u8) -> ColorRGBAByte
{
    ColorRGBAByte::new(red, green, blue, alpha)
}