use super::*;

/// 2 dimensions: x, y
#[repr(C)]
pub struct CoordXY<T>
{
    pub x: T,
    pub y: T,
}
impl<T> Deref for Vector<T, 2>
{
    type Target = CoordXY<T>;
    fn deref(&self) -> &Self::Target { unsafe { std::mem::transmute(self) } }
}
impl<T> DerefMut for Vector<T, 2>
{
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } }
}

pub type Vector2<T> = Vector<T, 2>;

impl<T> Vector<T, 2> // Hardcode N here otherwise rust-analyser will not like it
{
    pub const fn new(x: T, y: T) -> Self { Self::from_array([x, y]) }
    pub fn with_z(self, z: T) -> Vector3<T>
    {
        let [x, y] = self.array;
        Vector3::new(x, y, z)
    }
}

pub trait SplatCoord2: Sized + Copy
{
    fn splat2(self) -> Vector2<Self> { Vector2::splat(self) }
}
impl<T: Copy> SplatCoord2 for T {}

impl<T> From<(T, T)> for Vector2<T>
{
    fn from(value: (T, T)) -> Self { Vector2::new(value.0, value.1) }
}
impl<T> From<Vector2<T>> for (T, T)
{
    fn from(value: Vector2<T>) -> Self
    {
        let [x, y] = value.array;
        (x, y)
    }
}

pub const fn vector2<T>(x: T, y: T) -> Vector2<T> { Vector2::new(x, y) }

pub type Bool2 = Bool<2>;
pub const fn bool2(x: bool, y: bool) -> Bool2 { Bool2::new(x, y) }

pub type Vec2 = Vector2<float>;
pub const fn vec2(x: float, y: float) -> Vec2 { Vec2::new(x, y) }
pub type Coef2 = Vec2;

pub type Point2 = Point<2>;
pub const fn point2(x: int, y: int) -> Point2 { Point2::new(x, y) }

impl<T> HaveX<T> for Vector2<T>
{
    fn iter_x<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.array().as_slice()[0..=Self::X_INDEX].iter()
    }

    fn iter_x_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a,
    {
        self.array_mut().as_mut_slice()[0..=Self::X_INDEX].iter_mut()
    }
}
impl<T> HaveXAndOne<T> for Vector2<T>
where
    T: One + Zero,
{
    const X: Self = Vector2::new(T::ONE, T::ZERO);
}
impl<T> HaveXAndMinusOne<T> for Vector2<T>
where
    T: MinusOne + Zero,
{
    const MINUS_X: Self = Vector2::new(T::MINUS_ONE, T::ZERO);
}

impl<T> HaveY<T> for Vector2<T>
{
    fn iter_xy<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.array().as_slice()[0..=Self::Y_INDEX].iter()
    }

    fn iter_xy_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a,
    {
        self.array_mut().as_mut_slice()[0..=Self::Y_INDEX].iter_mut()
    }
}
impl<T> HaveYAndOne<T> for Vector2<T>
where
    T: One + Zero,
{
    const Y: Self = Vector2::new(T::ZERO, T::ONE);
}
impl<T> HaveYAndMinusOne<T> for Vector2<T>
where
    T: MinusOne + Zero,
{
    const MINUS_Y: Self = Vector2::new(T::ZERO, T::MINUS_ONE);
}

impl<T> From<Vector1<T>> for Vector2<T>
where
    T: Default,
{
    fn from(value: Vector1<T>) -> Self { value.resize() }
}
impl<T> From<Vector3<T>> for Vector2<T>
{
    fn from(value: Vector3<T>) -> Self
    {
        let [x, y, ..] = value.to_array();
        Self::new(x, y)
    }
}
impl<T> From<Vector4<T>> for Vector2<T>
{
    fn from(value: Vector4<T>) -> Self
    {
        let [x, y, ..] = value.to_array();
        Self::new(x, y)
    }
}

pub type Vector2Iter<T> = VectorIter<Vector2<T>, 2>;

pub(crate) mod prelude
{
    pub use super::{Bool2, Point2, SplatCoord2, Vec2, Vector2, bool2, point2, vec2, vector2};
}
