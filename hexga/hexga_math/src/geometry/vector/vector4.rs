use super::*;

/// 4 dimensions: x, y, z, w
#[repr(C)]
pub struct CoordXYZW<T>
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
impl<T> Deref for Vector<T, 4>
{
    type Target=CoordXYZW<T>;
    fn deref(&self) -> &Self::Target { unsafe { std::mem::transmute(self) } }
}
impl<T> DerefMut for Vector<T, 4>
{
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } }
}

pub type Vector4<T> = Vector<T, 4>;

impl<T> Vector<T,4> // Hardcode N here otherwise rust-analyser will not like it
{
    pub const fn new(x: T, y: T, z: T, w: T) -> Self { Self::from_array([x,y,z,w]) }
}

pub trait SplatCoord4: Sized + Copy { fn splat4(self) -> Vector4<Self> { Vector4::splat(self) }}
impl<T:Copy> SplatCoord4 for T {}

impl<T> From<(T,T,T,T,)> for Vector4<T> { fn from(value: (T,T,T,T,)) -> Self { Vector4::new(value.0, value.1, value.2, value.3) }}
impl<T> From<Vector4<T>> for (T,T,T,T,) { fn from(value: Vector4<T>) -> Self { let [x, y, z, w] = value.array; (x,y,z,w,) }}

pub const fn vector4<T>(x: T, y: T, z: T, w: T) -> Vector4<T> { Vector4::new(x, y, z, w) }

pub type Bool4 = Bool<4>;
pub const fn vec4b(x: bool, y: bool, z: bool, w: bool) -> Bool4 { Bool4::new(x, y, z, w) }

pub type Vec4 = Vector4<float>;
pub const fn vec4(x: float, y: float, z: float, w: float) -> Vec4 { Vec4::new(x, y, z, w) }
pub type Coef4 = Vec4;

pub type Point4 = Point<4>;
pub const fn point4(x: int, y: int, z: int, w: int) -> Point4 { Point4::new(x, y, z, w) }

impl<T> HaveX<T> for Vector4<T>
{
    fn iter_x<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a {
        self.array().as_slice()[0..=Self::X_INDEX].iter()
    }

    fn iter_x_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> where T: 'a {
        self.array_mut().as_mut_slice()[0..=Self::X_INDEX].iter_mut()
    }
}
impl<T> HaveXAndOne<T> for Vector4<T> where T: One + Zero { const X: Self = Vector4::new(T::ONE, T::ZERO, T::ZERO, T::ZERO); }
impl<T> HaveXAndMinusOne<T> for Vector4<T> where T: MinusOne + Zero { const MINUS_X: Self = Vector4::new(T::MINUS_ONE, T::ZERO, T::ZERO, T::ZERO); }

impl<T> HaveY<T> for Vector4<T>
{
    fn iter_xy<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a {
        self.array().as_slice()[0..=Self::Y_INDEX].iter()
    }

    fn iter_xy_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> where T: 'a {
        self.array_mut().as_mut_slice()[0..=Self::Y_INDEX].iter_mut()
    }
}
impl<T> HaveYAndOne<T> for Vector4<T> where T: One + Zero { const Y: Self = Vector4::new(T::ZERO, T::ONE, T::ZERO, T::ZERO); }
impl<T> HaveYAndMinusOne<T> for Vector4<T> where T: MinusOne + Zero { const MINUS_Y: Self = Vector4::new(T::ZERO, T::MINUS_ONE, T::ZERO, T::ZERO); }

impl<T> HaveZ<T> for Vector4<T>
{
    fn iter_xyz<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a {
        self.array().as_slice()[0..=Self::Z_INDEX].iter()
    }

    fn iter_xyz_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> where T: 'a {
        self.array_mut().as_mut_slice()[0..=Self::Z_INDEX].iter_mut()
    }
}
impl<T> HaveZAndOne<T> for Vector4<T> where T: One + Zero { const Z: Self = Vector4::new(T::ZERO, T::ZERO, T::ONE, T::ZERO); }
impl<T> HaveZAndMinusOne<T> for Vector4<T> where T: MinusOne + Zero { const MINUS_Z: Self = Vector4::new(T::ZERO, T::ZERO, T::MINUS_ONE, T::ZERO); }

impl<T> HaveW<T> for Vector4<T>
{
    fn iter_xyzw<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a {
        self.array().as_slice()[0..=Self::W_INDEX].iter()
    }

    fn iter_xyzw_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> where T: 'a {
        self.array_mut().as_mut_slice()[0..=Self::W_INDEX].iter_mut()
    }
}
impl<T> HaveWAndOne<T> for Vector4<T> where T: One + Zero { const W: Self = Vector4::new(T::ZERO, T::ZERO, T::ZERO, T::ONE); }
impl<T> HaveWAndMinusOne<T> for Vector4<T> where T: MinusOne + Zero { const MINUS_W: Self = Vector4::new(T::ZERO, T::ZERO, T::ZERO, T::MINUS_ONE); }

impl<T> From<Vector1<T>> for Vector4<T> where T: Default { fn from(value: Vector1<T>) -> Self { value.resize() } }
impl<T> From<Vector2<T>> for Vector4<T> where T: Default { fn from(value: Vector2<T>) -> Self { value.resize() } }
impl<T> From<Vector3<T>> for Vector4<T> where T: Default { fn from(value: Vector3<T>) -> Self { value.resize() } }

pub type Vector4Iter<T> = VectorIter<Vector4<T>, 4>;

pub(crate) mod prelude
{
    pub use super::
    {
        SplatCoord4,
        Vector4,vector4,
        Vec4,vec4,
        Bool4,vec4b,
        Point4,point4,
    };
}