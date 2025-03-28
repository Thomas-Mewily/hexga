use crate::*;

pub type Color = ColorRGBA;

pub type ColorRGBA = ColorRGBAOf<float>;

pub type ColorRGBAByte = ColorRGBAOf<u8>;
pub type ColorByte = ColorRGBAByte;

#[repr(C)]
pub struct ColorRGBAOf<T>
{
    /// Red
    pub r : T,
    /// Green
    pub g : T,
    /// Blue
    pub b : T,
    /// Alpha
    pub a : T,
}
impl_fixed_array_like_with_op!(ColorRGBAOf, 4);


impl From<u32> for ColorRGBAByte { fn from(value: u32) -> Self { Self::from_rgba_hex(value) } }

#[allow(dead_code)]
impl<T> ColorRGBAOf<T>
{
    pub const fn new(red : T, green : T, blue : T, alpha : T) -> Self { Self { r:red, g:green, b:blue, a:alpha }}

    pub const fn rgba(red : T, green : T, blue : T, alpha : T) -> Self { Self::new(red, green, blue, alpha) }
    pub const fn rgb (red : T, green : T, blue : T) -> Self where T : DefaultRange { Self::rgba(red, green, blue, T::MAX_RANGE) }
    pub const fn gray(rgb : T) -> Self where T : DefaultRange + Copy { Self::rgb(rgb, rgb, rgb) }

    pub fn rgba_ref(&    self) -> &    [T; 4] { self.as_array() }
    pub fn rgba_mut(&mut self) -> &mut [T; 4] { self.as_array_mut() }

    pub fn rgb_ref (&    self) -> &    [T; 3] { self.rgba_ref()[0..3].try_into().unwrap() }
    pub fn rgb_mut (&mut self) -> &mut [T; 3] 
    {
        let ptr = self.rgba_mut().as_mut_ptr();
        unsafe {
            &mut *(ptr as *mut [T; 3])
        } 
    }

    pub(crate) const fn const_splat(rgba : T) -> Self where T : Copy { Self::new(rgba, rgba, rgba, rgba) }
    pub(crate) const fn const_splat_rgba(rgba : T) -> Self where T : Copy { Self::const_splat(rgba) }
    pub(crate) const fn const_splat_rgb(rgb : T, a : T) -> Self where T : Copy { Self::new(rgb, rgb, rgb, a) }

    pub fn splat_rgba(rgba : T) -> Self where T : Clone { Self::new(rgba.clone(), rgba.clone(), rgba.clone(), rgba) }
    /// Alpha is at max
    pub fn splat_rgb(rgb : T) -> Self where T : Clone + DefaultRange { Self::splat_rgb_with_a(rgb, T::MAX_RANGE) }
    pub fn splat_rgb_with_a(rgb : T, a : T) -> Self where T : Clone { Self::new(rgb.clone(), rgb.clone(), rgb.clone(), a) }

    /// Red
    pub const R_INDEX : usize = 0;
    /// Red
    pub fn r(&self) -> T where T : Copy { self.r }
    /// Red
    pub fn with_r(mut self, r : T) -> Self where T : Copy { self.set_r(r); self }
    /// Red
    pub fn set_r(&mut self, r : T) -> &mut Self { self.r = r; self }
    /// Red
    pub fn replace_r(mut self, r : T) -> T { self.replace(Self::R_INDEX, r) }

    /// Green
    pub const G_INDEX : usize = 1;
    /// Green
    pub fn g(&self) -> T where T : Copy { self.g }
    /// Green
    pub fn with_g(mut self, g : T) -> Self where T : Copy { self.set_g(g); self }
    /// Green
    pub fn set_g(&mut self, g : T) -> &mut Self { self.g = g; self }
    /// Green
    pub fn replace_g(mut self, g : T) -> T { self.replace(Self::G_INDEX, g) }

    /// Blue
    pub const B_INDEX : usize = 2;
    /// Blue
    pub fn b(&self) -> T where T : Copy { self.b }
    /// Blue
    pub fn with_b(mut self, b : T) -> Self where T : Copy { self.set_b(b); self }
    /// Blue
    pub fn set_b(&mut self, b : T) -> &mut Self { self.b = b; self }
    /// Blue
    pub fn replace_b(mut self, b : T) -> T { self.replace(Self::B_INDEX, b) }

    /// Alpha
    pub const A_INDEX : usize = 3;
    /// Alpha
    pub fn a(&self) -> T where T : Copy { self.a }
    /// Alpha
    pub fn with_a(mut self, a : T) -> Self where T : Copy { self.set_a(a); self }
    /// Alpha
    pub fn set_a(&mut self, a : T) -> &mut Self { self.a = a; self }
    /// Alpha
    pub fn replace_a(mut self, a : T) -> T { self.replace(Self::A_INDEX, a) }

    pub fn unpack_rgb(self) -> (T, T, T) { self.into() }
    pub fn unpack_rgba(self) -> (T, T, T, T) { self.into() }
}

impl<T> From<(T,T,T,T,)> for ColorRGBAOf<T> { fn from(value: (T,T,T,T,)) -> Self { ColorRGBAOf::rgba(value.0, value.1, value.2, value.3) }}
impl<T> From<ColorRGBAOf<T>> for (T,T,T,T,) { fn from(value: ColorRGBAOf<T>) -> Self { (value.r, value.g, value.b, value.a) }}

impl<T> From<(T,T,T,)> for ColorRGBAOf<T> where T : DefaultRange { fn from(value: (T,T,T,)) -> Self { ColorRGBAOf::rgb(value.0, value.1, value.2) }}
impl<T> From<ColorRGBAOf<T>> for (T,T,T,) { fn from(value: ColorRGBAOf<T>) -> Self { (value.r, value.g, value.b) }}

impl<T> From<[T; 3]> for ColorRGBAOf<T> where T : DefaultRange { fn from(value: [T; 3]) -> Self { let [r,g,b] = value; ColorRGBAOf::rgb(r,g,b) }}
impl<T> From<ColorRGBAOf<T>> for [T; 3] { fn from(value: ColorRGBAOf<T>) -> Self { [value.r, value.g, value.b] }}

impl<T> From<Vector4<T>> for ColorRGBAOf<T> { fn from(value: Vector4<T>) -> Self { let [r,g,b,a] = value.array; ColorRGBAOf::rgba(r,g,b,a) }}
impl<T> From<ColorRGBAOf<T>> for Vector4<T> { fn from(value: ColorRGBAOf<T>) -> Self { let [x,y,z,w] = value.into(); vector4(x,y,z,w) }}

impl<T> From<Vector3<T>> for ColorRGBAOf<T> where T : DefaultRange { fn from(value: Vector3<T>) -> Self { let [r,g,b] = value.array; ColorRGBAOf::rgb(r,g,b) }}
impl<T> From<ColorRGBAOf<T>> for Vector3<T> { fn from(value: ColorRGBAOf<T>) -> Self { let [x,y,z,_] = value.into(); vector3(x,y,z) }}

impl From<ColorHSLA> for Color { fn from(value: ColorHSLA) -> Self { value.to_rgba() }}
impl From<ColorHSLA> for ColorByte { fn from(value: ColorHSLAOf<float>) -> Self { value.to_rgba_u8() }}

impl From<ColorByte> for Color { fn from(value: ColorByte) -> Self { value.to_rgba() }}
impl From<Color> for ColorByte { fn from(value: Color) -> Self { value.to_rgba_u8() }}


impl<T> Default for ColorRGBAOf<T> where T : DefaultRange
{
    fn default() -> Self { Self { r: T::MAX_RANGE, g: T::MAX_RANGE, b: T::MAX_RANGE, a: T::MAX_RANGE } }
}

impl<T> IColor for ColorRGBAOf<T> 
    where 
        Self : From<Color> + From<ColorByte> + From<ColorHSLA> + ToFloat<Output = ColorRGBAOf<float>>,
        T : ToCoef<Output=Coef> + FromCoef + ToFloat<Output=float> + DefaultRange + Copy + PartialEq + Default 
    //+ From<Color> + From<ColorByte> + From<ColorHSLA>, 
    //ColorRGBAOf<T> : From<Color> + From<ColorByte> + From<ColorHSLA>,
    //float : CastTo<T>
{
    const TRANSPARENT : Self = Self::rgba(T::MAX_RANGE, T::MAX_RANGE, T::MAX_RANGE, T::MAX_RANGE);

    const BLACK : Self = Self { r: T::MIN_RANGE , g: T::MIN_RANGE , b: T::MIN_RANGE , a: T::MAX_RANGE };
    const GRAY  : Self = Self { r: T::HALF_RANGE, g: T::HALF_RANGE, b: T::HALF_RANGE, a: T::MAX_RANGE };
    const WHITE : Self = Self { r: T::MAX_RANGE , g: T::MAX_RANGE , b: T::MAX_RANGE , a: T::MAX_RANGE };
    
    const RED    : Self = Self::rgba(T::MAX_RANGE, T::MIN_RANGE, T::MIN_RANGE, T::MAX_RANGE);
    const GREEN  : Self = Self::rgba(T::MIN_RANGE, T::MAX_RANGE, T::MIN_RANGE, T::MAX_RANGE);
    const BLUE   : Self = Self::rgba(T::MIN_RANGE, T::MIN_RANGE, T::MAX_RANGE, T::MAX_RANGE);
    
    const CYAN   : Self = Self::rgba(T::MIN_RANGE, T::MAX_RANGE, T::MAX_RANGE, T::MAX_RANGE);
    const PINK   : Self = Self::rgba(T::MAX_RANGE, T::MIN_RANGE, T::MAX_RANGE, T::MAX_RANGE);
    const YELLOW : Self = Self::rgba(T::MAX_RANGE, T::MAX_RANGE, T::MIN_RANGE, T::MAX_RANGE);
    
    fn rgba_from_bytes(r : u8, g : u8, b : u8, a : u8) -> Self 
    {
        Self::new
        (
            // Todo : do better I guess... Specialization isn't here...
            T::from_coef(r.to_coef()), 
            T::from_coef(g.to_coef()),
            T::from_coef(b.to_coef()),
            T::from_coef(a.to_coef())
        )
    }

    fn to_hsla(self) -> ColorHSLA
    {
        // Thank to MacroQuad, the following code was copied and edited the code from the MacroQuad crate

        

        let f = self.to_float();
        let (r, g, b, a) = f.unpack_rgba();

        let max = *f.max_element();
        let min = *f.min_element();

        // Luminosity is the average of the max and min rgb color intensities.
        let l= (max + min) / 2.0;

        // Saturation
        let delta = max - min;
        if delta == 0.0 { return ColorHSLA::new(0.0, 0.0, l, a); }

        // it's not gray
        let s : float = if l < 0.5 
        {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        // Hue
        let r2 = (((max - r) / 6.0) + (delta / 2.0)) / delta;
        let g2 = (((max - g) / 6.0) + (delta / 2.0)) / delta;
        let b2 = (((max - b) / 6.0) + (delta / 2.0)) / delta;

        let mut h = match max {
            x if x == r => b2 - g2,
            x if x == g => (1.0 / 3.0) + r2 - b2,
            _ => (2.0 / 3.0) + g2 - r2,
        };

        // Fix wraparounds
        if h < 0. { h += 1.0; } else if h > 1. { h -= 1.0; }

        ColorHSLA::new(h, s, l, a)
    }

    fn to_rgba(self) -> Color { Color::rgba(self.r.to_coef(), self.g.to_coef(), self.b.to_coef(), self.a.to_coef()) }
    fn to_rgba_u8(self) -> ColorRGBAByte
    {
        ColorRGBAByte::rgba
        (
          (self.r.to_coef() * (u8::MAX as Coef)) as u8,
        (self.g.to_coef() * (u8::MAX as Coef)) as u8,
         (self.b.to_coef() * (u8::MAX as Coef)) as u8,
        (self.a.to_coef() * (u8::MAX as Coef)) as u8
        )
    }
}