use std::borrow::Cow;
use std::mem;
use std::slice;

use crate::*;

pub enum ColorKind
{
    RGBABool,
    RGBAByte,
    RGBAU16,
    RGBAF32,
    RGBAF64,
    HSLAF32,
    HSLAF64,
    Unknow,
}

pub trait ToColorComposite
{
    const COLOR_INSIDE : ColorKind;

    #[cfg(any(
        feature = "float_are_32_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "32")
    ))]
    fn to_color(&self) -> Self::ColorRGBAF32 { self.to_color_rgba_float() }
    #[cfg(any(
        feature = "float_are_64_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "64")
    ))]
    fn to_color(&self) -> Self::ColorRGBAF64 { self.to_color_rgba_float() }

    fn to_color_byte(&self) -> Self::ColorRGBAByte { self.to_color_rgba_byte() }

    #[cfg(any(
        feature = "float_are_32_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "32")
    ))]
    fn to_color_rgba(&self) -> Self::ColorRGBAF32 { self.to_color_rgba_float() }
    #[cfg(any(
        feature = "float_are_64_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "64")
    ))]
    fn to_color_rgba(&self) -> Self::ColorRGBAF64 { self.to_color_rgba_float() }

    type ColorRGBAF32;
    fn to_color_rgba_f32(&self) -> Self::ColorRGBAF32;

    type ColorRGBAF64;
    fn to_color_rgba_f64(&self) -> Self::ColorRGBAF64;

    #[cfg(any(
        feature = "float_are_32_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "32")
    ))]
    fn to_color_rgba_float(&self) -> Self::ColorRGBAF32 { self.to_color_rgba_f32() }
    #[cfg(any(
        feature = "float_are_64_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "64")
    ))]
    fn to_color_rgba_float(&self) -> Self::ColorRGBAF64 { self.to_color_rgba_f64() }

    type ColorRGBAByte;
    fn to_color_rgba_byte(&self) -> Self::ColorRGBAByte;

    type ColorRGBABool;
    fn to_color_rgba_bool(&self) -> Self::ColorRGBABool;
    fn to_color_rgba_mask(&self) -> Self::ColorRGBABool { self.to_color_rgba_bool() }



    #[cfg(any(
        feature = "float_are_32_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "32")
    ))]
    fn to_color_hsla(&self) -> Self::ColorHSLAF32 { self.to_color_hsla_f32() }
    #[cfg(any(
        feature = "float_are_64_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "64")
    ))]
    fn to_color_hsla(&self) -> Self::ColorHSLAF64 { self.to_color_hsla_f64() }

    #[cfg(any(
        feature = "float_are_32_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "32")
    ))]
    fn to_color_hsla_float(&self) -> Self::ColorHSLAF32 { self.to_color_hsla_f32() }
    #[cfg(any(
        feature = "float_are_64_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "64")
    ))]
    fn to_color_hsla_float(&self) -> Self::ColorHSLAF64 { self.to_color_hsla_f64() }

    type ColorHSLAF32;
    fn to_color_hsla_f32(&self) -> Self::ColorHSLAF32;
    type ColorHSLAF64;
    fn to_color_hsla_f64(&self) -> Self::ColorHSLAF64;
}

impl<T, const N : usize> ToColorComposite for [T;N] where T : ToColorComposite
{
    type ColorRGBAF32 = [T::ColorRGBAF32;N];
    fn to_color_rgba_f32(&self) -> Self::ColorRGBAF32 { std::array::from_fn(|i| self[i].to_color_rgba_f32()) }

    type ColorRGBAF64 = [T::ColorRGBAF64;N];
    fn to_color_rgba_f64(&self) -> Self::ColorRGBAF64 { std::array::from_fn(|i| self[i].to_color_rgba_f64()) }

    type ColorRGBAByte = [T::ColorRGBAByte;N];
    fn to_color_rgba_byte(&self) -> Self::ColorRGBAByte { std::array::from_fn(|i| self[i].to_color_rgba_byte()) }

    type ColorRGBABool = [T::ColorRGBABool;N];
    fn to_color_rgba_bool(&self) -> Self::ColorRGBABool { std::array::from_fn(|i| self[i].to_color_rgba_bool()) }

    type ColorHSLAF32 = [T::ColorHSLAF32;N];
    fn to_color_hsla_f32(&self) -> Self::ColorHSLAF32 { std::array::from_fn(|i| self[i].to_color_hsla_f32()) }

    type ColorHSLAF64 = [T::ColorHSLAF64;N];
    fn to_color_hsla_f64(&self) -> Self::ColorHSLAF64 { std::array::from_fn(|i| self[i].to_color_hsla_f64()) }

    const COLOR_INSIDE : ColorKind = T::COLOR_INSIDE;
}

impl<T> ToColorComposite for [T] where T : ToColorComposite
{
    type ColorRGBAF32 = Vec<T::ColorRGBAF32>;
    fn to_color_rgba_f32(&self) -> Self::ColorRGBAF32 { self.iter().map(|v| v.to_color_rgba_f32()).collect() }

    type ColorRGBAF64 = Vec<T::ColorRGBAF64>;
    fn to_color_rgba_f64(&self) -> Self::ColorRGBAF64 { self.iter().map(|v| v.to_color_rgba_f64()).collect() }

    type ColorRGBAByte = Vec<T::ColorRGBAByte>;
    fn to_color_rgba_byte(&self) -> Self::ColorRGBAByte { self.iter().map(|v| v.to_color_rgba_byte()).collect() }

    type ColorRGBABool = Vec<T::ColorRGBABool>;
    fn to_color_rgba_bool(&self) -> Self::ColorRGBABool { self.iter().map(|v| v.to_color_rgba_bool()).collect() }

    type ColorHSLAF32 = Vec<T::ColorHSLAF32>;
    fn to_color_hsla_f32(&self) -> Self::ColorHSLAF32 { self.iter().map(|v| v.to_color_hsla_f32()).collect() }

    type ColorHSLAF64 = Vec<T::ColorHSLAF64>;
    fn to_color_hsla_f64(&self) -> Self::ColorHSLAF64 { self.iter().map(|v| v.to_color_hsla_f64()).collect() }

    const COLOR_INSIDE : ColorKind = T::COLOR_INSIDE;
}

pub trait ToColor :
    ToColorComposite
    <
        ColorRGBAF32 = ColorRGBAOf<f32>, ColorRGBAF64 = ColorRGBAOf<f64>,
        ColorHSLAF32 = ColorHSLAF32, ColorHSLAF64 = ColorHSLAF64,
        ColorRGBAByte = ColorRGBAByte,
        ColorRGBABool = ColorRGBABool,
    > + Copy {}
impl<T> ToColor for T where T: ToColorComposite
    <
        ColorRGBAF32 = ColorRGBAOf<f32>, ColorRGBAF64 = ColorRGBAOf<f64>,
        ColorHSLAF32 = ColorHSLAF32, ColorHSLAF64 = ColorHSLAF64,
        ColorRGBAByte = ColorRGBAByte,
        ColorRGBABool = ColorRGBABool,
    > + Copy {}

/// Constant color name are based on <https://colornames.org/>
///
/// (+-1 u8 unit per channel, otherwise `#FF7F00` should be named `Orange Juice` and not `Orange`, because `Orange` is `#ff7f00`)
pub trait IColor<T> : Sized + ToColor
    where T : Primitive
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


    fn to_color_rgba_of<T2>(self) -> ColorRGBAOf<T2> where T2 : Primitive + CastRangeFrom<T>;
    fn to_color_hsla_of<T2>(self) -> ColorHSLAOf<T2> where T2 : Float + CastRangeFrom<T>;

    fn rgba_from_hex(hex: u32) -> ColorRGBAOf<T> where T : CastRangeFrom<u8>
    {
        let [r,g,b,a] = hex.to_be_bytes();
        Self::rgba_from_bytes(r,g,b,a)
    }
    fn rgba_from_array(rgba : [u8;4]) -> ColorRGBAOf<T> where T : CastRangeFrom<u8>
    {
        let [r,g,b,a] = rgba;
        Self::rgba_from_bytes(r,g,b,a)
    }
    fn rgba_from_bytes(r : u8, g : u8, b : u8, a : u8) -> ColorRGBAOf<T> where T : CastRangeFrom<u8>
    {
        ColorRGBAByte::new(r, g, b, a).to_color_rgba_of()
    }

    /// Cast to color byte and format the color : `#RRGGBBAA`
    fn to_rgba_byte_hex_string(self) -> String
    {
        let rgba = self.to_color_byte();
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            rgba.r,
            rgba.g,
            rgba.b,
            rgba.a,
        )
    }
}