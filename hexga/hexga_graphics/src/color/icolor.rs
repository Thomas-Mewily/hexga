use std::any::Any;
use std::borrow::Cow;
use std::mem;
use std::slice;

use super::*;

pub trait ToColor<P> : Sized where P: Primitive
{
    type ToRgba<R> : ToColor<R> where R: Primitive;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<P>;


    type ToHsla<R> : ToColor<R> where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<P>;


    fn to_hsla(self) -> Self::ToHsla<float> where float: CastRangeFrom<P> { self.to_hsla_of() }
    fn to_hsla_f32(self) -> Self::ToHsla<f32> where f32: CastRangeFrom<P> { self.to_hsla_of() }
    fn to_hsla_f64(self) -> Self::ToHsla<f64> where f64: CastRangeFrom<P> { self.to_hsla_of() }

    fn to_rgba(self) -> Self::ToRgba<float> where float: CastRangeFrom<P> { self.to_rgba_of() }
    fn to_rgba_f32(self) -> Self::ToRgba<f32> where f32: CastRangeFrom<P> { self.to_rgba_of() }
    fn to_rgba_f64(self) -> Self::ToRgba<f64> where f64: CastRangeFrom<P> { self.to_rgba_of() }
    fn to_rgba_u8(self) -> Self::ToRgba<u8> where u8: CastRangeFrom<P> { self.to_rgba_of() }
    fn to_rgba_u16(self) -> Self::ToRgba<u16> where u16: CastRangeFrom<P> { self.to_rgba_of() }

    fn to_color(self) -> Self::ToRgba<float> where float: CastRangeFrom<P> { self.to_rgba_of() }
    fn to_color_float(self) -> Self::ToRgba<float> where float: CastRangeFrom<P> { self.to_rgba_of() }
    fn to_color_f32(self) -> Self::ToRgba<f32> where f32: CastRangeFrom<P> { self.to_rgba_of() }
    fn to_color_f64(self) -> Self::ToRgba<f64> where f64: CastRangeFrom<P> { self.to_rgba_of() }
    fn to_color_u8(self) -> Self::ToRgba<u8> where u8: CastRangeFrom<P> { self.to_rgba_of() }
    fn to_color_u16(self) -> Self::ToRgba<u16> where u16: CastRangeFrom<P> { self.to_rgba_of() }
}



// Hack, waiting for specialization to impl ToColor for Rgba (impl Map) and Hsla (impl Map) and for every type T that also impl Map.
impl<P> ToColor<P> for RgbaOf<P> where P: Primitive
{
    type ToRgba<R> = RgbaOf<R> where R: Primitive;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<P> { self.to_rgba_of() }

    type ToHsla<R>  = HslaOf<R> where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<P> { self.to_hsla_of() }
}
impl<T> ToColor<T> for HslaOf<T> where T: Float
{
    type ToRgba<R> = RgbaOf<R> where R: Primitive;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<T> { self.to_rgba_of() }

    type ToHsla<R>  = HslaOf<R> where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<T> { self.to_hsla_of() }
}

impl<T, const N:usize,P> ToColor<P> for [T;N] where P: Primitive, T: ToColor<P>
{
    type ToRgba<R> = [T::ToRgba<R>;N] where R: Primitive;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<P> { self.map(ToColor::to_rgba_of) }

    type ToHsla<R>  = [T::ToHsla<R>;N] where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<P> { self.map(ToColor::to_hsla_of) }
}
impl<T,P> ToColor<P> for Vec<T> where P: Primitive, T: ToColor<P>
{
    type ToRgba<R> = Vec<T::ToRgba<R>> where R: Primitive;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<P> { self.map(ToColor::to_rgba_of) }

    type ToHsla<R>  = Vec<T::ToHsla<R>> where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<P> { self.map(ToColor::to_hsla_of) }
}
impl<P, T, Idx, const N : usize> ToColor<P> for GridBase<T, Idx, N> where Idx : Integer, T: ToColor<P>, P: Primitive
{
    type ToRgba<R> = GridBase<T::ToRgba<R>,Idx,N> where R: Primitive;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<P> { self.map(ToColor::to_rgba_of) }

    type ToHsla<R>  = GridBase<T::ToHsla<R>,Idx,N> where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<P> { self.map(ToColor::to_hsla_of) }
}
impl<T, C, Idx> ToColor<T> for ImageBase<C, Idx> where Idx : Integer, C: ToColor<T>, T: Primitive
{
    type ToRgba<R> = ImageBase<C::ToRgba<R>,Idx> where R: Primitive;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<T> { self.map(ToColor::to_rgba_of) }

    type ToHsla<R>  = ImageBase<C::ToHsla<R>,Idx> where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<T> { self.map(ToColor::to_hsla_of) }
}
impl<'a, P, T, Idx, const N : usize> ToColor<P> for GridView<'a, GridBase<T,Idx,N>, T, Idx, N> where Idx : Integer, T: ToColor<P> + Copy, P: Primitive
{
    type ToRgba<R> = GridBase<T::ToRgba<R>,Idx,N> where R: Primitive;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<P> { self.transform(|v| v.to_rgba_of()) }

    type ToHsla<R>  = GridBase<T::ToHsla<R>,Idx,N> where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<P> { self.transform(|v| v.to_hsla_of()) }
}
impl<'a, P, T, Idx, const N : usize> ToColor<P> for GridViewMut<'a, GridBase<T,Idx,N>, T, Idx, N> where Idx : Integer, T: ToColor<P> + Copy, P: Primitive
{
    type ToRgba<R> = GridBase<T::ToRgba<R>,Idx,N> where R: Primitive;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<P> { self.transform(|v| v.to_rgba_of()) }

    type ToHsla<R>  = GridBase<T::ToHsla<R>,Idx,N> where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<P> { self.transform(|v| v.to_hsla_of()) }
}


/*
impl<C,T> ToColor<T> for C where C:Map, C::Item: ToColor<T>, T: Primitive
{
    type ToRgba<R> = C::WithType<<C::Item as ToColor<T>>::ToRgba<R>> where R: Primitive;
    fn to_rgba_of<R>(self) -> Self::ToRgba<R> where R: Primitive + CastRangeFrom<T>
    {
        /*
        if let Some(rgba) = (&self as &dyn Any).downcast_ref::<&RgbaOf<T>>()
        {
            let rgba = **rgba;
            let casted : RgbaOf::<R> = rgba.cast_range_into();
            return unsafe { std::mem::transmute(casted) };// as Self::ToRgba<R>;
        }
        */
        self.map(ToColor::to_rgba_of)
    }

    type ToHsla<R> = C::WithType<<C::Item as ToColor<T>>::ToHsla<R>> where R: Float;
    fn to_hsla_of<R>(self) -> Self::ToHsla<R> where R: Float + CastRangeFrom<T> {
        self.map(ToColor::to_hsla_of)
    }
}
*/

/*
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
*/

/// Constant color name are based on <https://colornames.org/>
///
/// (+-1 u8 unit per channel, otherwise `#FF7F00` should be named `Orange Juice` and not `Orange`, because `Orange` is `#FF7F00`)
pub trait IColor : Sized + ToColor<Self::Component> //+ ToColor<Self::Component> //+ ToRgbaComposite<Output<Self::Component> = RgbaOf::<Self::Component>>
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

    fn to_rgba_of<R>(self) -> RgbaOf<R> where R : Primitive + CastRangeFrom<Self::Component>;
    fn to_hsla_of<R>(self) -> HslaOf<R> where R : Float + CastRangeFrom<Self::Component>;

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


impl<S, T, const N : usize> ArrayToColor<T,N> for S where S : Array<T,N> {}
pub trait ArrayToColor<T, const N : usize> : Array<T,N>
{
    fn to_rgba(self) -> RgbaOf<T> where T: Default { RgbaOf::from_array(self.to_array_resized()) }
    fn to_hlsa(self) -> HslaOf<T> where T: Default { HslaOf::from_array(self.to_array_resized()) }
}
