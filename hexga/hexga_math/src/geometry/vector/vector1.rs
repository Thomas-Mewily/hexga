use super::*;

/// 1 dimension : x
#[repr(C)]
pub struct CoordX<T> { pub x : T }
impl<T> Deref for Vector<T, 1>
{
    type Target=CoordX<T>;
    fn deref(&self) -> &Self::Target { unsafe { std::mem::transmute(self) } }
}
impl<T> DerefMut for Vector<T, 1>
{
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } }
}

pub type Vector1<T> = Vector<T, 1>;

impl<T> Vector<T,1> // Hardcode N here otherwise rust-analyser will not like it
{
    pub const fn new(x : T) -> Self { Self::from_array([x]) }
    pub fn with_y(self, y : T) -> Vector2<T> { let [x] = self.array; Vector2::new(x, y) }
}

pub trait SplatCoord1 : Sized + Copy { fn splat1(self) -> Vector1<Self> { Vector1::splat(self) }}
impl<T:Copy> SplatCoord1 for T {}

impl<T> From<(T,)> for Vector1<T> { fn from(value: (T,)) -> Self { Vector1::new(value.0) }}
impl<T> From<Vector1<T>> for (T,) { fn from(value: Vector1<T>) -> Self { let [x] = value.array; (x,) }}

pub const fn vector1<T>(x : T) -> Vector1<T> { Vector1::new(x) }

pub type Bool1 = Vector1<bool>;
pub const fn bool1(x : bool) -> Bool1 { Bool1::new(x) }

pub type Vec1 = Vector1<float>;
pub const fn vec1(x : float) -> Vec1 { Vec1::new(x) }
pub type Coef1 = Vec1;

pub type Point1 = Vector1<int>;
pub const fn point1(x : int) -> Point1 { Point1::new(x) }

impl<T> HaveX<T> for Vector1<T>
{
    fn iter_x<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a {
        self.array().as_slice()[0..=Self::X_INDEX].iter()
    }

    fn iter_x_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> where T: 'a {
        self.array_mut().as_mut_slice()[0..=Self::X_INDEX].iter_mut()
    }
}
impl<T> HaveXAndOne<T> for Vector1<T> where T: One + Zero { const X : Self = Vector1::new(T::ONE); }
impl<T> HaveXAndMinusOne<T> for Vector1<T> where T: MinusOne + Zero { const MINUS_X : Self = Vector1::new(T::MINUS_ONE); }

impl<T> From<Vector2<T>> for Vector1<T> { fn from(value: Vector2<T>) -> Self { let [x,..] = value.to_array(); Self::new(x) } }
impl<T> From<Vector3<T>> for Vector1<T> { fn from(value: Vector3<T>) -> Self { let [x,..] = value.to_array(); Self::new(x) } }
impl<T> From<Vector4<T>> for Vector1<T> { fn from(value: Vector4<T>) -> Self { let [x,..] = value.to_array(); Self::new(x) } }

pub type Vector1Iter<T> = VectorIter<Vector1<T>, 1>;

pub(crate) mod prelude
{
    pub use super::
    {
        SplatCoord1,
        Vector1,vector1,
        Vec1,vec1,
        Bool1,bool1,
        Point1,point1,
    };
}