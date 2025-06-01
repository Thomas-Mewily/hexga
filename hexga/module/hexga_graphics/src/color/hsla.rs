use crate::*;


pub type ColorHsla      = ColorHslaFloat;
pub type ColorHslaFloat = ColorHslaOf<float>;
pub type ColorHslaF32   = ColorHslaOf<f32>;
pub type ColorHslaF64   = ColorHslaOf<f64>;

#[repr(C)]
pub struct ColorHslaOf<T>
{
    /// Hue. Color coefficient. Ex:  `0` = red, `0.25` = green, `0.5` = blue, `0.75` = magenta
    pub h : T,
    /// Saturation. Grayscale : `0`, `1`: pure color.
    pub s : T,
    /// Light. Black & White level. `0` = black, `0.5` = pure color, `1` = white
    pub l : T,
    /// Alpha
    pub a : T,
}
impl_fixed_array_like_with_op!(ColorHslaOf, 4);


impl<T> ColorHslaOf<T>
{
    #[inline(always)] pub const fn new(hue : T, saturation : T, lightness : T, alpha : T) -> Self  { Self { h: hue, s: saturation, l : lightness, a: alpha, }}
    pub const fn new_hue(hue : T) -> Self where T: Float { Self::hsl(hue, T::ONE, T::HALF) }
    pub const fn gray(coef : T) -> Self where T: Float { Self::hsl(T::ZERO, T::ZERO, coef) }

    /// H : Color coefficient. Ex:  `0` = red, `0.25` = green, `0.5` = blue, `0.75` = magenta
    ///
    /// S : Grayscale : `0`, `1`: pure color.
    ///
    /// L : Black & White level. `0` = black, `0.5` = pure color, `1` = white
    pub const fn hsla(hue : T, saturation : T, lightness : T, alpha : T) -> Self { Self::new(hue, saturation, lightness, alpha) }

    /// Alpha is at max
    ///
    /// H : Color coefficient. Ex:  `0` = red, `0.25` = green, `0.5` = blue, `0.75` = magenta
    ///
    /// S : Grayscale : `0`, `1`: pure color.
    ///
    /// L : Black & White level. `0` = black, `0.5` = pure color, `1` = white
    pub const fn hsl(hue : T, saturation : T, lightness : T) -> Self where T: Float { Self::hsla(hue, saturation, lightness, T::ONE) }


    pub const fn splat_hsla(hsla : T) -> Self where T: Copy { Self::new(hsla, hsla, hsla, hsla) }
    /// Alpha is at max
    pub const fn splat_hsl(hsl : T) -> Self where T: Copy + RangeDefault { Self::splat_hsl_with_a(hsl, T::RANGE_MAX) }
    pub const fn splat_hsl_with_a(hsl : T, a : T) -> Self where T: Copy { Self::new(hsl, hsl, hsl, a) }


    pub fn hsla_ref(&    self) -> &    [T; 4] { self.as_array() }
    pub fn hsla_mut(&mut self) -> &mut [T; 4] { self.as_array_mut() }

    pub fn hsl_ref(&self) -> &[T; 3] {
        // SAFETY: ColorHSLAOf<T> always has at least 3 fields, and they are laid out contiguously.
        unsafe { &*(self.as_array().as_ptr() as *const [T; 3]) }
    }
    pub fn hsl_mut(&mut self) -> &mut [T; 3] {
        // SAFETY: ColorHSLAOf<T> always has at least 3 fields, and they are laid out contiguously.
        unsafe { &mut *(self.as_array_mut().as_mut_ptr() as *mut [T; 3]) }
    }

    /// Hue
    pub const H_INDEX : usize = 0;
    /// Hue
    pub fn h(&self) -> T where T: Copy { self.h }
    /// Hue
    pub fn with_h(mut self, h : T) -> Self where T: Copy { self.set_h(h); self }
    /// Hue
    pub fn set_h(&mut self, h : T) -> &mut Self { self.h = h; self }
    /// Hue
    pub fn replace_h(mut self, h : T) -> T { self.replace_or_panic(Self::H_INDEX, h) }

    /// Saturation
    pub const S_INDEX : usize = 1;
    /// Saturation
    pub fn s(&self) -> T where T: Copy { self.s }
    /// Saturation
    pub fn with_s(mut self, s : T) -> Self where T: Copy { self.set_s(s); self }
    /// Saturation
    pub fn set_s(&mut self, s : T) -> &mut Self { self.s = s; self }
    /// Saturation
    pub fn replace_s(mut self, s : T) -> T { self.replace_or_panic(Self::S_INDEX, s) }

    /// Light
    pub const L_INDEX : usize = 2;
    /// Light
    pub fn l(&self) -> T where T: Copy { self.l }
    /// Light
    pub fn with_l(mut self, l : T) -> Self where T: Copy { self.set_l(l); self }
    /// Light
    pub fn set_l(&mut self, l : T) -> &mut Self { self.l = l; self }
    /// Light
    pub fn replace_l(mut self, l : T) -> T { self.replace_or_panic(Self::L_INDEX, l) }

    /// Alpha
    pub const A_INDEX : usize = 3;
    /// Alpha
    pub fn a(&self) -> T where T: Copy { self.a }
    /// Alpha
    pub fn with_a(mut self, a : T) -> Self where T: Copy { self.set_a(a); self }
    /// Alpha
    pub fn set_a(&mut self, a : T) -> &mut Self { self.a = a; self }
    /// Alpha
    pub fn replace_a(mut self, a : T) -> T { self.replace_or_panic(Self::A_INDEX, a) }
}

impl<T> From<(T,T,T,T,)> for ColorHslaOf<T> { fn from(value: (T,T,T,T,)) -> Self { ColorHslaOf::hsla(value.0, value.1, value.2, value.3) }}
impl<T> From<ColorHslaOf<T>> for (T,T,T,T,) { fn from(value: ColorHslaOf<T>) -> Self { (value.h, value.s, value.l, value.a) }}

impl<T> From<(T,T,T,)> for ColorHslaOf<T> where T: Float { fn from(value: (T,T,T,)) -> Self { ColorHslaOf::hsl(value.0, value.1, value.2) }}
impl<T> From<ColorHslaOf<T>> for (T,T,T,) { fn from(value: ColorHslaOf<T>) -> Self { (value.h, value.s, value.l) }}

impl<T> From<[T; 3]> for ColorHslaOf<T> where T: Float { fn from(value: [T; 3]) -> Self { let [r,g,b] = value; ColorHslaOf::hsl(r,g,b) }}
impl<T> From<ColorHslaOf<T>> for [T; 3] { fn from(value: ColorHslaOf<T>) -> Self { [value.h, value.s, value.l] }}

impl<T> From<Vector4<T>> for ColorHslaOf<T> { fn from(value: Vector4<T>) -> Self { let [h,s,l,a] = value.array; ColorHslaOf::hsla(h,s,l,a) }}
impl<T> From<ColorHslaOf<T>> for Vector4<T> { fn from(value: ColorHslaOf<T>) -> Self { let [x,y,z,w] = value.into(); vector4(x,y,z,w) }}

impl<T> From<Vector3<T>> for ColorHslaOf<T> where T: Float { fn from(value: Vector3<T>) -> Self { let [h,s,l] = value.array; ColorHslaOf::hsl(h,s,l) }}
impl<T> From<ColorHslaOf<T>> for Vector3<T> { fn from(value: ColorHslaOf<T>) -> Self { let [x,y,z,_] = value.into(); vector3(x,y,z) }}

impl<C:Float> Default for ColorHslaOf<C>
{
    fn default() -> Self { Self::hsla(zero(), zero(), one(), one()) }
}

impl<T> IColor<T> for ColorHslaOf<T> where T: Float
{
    const TRANSPARENT : Self = Self::hsla(T::ZERO, T::ZERO, T::ZERO, T::ZERO);

    const BLACK : Self = Self::hsl(T::ZERO,T::ZERO,T::ZERO);
    const GRAY  : Self = Self::hsl(T::ZERO,T::ZERO,T::HALF);
    const WHITE : Self = Self::hsl(T::ZERO,T::ZERO,T::ONE);

    const RED    : Self = Self::hsl(T::ZERO, T::ONE,T::HALF);
    const GREEN  : Self = Self::hsl(T::COLOR_120_DIV_360,T::ONE,T::HALF);
    const BLUE   : Self = Self::hsl(T::COLOR_240_DIV_360,T::ONE,T::HALF);

    const CYAN   : Self = Self::hsl(T::COLOR_180_DIV_360,T::ONE,T::HALF);
    const MAGENTA: Self = Self::hsl(T::COLOR_300_DIV_360,T::ONE,T::HALF);
    const YELLOW : Self = Self::hsl(T::COLOR_60_DIV_360 ,T::ONE,T::HALF);

    const SPRING : Self = Self::hsl(T::COLOR_150_DIV_360, T::ONE, T::HALF);
    const AZURE  : Self = Self::hsl(T::COLOR_210_DIV_360, T::ONE, T::HALF);
    const VIOLET : Self = Self::hsl(T::COLOR_270_DIV_360, T::ONE, T::HALF);
    const ROSE   : Self = Self::hsl(T::COLOR_330_DIV_360, T::ONE, T::HALF);
    const ORANGE : Self = Self::hsl(T::COLOR_30_DIV_360, T::ONE, T::HALF);
    const LIME   : Self = Self::hsl(T::COLOR_90_DIV_360, T::ONE, T::HALF);
    const CANARY : Self = Self::hsl(T::COLOR_60_DIV_360, T::ONE, T::COLOR_270_DIV_360);
    const PINK   : Self = Self::hsl(T::COLOR_300_DIV_360, T::ONE, T::COLOR_270_DIV_360);
    const GLACE  : Self = Self::hsl(T::COLOR_180_DIV_360, T::ONE, T::COLOR_270_DIV_360);


    fn to_rgba_of<T2>(self) -> ColorRgbaOf<T2> where T2 : Primitive, T2 : CastRangeFrom<T>
    {
        // Thank to MacroQuad, the following code was copied and edited from the MacroQuad crate
        let r;
        let g;
        let b;

        if self.s == T::ZERO {  r = self.l; g = self.l; b = self.l; }
        else {
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

        ColorRgbaOf::from_array([r, g, b, self.a].map(|v| T2::cast_range_from(v)))
    }

    fn to_hsla_of<T2>(self) -> ColorHslaOf<T2> where T2 : Float + CastRangeFrom<T>
    {
        ColorHslaOf::from_array(self.to_array4().map(|v| T2::cast_range_from(v)))
    }
}



impl<T> ToColorComposite for ColorHslaOf<T> where T: Float
{
    type RgbaF32 = ColorRgbaOf<f32>;
    fn to_rgba_f32(&self) -> Self::RgbaF32 { self.to_rgba_of() }

    type RgbaF64 = ColorRgbaOf<f64>;
    fn to_rgba_f64(&self) -> Self::RgbaF64 { self.to_rgba_of() }

    type RgbaU8 = ColorRgbaU8;
    fn to_rgba_u8(&self) -> Self::RgbaU8 { self.to_rgba_of() }

    type RgbaU16 = ColorRgbaU16;
    fn to_rgba_u16(&self) -> Self::RgbaU16 { self.to_rgba_of() }

    type RgbaBool = ColorRgbaMask;
    fn to_rgba_bool(&self) -> Self::RgbaBool { self.to_rgba_of() }

    type HslaF32 = ColorHslaF32;
    fn to_hsla_f32(&self) -> Self::HslaF32 { self.to_hsla_of() }

    type HslaF64 = ColorHslaF64;
    fn to_hsla_f64(&self) -> Self::HslaF64 { self.to_hsla_of() }

    const COLOR_INSIDE : ColorKind = match std::mem::size_of::<T>()
        {
            4 => ColorKind::HslaF32,
            8 => ColorKind::HslaF64,
            _ => ColorKind::Unknow,
        };
}



impl<T,Dest> CastIntoComposite<Dest> for ColorHslaOf<T> where T : CastIntoComposite<Dest>
{
    type Output=ColorHslaOf<T::Output>;
    fn cast_into_composite(self) -> Self::Output { self.map(|v| v.cast_into_composite()) }
}
impl<T,Dest> CastRangeIntoComposite<Dest> for ColorHslaOf<T> where T : CastRangeIntoComposite<Dest>
{
    type Output=ColorHslaOf<T::Output>;
    fn cast_range_into_composite(self) -> Self::Output { self.map(|v| v.cast_range_into_composite()) }
}
impl<T> ToUnsigned for ColorHslaOf<T> where T : ToUnsigned
{
    type Output=ColorHslaOf<T::Output>;
    fn to_unsigned(self) -> Self::Output { self.map(|v| v.to_unsigned()) }
}
impl<T> ToSigned for ColorHslaOf<T> where T : ToSigned
{
    type Output=ColorHslaOf<T::Output>;
    fn to_signed(self) -> Self::Output { self.map(|v| v.to_signed()) }
}
impl<T> Abs for ColorHslaOf<T> where T : Abs
{
    type Output=ColorHslaOf<T::Output>;
    fn abs(self) -> Self::Output { self.map(|v| v.abs()) }
}
map_on_constant!
(
    (($trait_name: tt, $constant_name: tt)) =>
    {
        impl<T> $trait_name for ColorHslaOf<T> where T: $trait_name + Copy { const $constant_name : Self = Self::splat_hsla(T::$constant_name); }
    }
);