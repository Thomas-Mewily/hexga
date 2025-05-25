use std::borrow::Cow;
use std::mem;
use std::slice;

use crate::*;

pub trait ToColor : Sized
{   
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


pub trait IColor<T> : 
    Sized +
    ToColor
    <
        ColorRGBAF32 = ColorRGBAOf<f32>, ColorRGBAF64 = ColorRGBAOf<f64>, 
        ColorHSLAF32 = ColorHSLAF32, ColorHSLAF64 = ColorHSLAF64, 
        ColorRGBAByte = ColorRGBAByte,
        ColorRGBABool = ColorRGBABool,
    >
    where T : Primitive
    //From<ColorRGBAOf<f32>> + From<ColorRGBAOf<f64>> + From<ColorRGBAByte> + From<ColorHSLA> +
    //Into<Color> + Into<ColorByte> + Into<ColorHSLA>
{
    const TRANSPARENT : Self;

    const BLACK : Self;
    const GRAY  : Self;
    const WHITE : Self;
    
    const RED    : Self; 
    const GREEN  : Self; 
    const BLUE   : Self; 
    
    const CYAN   : Self; 
    const PINK   : Self; 
    const YELLOW : Self;

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
    
    /* 
    fn rgba_from_bytes_slice(rgba : &[u8]) -> Self { Self::rgba_from_bytes(rgba[0], rgba[1], rgba[2], rgba[3]) }
    fn rgba_from_bytes(r : u8, g : u8, b : u8, a : u8) -> Self;

    fn from_rgb_hex(hex: u32) -> Self 
    {
        let bytes: [u8; 4] = hex.to_be_bytes();
        Self::rgba_from_bytes_slice(&bytes)
    }
    fn from_rgba_hex(hex: u32) -> Self 
    {
        let mut bytes: [u8; 4] = hex.to_be_bytes();
        bytes[3] = u8::MAX;
        Self::rgba_from_bytes_slice(&bytes)
    }

    /// Cast to color byte and convert to u32 using : `#RRGGBBAA`
    fn to_rgba_hex(self) -> u32 
    { 
        let ColorRGBAByte { r, g, b, a } = self.to_color_byte(); 
        ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32) 
    }

    /// Cast to color byte and format the color : `#RRGGBBAA`
    fn to_rgba_hex_string(self) -> String
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

    /* 
    // For encoding purpose when saving an image
    fn slice_to_bytes(slice : &[Self]) -> Cow<'_, [u8]>
    {

    }
    */
    */
}