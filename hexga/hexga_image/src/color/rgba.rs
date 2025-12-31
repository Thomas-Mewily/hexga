use super::*;

pub type Rgba      = RgbaFloat;
pub type RgbaFloat = RgbaOf<float>;
pub type RgbaU8    = RgbaOf<u8>;
pub type RgbaU16   = RgbaOf<u16>;
pub type RgbaF32   = RgbaOf<f32>;
pub type RgbaF64   = RgbaOf<f64>;

#[repr(C)]
pub struct RgbaOf<T>
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
hexga_math::impl_fixed_array!(RgbaOf, 4);
unsafe impl<T> BitAllUsed for RgbaOf<T> where T:BitAllUsed {}

#[allow(dead_code)]
impl<T> RgbaOf<T>
{
    pub const fn new(red : T, green : T, blue : T, alpha : T) -> Self { Self { r:red, g:green, b:blue, a:alpha }}

    pub const fn rgba(red : T, green : T, blue : T, alpha : T) -> Self { Self::new(red, green, blue, alpha) }
    /// Alpha is at max
    pub const fn rgb (red : T, green : T, blue : T) -> Self where T: RangeDefault { Self::rgba(red, green, blue, T::RANGE_MAX) }
    /// Alpha is at max
    pub const fn gray(rgb : T) -> Self where T: RangeDefault + Copy { Self::rgb(rgb, rgb, rgb) }

    pub const fn splat_rgba(rgba : T) -> Self where T: Copy { Self::new(rgba, rgba, rgba, rgba) }
    /// Alpha is at max
    pub const fn splat_rgb(rgb : T) -> Self where T: Copy + RangeDefault { Self::splat_rgb_with_a(rgb, T::RANGE_MAX) }
    pub const fn splat_rgb_with_a(rgb : T, a : T) -> Self where T: Copy { Self::new(rgb, rgb, rgb, a) }



    pub fn rgba_ref(&    self) -> &    [T; 4] { self.as_array() }
    pub fn rgba_mut(&mut self) -> &mut [T; 4] { self.as_array_mut() }

    pub fn rgb_ref(&self) -> &[T; 3] {
        // SAFETY: ColorRGBAOf<T> always has at least 3 fields, and they are laid out contiguously.
        unsafe { &*(self.as_array().as_ptr() as *const [T; 3]) }
    }
    pub fn rgb_mut(&mut self) -> &mut [T; 3] {
        // SAFETY: ColorRGBAOf<T> always has at least 3 fields, and they are laid out contiguously.
        unsafe { &mut *(self.as_array_mut().as_mut_ptr() as *mut [T; 3]) }
    }


    /// Red
    pub const R_INDEX : usize = 0;
    /// Red
    pub fn r(&self) -> T where T: Copy { self.r }
    /// Red
    pub fn with_r(mut self, r : T) -> Self where T: Copy { self.set_r(r); self }
    /// Red
    pub fn set_r(&mut self, r : T) -> &mut Self { self.r = r; self }
    /// Red
    pub fn replace_r(mut self, r : T) -> T { self.replace_or_panic(Self::R_INDEX, r) }

    /// Green
    pub const G_INDEX : usize = 1;
    /// Green
    pub fn g(&self) -> T where T: Copy { self.g }
    /// Green
    pub fn with_g(mut self, g : T) -> Self where T: Copy { self.set_g(g); self }
    /// Green
    pub fn set_g(&mut self, g : T) -> &mut Self { self.g = g; self }
    /// Green
    pub fn replace_g(mut self, g : T) -> T { self.replace_or_panic(Self::G_INDEX, g) }

    /// Blue
    pub const B_INDEX : usize = 2;
    /// Blue
    pub fn b(&self) -> T where T: Copy { self.b }
    /// Blue
    pub fn with_b(mut self, b : T) -> Self where T: Copy { self.set_b(b); self }
    /// Blue
    pub fn set_b(&mut self, b : T) -> &mut Self { self.b = b; self }
    /// Blue
    pub fn replace_b(mut self, b : T) -> T { self.replace_or_panic(Self::B_INDEX, b) }

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

impl<T> From<(T,T,T,T,)> for RgbaOf<T> { fn from(value: (T,T,T,T,)) -> Self { RgbaOf::rgba(value.0, value.1, value.2, value.3) }}
impl<T> From<RgbaOf<T>> for (T,T,T,T,) { fn from(value: RgbaOf<T>) -> Self { (value.r, value.g, value.b, value.a) }}

impl<T> From<(T,T,T,)> for RgbaOf<T> where T: RangeDefault { fn from(value: (T,T,T,)) -> Self { RgbaOf::rgb(value.0, value.1, value.2) }}
impl<T> From<RgbaOf<T>> for (T,T,T,) { fn from(value: RgbaOf<T>) -> Self { (value.r, value.g, value.b) }}

impl<T> From<[T; 3]> for RgbaOf<T> where T: RangeDefault { fn from(value: [T; 3]) -> Self { let [r,g,b] = value; RgbaOf::rgb(r,g,b) }}
impl<T> From<RgbaOf<T>> for [T; 3] { fn from(value: RgbaOf<T>) -> Self { [value.r, value.g, value.b] }}

impl<T> From<Vector4<T>> for RgbaOf<T> { fn from(value: Vector4<T>) -> Self { let [r,g,b,a] = value.to_array(); RgbaOf::rgba(r,g,b,a) }}
impl<T> From<RgbaOf<T>> for Vector4<T> { fn from(value: RgbaOf<T>) -> Self { let [x,y,z,w] = value.into(); vector4(x,y,z,w) }}

impl<T> From<Vector3<T>> for RgbaOf<T> where T: RangeDefault { fn from(value: Vector3<T>) -> Self { let [r,g,b] = value.to_array(); RgbaOf::rgb(r,g,b) }}
impl<T> From<RgbaOf<T>> for Vector3<T> { fn from(value: RgbaOf<T>) -> Self { let [x,y,z,_] = value.into(); vector3(x,y,z) }}

impl<T> Default for RgbaOf<T> where T: Primitive
{
    fn default() -> Self { Self { r: T::RANGE_MAX, g: T::RANGE_MAX, b: T::RANGE_MAX, a: T::RANGE_MAX } }
}

impl<T> RgbaOf<T> where T: Primitive
{
    pub fn to_rgba_of<R>(self) -> RgbaOf<R> where R : Primitive + CastRangeFrom<T> { self.cast_range_into() }

    pub fn to_hsla_of<R>(self) -> HslaOf<R> where R : Float + CastRangeFrom<T> {

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


impl<T> IColor for RgbaOf<T> where T: Primitive
{
    type Component = T;

    const TRANSPARENT : Self = Self::rgba(T::RANGE_MAX, T::RANGE_MAX, T::RANGE_MAX, T::RANGE_MAX);

    const BLACK : Self = Self { r: T::RANGE_MIN , g: T::RANGE_MIN , b: T::RANGE_MIN , a: T::RANGE_MAX };
    const GREY  : Self = Self { r: T::RANGE_HALF, g: T::RANGE_HALF, b: T::RANGE_HALF, a: T::RANGE_MAX };
    const WHITE : Self = Self { r: T::RANGE_MAX , g: T::RANGE_MAX , b: T::RANGE_MAX , a: T::RANGE_MAX };

    const RED    : Self = Self::rgb(T::RANGE_MAX, T::RANGE_MIN, T::RANGE_MIN);
    const GREEN  : Self = Self::rgb(T::RANGE_MIN, T::RANGE_MAX, T::RANGE_MIN);
    const BLUE   : Self = Self::rgb(T::RANGE_MIN, T::RANGE_MIN, T::RANGE_MAX);

    const CYAN   : Self = Self::rgb(T::RANGE_MIN, T::RANGE_MAX, T::RANGE_MAX);
    const MAGENTA: Self = Self::rgb(T::RANGE_MAX, T::RANGE_MIN, T::RANGE_MAX);
    const YELLOW : Self = Self::rgb(T::RANGE_MAX, T::RANGE_MAX, T::RANGE_MIN);

    const SPRING : Self = Self::rgb(T::RANGE_MIN, T::RANGE_MAX, T::RANGE_HALF);
    const AZURE  : Self = Self::rgb(T::RANGE_MIN, T::RANGE_HALF, T::RANGE_MAX);
    const VIOLET : Self = Self::rgb(T::RANGE_HALF, T::RANGE_MIN, T::RANGE_MAX);
    const ROSE   : Self = Self::rgb(T::RANGE_MAX, T::RANGE_MIN, T::RANGE_HALF);
    const ORANGE : Self = Self::rgb(T::RANGE_MAX, T::RANGE_HALF, T::RANGE_MIN);
    const LIME   : Self = Self::rgb(T::RANGE_HALF, T::RANGE_MAX, T::RANGE_MIN);
    const CANARY : Self = Self::rgb(T::RANGE_MAX, T::RANGE_MAX, T::RANGE_HALF);
    const PINK   : Self = Self::rgb(T::RANGE_MAX, T::RANGE_HALF, T::RANGE_MAX);
    const GLACE  : Self = Self::rgb(T::RANGE_HALF, T::RANGE_MAX, T::RANGE_MAX);

    fn to_rgba_of<R>(self) -> RgbaOf<R> where R : Primitive + CastRangeFrom<Self::Component> {
        self.to_rgba_of()
    }

    fn to_hsla_of<R>(self) -> HslaOf<R> where R : Float + CastRangeFrom<Self::Component> {
        self.to_hsla_of()
    }

    fn from_rgba_u8(rgba: RgbaU8) -> Self { rgba.to_rgba_of() }
    fn from_rgba_u16(rgba: RgbaU16) -> Self { rgba.to_rgba_of() }
    fn from_rgba_f32(rgba: RgbaOf<f32>) -> Self { rgba.to_rgba_of() }
    fn from_rgba_f64(rgba: RgbaOf<f64>) -> Self { rgba.to_rgba_of() }
    fn from_rgba_float(rgba: RgbaFloat) -> Self { rgba.to_rgba_of() }
}

/*
impl<T> ToColorComposite for ColorRgbaOf<T> where T: Primitive
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

    const COLOR_INSIDE : ColorKind =
    {
        match (T::PRIMITIVE_TYPE, std::mem::size_of::<T>())
        {
            (NumberType::Bool,  1) => ColorKind::RgbaBool,
            (NumberType::Float, 4) => ColorKind::RgbaF32,
            (NumberType::Float, 8) => ColorKind::RgbaF64,
            (NumberType::IntegerUnsigned, 1) => ColorKind::RgbaU8,
            (NumberType::IntegerUnsigned, 2) => ColorKind::RgbaU16,
            _ => ColorKind::Unknow,
        }
    };
}
*/

pub const fn rgba<T>(red : T, green : T, blue : T, alpha : T) -> RgbaOf<T>
{
    RgbaOf::rgba(red, green, blue, alpha)
}
/// Alpha is at max
pub const fn rgb<T>(red : T, green : T, blue : T) -> RgbaOf<T> where T: RangeDefault
{
    RgbaOf::rgb(red, green, blue)
}