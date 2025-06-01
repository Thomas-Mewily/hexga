use std::borrow::Cow;
use std::mem;
use std::slice;

use crate::*;

pub enum ColorKind
{
    /// Pixel is 1-bit RGB with an alpha channel
    RgbaBool,
    /// Pixel is 8-bit RGB with an alpha channel
    RgbaU8,
    /// Pixel is 16-bit RGBA with an alpha channel
    RgbaU16,
    /// Pixel is 32-bit float RGB with an alpha channel. Casted to RGBAU16 if not supported
    RgbaF32,
    /// Pixel is 64-bit float RGB with an alpha channel. Casted to RGBAF32 if not supported
    RgbaF64,
    /// Pixel is 32-bit float HSL with an alpha channel. Casted to RGBAByte if not supported
    HslaF32,
    /// Pixel is 64-bit float HSL with an alpha channel. Casted to RGBAByte if not supported
    HslaF64,
    // Casted to RGBAByte if not supported
    Unknow,
}

pub trait ToColorComposite
{
    const COLOR_INSIDE : ColorKind;

    #[cfg(any(
        feature = "float_are_32_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "32")
    ))]
    fn to_color(&self) -> Self::RgbaF32 { self.to_rgba() }
    #[cfg(any(
        feature = "float_are_64_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "64")
    ))]
    fn to_color(&self) -> Self::RgbaF64 { self.to_color_rgba_float() }

    fn to_color_u8(&self) -> Self::RgbaU8 { self.to_rgba_u8() }


    type RgbaF32;
    fn to_rgba_f32(&self) -> Self::RgbaF32;

    type RgbaF64;
    fn to_rgba_f64(&self) -> Self::RgbaF64;

    #[cfg(any(
        feature = "float_are_32_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "32")
    ))]
    fn to_rgba(&self) -> Self::RgbaF32 { self.to_rgba_f32() }
    #[cfg(any(
        feature = "float_are_64_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "64")
    ))]
    fn to_rgba(&self) -> Self::RgbaF64 { self.to_color_rgba_f64() }

    type RgbaU8;
    fn to_rgba_u8(&self) -> Self::RgbaU8;

    type RgbaU16;
    fn to_rgba_u16(&self) -> Self::RgbaU16;

    type RgbaBool;
    fn to_rgba_bool(&self) -> Self::RgbaBool;
    fn to_rgba_mask(&self) -> Self::RgbaBool { self.to_rgba_bool() }


    #[cfg(any(
        feature = "float_are_32_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "32")
    ))]
    fn to_hsla(&self) -> Self::HslaF32 { self.to_hsla_f32() }
    #[cfg(any(
        feature = "float_are_64_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "64")
    ))]
    fn to_hsla(&self) -> Self::HslaF64 { self.to_color_hsla_f64() }

    type HslaF32;
    fn to_hsla_f32(&self) -> Self::HslaF32;
    type HslaF64;
    fn to_hsla_f64(&self) -> Self::HslaF64;
}

impl<T, const N : usize> ToColorComposite for [T;N] where T: ToColorComposite
{
    type RgbaF32 = [T::RgbaF32;N];
    fn to_rgba_f32(&self) -> Self::RgbaF32 { std::array::from_fn(|i| self[i].to_rgba_f32()) }

    type RgbaF64 = [T::RgbaF64;N];
    fn to_rgba_f64(&self) -> Self::RgbaF64 { std::array::from_fn(|i| self[i].to_rgba_f64()) }

    type RgbaU8 = [T::RgbaU8;N];
    fn to_rgba_u8(&self) -> Self::RgbaU8 { std::array::from_fn(|i| self[i].to_rgba_u8()) }

    type RgbaU16 = [T::RgbaU16;N];
    fn to_rgba_u16(&self) -> Self::RgbaU16 { std::array::from_fn(|i| self[i].to_rgba_u16()) }

    type RgbaBool = [T::RgbaBool;N];
    fn to_rgba_bool(&self) -> Self::RgbaBool { std::array::from_fn(|i| self[i].to_rgba_bool()) }

    type HslaF32 = [T::HslaF32;N];
    fn to_hsla_f32(&self) -> Self::HslaF32 { std::array::from_fn(|i| self[i].to_hsla_f32()) }

    type HslaF64 = [T::HslaF64;N];
    fn to_hsla_f64(&self) -> Self::HslaF64 { std::array::from_fn(|i| self[i].to_hsla_f64()) }

    const COLOR_INSIDE : ColorKind = T::COLOR_INSIDE;
}

impl<T> ToColorComposite for [T] where T: ToColorComposite
{
    type RgbaF32 = Vec<T::RgbaF32>;
    fn to_rgba_f32(&self) -> Self::RgbaF32 { self.iter().map(|v| v.to_rgba_f32()).collect() }

    type RgbaF64 = Vec<T::RgbaF64>;
    fn to_rgba_f64(&self) -> Self::RgbaF64 { self.iter().map(|v| v.to_rgba_f64()).collect() }

    type RgbaU8 = Vec<T::RgbaU8>;
    fn to_rgba_u8(&self) -> Self::RgbaU8 { self.iter().map(|v| v.to_rgba_u8()).collect() }

    type RgbaU16 = Vec<T::RgbaU16>;
    fn to_rgba_u16(&self) -> Self::RgbaU16 { self.iter().map(|v| v.to_rgba_u16()).collect() }

    type RgbaBool = Vec<T::RgbaBool>;
    fn to_rgba_bool(&self) -> Self::RgbaBool { self.iter().map(|v| v.to_rgba_bool()).collect() }

    type HslaF32 = Vec<T::HslaF32>;
    fn to_hsla_f32(&self) -> Self::HslaF32 { self.iter().map(|v| v.to_hsla_f32()).collect() }

    type HslaF64 = Vec<T::HslaF64>;
    fn to_hsla_f64(&self) -> Self::HslaF64 { self.iter().map(|v| v.to_hsla_f64()).collect() }

    const COLOR_INSIDE : ColorKind = T::COLOR_INSIDE;
}

pub trait ToColor :
    ToColorComposite
    <
        RgbaF32 = ColorRgbaOf<f32>, RgbaF64 = ColorRgbaOf<f64>,
        HslaF32 = ColorHslaF32, HslaF64 = ColorHslaF64,
        RgbaU8 = ColorRgbaU8,
        RgbaBool = ColorRgbaBool,
    > + Copy {}
impl<T> ToColor for T where T: ToColorComposite
    <
        RgbaF32 = ColorRgbaOf<f32>, RgbaF64 = ColorRgbaOf<f64>,
        HslaF32 = ColorHslaF32, HslaF64 = ColorHslaF64,
        RgbaU8 = ColorRgbaU8,
        RgbaBool = ColorRgbaBool,
    > + Copy {}

/// Constant color name are based on <https://colornames.org/>
///
/// (+-1 u8 unit per channel, otherwise `#FF7F00` should be named `Orange Juice` and not `Orange`, because `Orange` is `#ff7f00`)
pub trait IColor<T> : Sized + ToColor
    where T: Primitive
{
    const TRANSPARENT : Self;

    /// #000000
    ///
    /// ⬛ : ⬛⬛⬛
    const BLACK : Self;
    /// #777777
    const GRAY  : Self;
    /// #FFFFFF
    ///
    /// ⬜ : 🟥🟩🟦
    const WHITE : Self;

    /// #FF0000
    ///
    /// 🟥 : 🟥⬛⬛
    const RED    : Self;
    /// #00FF00
    ///
    /// 🟩 : ⬛🟩⬛
    const GREEN  : Self;
    /// #0000FF
    ///
    /// 🟦 : ⬛⬛🟦
    const BLUE   : Self;

    /// #00FFFF
    ///
    /// _ : ⬛🟩🟦
    const CYAN   : Self;

    /// #FF00FF
    ///
    /// _ : 🟥⬛🟦
    const MAGENTA   : Self;

    /// #FFFF00
    ///
    /// 🟨 : 🟥🟩⬛
    const YELLOW : Self;

    /// #00FF7F
    const SPRING : Self;
    /// #007FFF
    const AZURE : Self;
    /// #7F00FF
    const VIOLET : Self;
    /// #FF007F
    const ROSE : Self;
    /// #FF7F00
    const ORANGE : Self;
    /// #7FFF00
    const LIME : Self;
    /// #FFFF7F
    const CANARY : Self; // hard to find an official name for this one with the website
    /// #FF7FFF
    const PINK : Self; // hard to find an official name for this one with the website
    /// #7FFFFF
    const GLACE : Self; // hard to find an official name for this one with the website


    fn to_rgba_of<T2>(self) -> ColorRgbaOf<T2> where T2 : Primitive + CastRangeFrom<T>;
    fn to_hsla_of<T2>(self) -> ColorHslaOf<T2> where T2 : Float + CastRangeFrom<T>;

    fn rgba_from_hex(hex: u32) -> ColorRgbaOf<T> where T: CastRangeFrom<u8>
    {
        let [r,g,b,a] = hex.to_be_bytes();
        Self::rgba_from_bytes(r,g,b,a)
    }
    fn rgba_from_array(rgba : [u8;4]) -> ColorRgbaOf<T> where T: CastRangeFrom<u8>
    {
        let [r,g,b,a] = rgba;
        Self::rgba_from_bytes(r,g,b,a)
    }
    fn rgba_from_bytes(r : u8, g : u8, b : u8, a : u8) -> ColorRgbaOf<T> where T: CastRangeFrom<u8>
    {
        ColorRgbaU8::new(r, g, b, a).to_rgba_of()
    }

    /// Cast to color byte and format the color : `#RRGGBBAA`
    fn to_rgba_u8_hex_string(self) -> String
    {
        let rgba = self.to_color_u8();
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            rgba.r,
            rgba.g,
            rgba.b,
            rgba.a,
        )
    }
}