use std::borrow::Cow;
use std::mem;
use std::slice;

use super::*;


/*
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
    /// Pixel is 32-bit float HSL with an alpha channel. Casted to RGBAU8 if not supported
    HslaF32,
    /// Pixel is 64-bit float HSL with an alpha channel. Casted to RGBAU8 if not supported
    HslaF64,
    // Casted to RGBAU8 if not supported
    Unknow,
}
    */

pub trait ToRgbaFloatComposite
{
    type RgbaFloat;
    fn to_rgba_float() -> Self::RgbaFloat;

    type RgbaF32;
    fn to_rgba_f32() -> Self::RgbaF32;

    type RgbaF64;
    fn to_rgba_f64() -> Self::RgbaF64;
}

/*
pub trait ToColorComposite : CompositeGeneric + Sized
{
    #[cfg(any(
        feature = "float_are_32_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "32")
    ))]
    fn to_color(self) -> Self::WithType<Color> { self.cast_range_into() }
    #[cfg(any(
        feature = "float_are_64_bits",
        all(feature = "float_are_size_bits", target_pointer_width = "64")
    ))]
    fn to_color(&self) -> Self::RgbaF64 { self.to_color_rgba_float() }

    fn to_color_u8(&self) -> Self::RgbaU8 { self.to_rgba_u8() }

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
        RgbaU16 = ColorRgbaU16,
        RgbaBool = ColorRgbaBool,
    > + Copy {}
impl<T> ToColor for T where T: ToColorComposite
    <
        RgbaF32 = ColorRgbaOf<f32>, RgbaF64 = ColorRgbaOf<f64>,
        HslaF32 = ColorHslaF32, HslaF64 = ColorHslaF64,
        RgbaU8 = ColorRgbaU8,
        RgbaU16 = ColorRgbaU16,
        RgbaBool = ColorRgbaBool,
    > + Copy {}

pub trait ToRgbaF32
{
    fn to_rgba_f32(self) -> ColorRgbaF32;
}
impl<T> ToRgbaF32 for T where ColorRgbaF32: CastFrom<T>
{
    fn to_rgba_f32(self) -> ColorRgbaF32 { self.cast_into() }
}
impl<T> ToRgbaF32 for ColorHslaOf<T> where T : Primitive, f32 : CastRangeFrom<T>
{
    fn to_rgba_f32(self) -> ColorRgbaF32 {
        self.cast_into()
    }
}
*/


// ToColor : CastRange
// Rgba => Any Color Type
// Image Color => Any Image Color Type

/*
pub trait ToRgbaComposite
{
    type Output<R>;
    fn to_rgba<T:Primitive>(&self) -> Self::Output<T>;
}
*/


pub trait ToColor<T> where T: Primitive
{
    type ToRgba<R> : ToColor<R> where R: Primitive;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<T>;

    type ToHsla<R> : ToColor<R> where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<T>;
}

impl<C,T> ToColor<T> for C where C:Map, C::Item: ToColor<T> + Primitive
{
    type ToRgba<R> = C::WithType<<C::Item as ToColor<T>>::ToRgba<R>> where R: Primitive;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<T> {
        self.map(ToColor::to_rgba_of)
    }

    type ToHsla<R> = C::WithType<<C::Item as ToColor<T>>::ToHsla<R>> where R: Primitive;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<T> {
        self.map(ToColor::to_hsla_of)
    }
}

impl<T> ToColor<T> for RgbaOf<T> where T: Primitive
{
    type ToRgba<R> = RgbaOf<R> where R: Primitive ;

    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<T> { self.cast_range_into() }

    type ToHsla<R> = HslaOf<R> where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<T> {
        // Thank to MacroQuad, the following code was copied and edited the code from the MacroQuad crate
        let [r, g, b, a] = self.to_array4().map(|v| R::cast_range_from(v));
        let f = [r, g, b];

        let max = *f.max_element();
        let min = *f.min_element();

        // Luminosity is the average of the max and min rgb color intensities.
        let l= (max + min) / R::TWO;

        // Saturation
        let delta = max - min;
        if delta.is_zero() { return HslaOf::new(R::ZERO, R::ZERO, l, a); }

        // it's not gray
        let s = if l < R::HALF
        {
            delta / (max + min)
        } else {
            delta / (R::TWO - max - min)
        };

        // Hue
        let r2 = (((max - r) / R::SIX) + (delta / R::TWO)) / delta;
        let g2 = (((max - g) / R::SIX) + (delta / R::TWO)) / delta;
        let b2 = (((max - b) / R::SIX) + (delta / R::TWO)) / delta;

        let mut h = match max {
            x if x == r => b2 - g2,
            x if x == g => (R::ONE / R::THREE) + r2 - b2,
            _ => (R::TWO / R::THREE) + g2 - r2,
        };

        // Fix wraparounds
        if h < R::ZERO { h += R::ONE; } else if h > R::ONE { h -= R::ONE; }

        HslaOf::new(h, s, l, a)
    }
}

impl<T> ToColor<T> for HslaOf<T> where T: Float
{
    type ToRgba<R> = RgbaOf<R> where R: Primitive ;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<T> {
        // Thank to MacroQuad, the following code was copied and edited from the MacroQuad crate
        let r;
        let g;
        let b;

        if self.s == T::ZERO {  r = self.l; g = self.l; b = self.l; }
        else
        {
            fn hue_to_rgb<T>(p: T, q: T, mut t: T) -> T where T: Float {
                if t < T::ZERO { t += T::ONE }
                if t > T::ONE { t -= T::ONE }
                if t < T::ONE / T::SIX { return p + (q - p) * T::SIX * t; }
                if t < T::ONE / T::TWO { return q; }
                if t < T::TWO / T::THREE { return p + (q - p) * (T::TWO / T::THREE - t) * T::SIX; }
                p
            }

            let q = if self.l < T::HALF {
                self.l * (T::ONE + self.s)
            } else {
                self.l + self.s - self.l * self.s
            };
            let p = T::TWO * self.l - q;
            r = hue_to_rgb(p, q, self.h + T::ONE / T::THREE);
            g = hue_to_rgb(p, q, self.h);
            b = hue_to_rgb(p, q, self.h - T::ONE / T::THREE);
        }
        RgbaOf::from_array([r, g, b, self.a].cast_range_into())
    }

    type ToHsla<R> = HslaOf<R> where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<T> { self.cast_range_into() }
}

/// Constant color name are based on <https://colornames.org/>
///
/// (+-1 u8 unit per channel, otherwise `#FF7F00` should be named `Orange Juice` and not `Orange`, because `Orange` is `#FF7F00`)
pub trait IColor : Sized + ToColor<Self::Component> //+ ToRgbaComposite<Output<Self::Component> = RgbaOf::<Self::Component>>
{
    type Component : Primitive;
    const TRANSPARENT : Self;

    /// #000000
    ///
    /// ⬛ : ⬛⬛⬛
    const BLACK : Self;
    /// #777777
    ///
    /// If you preferer the name `Gray`, just do :
    ///
    /// ```
    /// use hexga_graphics::prelude::*;
    /// use hexga_math::prelude::*;
    ///
    /// pub trait IPreferGray
    /// {
    ///    const GRAY : Self;
    /// }
    /// impl<C> IPreferGray for C where C: IColor
    /// {
    ///    const GRAY : Self = Self::GREY;
    /// }
    /// ```
    const GREY  : Self;
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

    fn rgb_from_hex(rgb: u32) -> RgbaOf<Self::Component> where Self::Component: CastRangeFrom<u8>
    {
        let [r,g,b,_] = rgb.to_be_bytes();
        Self::rgb_from_u8(r,g,b)
    }
    fn rgba_from_hex(rgba: u32) -> RgbaOf<Self::Component> where Self::Component: CastRangeFrom<u8>
    {
        let [r,g,b,a] = rgba.to_be_bytes();
        Self::rgba_from_u8(r,g,b,a)
    }
    fn rgba_from_array(rgba : [u8;4]) -> RgbaOf<Self::Component> where Self::Component: CastRangeFrom<u8>
    {
        let [r,g,b,a] = rgba;
        Self::rgba_from_u8(r,g,b,a)
    }
    fn rgb_from_array(rgba : [u8;3]) -> RgbaOf<Self::Component> where Self::Component: CastRangeFrom<u8>
    {
        let [r,g,b] = rgba;
        Self::rgb_from_u8(r,g,b)
    }

    fn rgba_from_u8(r : u8, g : u8, b : u8, a : u8) -> RgbaOf<Self::Component> where Self::Component: CastRangeFrom<u8>
    {
        RgbaU8::rgba(r, g, b, a).to_rgba_of()
    }
    fn rgb_from_u8(r : u8, g : u8, b : u8) -> RgbaOf<Self::Component> where Self::Component: CastRangeFrom<u8>
    {
        RgbaU8::rgb(r, g, b).to_rgba_of()
    }

    /*
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
    */
}


impl<S, T, const N : usize> ColorArrayExtension<T,N> for S where S : Array<T,N> {}
pub trait ColorArrayExtension<T, const N : usize> : Array<T,N>
{
    fn to_rgba(self) -> RgbaOf<T> where T: Default { RgbaOf::from_array(self.to_array_resized()) }
    fn to_hlsa(self) -> HslaOf<T> where T: Default { HslaOf::from_array(self.to_array_resized()) }
}
