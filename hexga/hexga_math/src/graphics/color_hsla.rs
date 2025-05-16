use crate::*;


pub type ColorHSLA     = ColorHSLAOf<float>;

#[repr(C)]
pub struct ColorHSLAOf<T>
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
impl_fixed_array_like_with_op!(ColorHSLAOf, 4);


impl<T> ColorHSLAOf<T>
{
    #[inline(always)] pub const fn new(hue : T, saturation : T, lightness : T, alpha : T) -> Self  { Self { h: hue, s: saturation, l : lightness, a: alpha, }}
    pub const fn new_hue(hue : T) -> Self where T : Float { Self::hsl(hue, T::ONE, T::HALF) }
    pub const fn gray(coef : T) -> Self where T : Float { Self::hsl(T::ZERO, T::ZERO, coef) }

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
    pub const fn hsl(hue : T, saturation : T, lightness : T) -> Self where T : Float { Self::hsla(hue, saturation, lightness, T::ONE) }

    /// Hue
    pub const H_INDEX : usize = 0;
    /// Hue
    pub fn h(&self) -> T where T : Copy { self.h }
    /// Hue
    pub fn with_h(mut self, h : T) -> Self where T : Copy { self.set_h(h); self }
    /// Hue
    pub fn set_h(&mut self, h : T) -> &mut Self { self.h = h; self }
    /// Hue
    pub fn replace_h(mut self, h : T) -> T { self.replace_or_panic(Self::H_INDEX, h) }

    /// Saturation
    pub const S_INDEX : usize = 1;
    /// Saturation
    pub fn s(&self) -> T where T : Copy { self.s }
    /// Saturation
    pub fn with_s(mut self, s : T) -> Self where T : Copy { self.set_s(s); self }
    /// Saturation
    pub fn set_s(&mut self, s : T) -> &mut Self { self.s = s; self }
    /// Saturation
    pub fn replace_s(mut self, s : T) -> T { self.replace_or_panic(Self::S_INDEX, s) }

    /// Light
    pub const L_INDEX : usize = 2;
    /// Light
    pub fn l(&self) -> T where T : Copy { self.l }
    /// Light
    pub fn with_l(mut self, l : T) -> Self where T : Copy { self.set_l(l); self }
    /// Light
    pub fn set_l(&mut self, l : T) -> &mut Self { self.l = l; self }
    /// Light
    pub fn replace_l(mut self, l : T) -> T { self.replace_or_panic(Self::L_INDEX, l) }

    /// Alpha
    pub const A_INDEX : usize = 3;
    /// Alpha
    pub fn a(&self) -> T where T : Copy { self.a }
    /// Alpha
    pub fn with_a(mut self, a : T) -> Self where T : Copy { self.set_a(a); self }
    /// Alpha
    pub fn set_a(&mut self, a : T) -> &mut Self { self.a = a; self }
    /// Alpha
    pub fn replace_a(mut self, a : T) -> T { self.replace_or_panic(Self::A_INDEX, a) }
 
    pub fn unpack_hsl(self) -> (T, T, T) { (self.h, self.s, self.l) }
    pub fn unpack_hsla(self) -> (T, T, T, T) { (self.h, self.s, self.l, self.a) }
}

impl<T> From<(T,T,T,T,)> for ColorHSLAOf<T> { fn from(value: (T,T,T,T,)) -> Self { ColorHSLAOf::hsla(value.0, value.1, value.2, value.3) }}
impl<T> From<ColorHSLAOf<T>> for (T,T,T,T,) { fn from(value: ColorHSLAOf<T>) -> Self { (value.h, value.s, value.l, value.a) }}

impl<T> From<(T,T,T,)> for ColorHSLAOf<T> where T : Float { fn from(value: (T,T,T,)) -> Self { ColorHSLAOf::hsl(value.0, value.1, value.2) }}
impl<T> From<ColorHSLAOf<T>> for (T,T,T,) { fn from(value: ColorHSLAOf<T>) -> Self { (value.h, value.s, value.l) }}

impl<T> From<[T; 3]> for ColorHSLAOf<T> where T : Float { fn from(value: [T; 3]) -> Self { let [r,g,b] = value; ColorHSLAOf::hsl(r,g,b) }}
impl<T> From<ColorHSLAOf<T>> for [T; 3] { fn from(value: ColorHSLAOf<T>) -> Self { [value.h, value.s, value.l] }}

impl<T> From<Vector4<T>> for ColorHSLAOf<T> { fn from(value: Vector4<T>) -> Self { let [h,s,l,a] = value.array; ColorHSLAOf::hsla(h,s,l,a) }}
impl<T> From<ColorHSLAOf<T>> for Vector4<T> { fn from(value: ColorHSLAOf<T>) -> Self { let [x,y,z,w] = value.into(); vector4(x,y,z,w) }}

impl<T> From<Vector3<T>> for ColorHSLAOf<T> where T : Float { fn from(value: Vector3<T>) -> Self { let [h,s,l] = value.array; ColorHSLAOf::hsl(h,s,l) }}
impl<T> From<ColorHSLAOf<T>> for Vector3<T> { fn from(value: ColorHSLAOf<T>) -> Self { let [x,y,z,_] = value.into(); vector3(x,y,z) }}

impl From<Color> for ColorHSLA { fn from(value: Color) -> Self { value.to_color_hsla() }}
impl From<ColorRGBAByte> for ColorHSLA { fn from(value: ColorRGBAByte) -> Self { value.to_color_hsla() }}

impl<C:Float> Default for ColorHSLAOf<C>
{
    fn default() -> Self { Self::hsla(zero(), zero(), one(), one()) }
}

impl IColor for ColorHSLA
{
    const TRANSPARENT : Self = Self::hsla(float::ZERO, float::ZERO, float::ZERO, float::ZERO);
    
    const BLACK : Self = Self { h: float::ZERO, s: float::ZERO, l: float::ZERO, a: float::ONE };
    const GRAY  : Self = Self { h: float::ZERO, s: float::ZERO, l: float::HALF, a: float::ONE };
    const WHITE : Self = Self { h: float::ZERO, s: float::ZERO, l: float::ONE , a: float::ONE };

    const RED    : Self = Self { h: float::ZERO,              s: float::ONE, l: float::HALF, a: float::ONE };
    const GREEN  : Self = Self { h: float::COLOR_120_DIV_360, s: float::ONE, l: float::HALF, a: float::ONE };
    const BLUE   : Self = Self { h: float::COLOR_240_DIV_360, s: float::ONE, l: float::HALF, a: float::ONE };
    
    const CYAN   : Self = Self { h: float::COLOR_180_DIV_360, s: float::ONE, l: float::HALF, a: float::ONE };
    const PINK   : Self = Self { h: float::COLOR_300_DIV_360, s: float::ONE, l: float::HALF, a: float::ONE };
    const YELLOW : Self = Self { h: float::COLOR_60_DIV_360 , s: float::ONE, l: float::HALF, a: float::ONE };
    
    fn rgba_from_bytes(r : u8, g : u8, b : u8, a : u8) -> Self { ColorRGBAByte::rgba(r,g,b,a).to_color_hsla() }
}

impl ToColorRep for ColorHSLA
{
    type ColorRGBAFloat=ColorRGBA;
    fn to_color_float(&self) -> Self::ColorRGBAFloat {
        // Thank to MacroQuad, the following code was copied and edited from the MacroQuad crate
        let r;
        let g;
        let b;
    
        if self.s == 0.0 {  r = self.l; g = self.l; b = self.l; }
        else {
            fn hue_to_rgb(p: Coef, q: Coef, mut t: Coef) -> Coef {
                if t < 0.0 { t += 1.0 }
                if t > 1.0 { t -= 1.0 }
                if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; }
                if t < 1.0 / 2.0 { return q; }
                if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
                p
            }
    
            let q = if self.l < 0.5 {
                self.l * (1.0 + self.s)
            } else {
                self.l + self.s - self.l * self.s
            };
            let p = 2.0 * self.l - q;
            r = hue_to_rgb(p, q, self.h + 1.0 / 3.0);
            g = hue_to_rgb(p, q, self.h);
            b = hue_to_rgb(p, q, self.h - 1.0 / 3.0);
        }
    
        Color::new(r, g, b, self.a)
    }

    type ColorHSLA=ColorHSLA;
    fn to_color_hsla(&self) -> Self::ColorHSLA {
        *self
    }

    type ColorRGBAByte=ColorRGBAByte;
    fn to_color_byte(&self) -> Self::ColorRGBAByte {
        self.to_color_float().to_color_byte()
    }
}