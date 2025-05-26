use crate::*;

pub use super::color_rgba::*;
//pub use color_hsla::*;
pub use super::icolor::*;

pub const fn rgba(red : float, green : float, blue : float, alpha : float) -> ColorRGBA
{
    ColorRGBA::rgba(red, green, blue, alpha)
}
pub const fn rgb(red : float, green : float, blue : float) -> ColorRGBA
{
    ColorRGBA::rgb(red, green, blue)
}

pub const fn rgba_byte(red : u8, green : u8, blue : u8, alpha : u8) -> ColorRGBAByte
{
    ColorRGBAByte::rgba(red, green, blue, alpha)
}
pub const fn rgb_byte(red : u8, green : u8, blue : u8) -> ColorRGBAByte
{
    ColorRGBAByte::rgb(red, green, blue)
}