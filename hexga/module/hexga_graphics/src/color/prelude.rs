use crate::*;

pub use super::rgba::*;
//pub use color_hsla::*;
pub use super::icolor::*;

pub const fn rgba(red : float, green : float, blue : float, alpha : float) -> ColorRgba
{
    ColorRgba::rgba(red, green, blue, alpha)
}
pub const fn rgb(red : float, green : float, blue : float) -> ColorRgba
{
    ColorRgba::rgb(red, green, blue)
}

pub const fn rgba_byte(red : u8, green : u8, blue : u8, alpha : u8) -> ColorRgbaU8
{
    ColorRgbaU8::rgba(red, green, blue, alpha)
}
pub const fn rgb_byte(red : u8, green : u8, blue : u8) -> ColorRgbaU8
{
    ColorRgbaU8::rgb(red, green, blue)
}