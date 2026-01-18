use super::*;

/// 3 dimensions: x, y, z
#[repr(C)]
pub struct CoordXYZ<T>
{
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T> Deref for Vector<T, 3>
{
    type Target=CoordXYZ<T>;
    fn deref(&self) -> &Self::Target { unsafe { std::mem::transmute(self) } }
}
impl<T> DerefMut for Vector<T, 3>
{
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } }
}

pub type Vector3<T> = Vector<T, 3>;

impl<T> Vector<T,3> // Hardcode N here otherwise rust-analyser will not like it
{
    pub const fn new(x: T, y: T, z: T) -> Self { Self::from_array([x,y,z]) }
    pub fn with_w(self, w: T) -> Vector4<T> { let [x, y, z] = self.array; Vector4::new(x, y, z, w) }
}

pub trait SplatCoord3: Sized + Copy { fn splat3(self) -> Vector3<Self> { Vector3::splat(self) }}
impl<T:Copy> SplatCoord3 for T {}

impl<T> From<(T,T,T,)> for Vector3<T> { fn from(value: (T,T,T,)) -> Self { Vector3::new(value.0, value.1, value.2) }}
impl<T> From<Vector3<T>> for (T,T,T,) { fn from(value: Vector3<T>) -> Self { let [x, y, z] = value.array; (x,y,z,) }}

pub const fn vector3<T>(x: T, y: T, z: T) -> Vector3<T> { Vector3::new(x, y, z) }

pub type Bool3 = Bool<3>;
pub const fn bool3(x: bool, y: bool, z: bool) -> Bool3 { Bool3::new(x, y, z) }

pub type Vec3 = Vector3<float>;
pub const fn vec3(x: float, y: float, z: float) -> Vec3 { Vec3::new(x, y, z) }
pub type Coef3 = Vec3;

pub type Point3 = Point<3>;
pub const fn point3(x: int, y: int, z: int) -> Point3 { Point3::new(x, y, z) }

impl<T> HaveX<T> for Vector3<T>
{
    fn iter_x<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a {
        self.array().as_slice()[0..=Self::X_INDEX].iter()
    }

    fn iter_x_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> where T: 'a {
        self.array_mut().as_mut_slice()[0..=Self::X_INDEX].iter_mut()
    }
}
impl<T> HaveXAndOne<T> for Vector3<T> where T: One + Zero { const X: Self = Vector3::new(T::ONE, T::ZERO, T::ZERO); }
impl<T> HaveXAndMinusOne<T> for Vector3<T> where T: MinusOne + Zero { const MINUS_X: Self = Vector3::new(T::MINUS_ONE, T::ZERO, T::ZERO); }

impl<T> HaveY<T> for Vector3<T>
{
    fn iter_xy<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a {
        self.array().as_slice()[0..=Self::Y_INDEX].iter()
    }

    fn iter_xy_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> where T: 'a {
        self.array_mut().as_mut_slice()[0..=Self::Y_INDEX].iter_mut()
    }
}
impl<T> HaveYAndOne<T> for Vector3<T> where T: One + Zero { const Y: Self = Vector3::new(T::ZERO, T::ONE, T::ZERO); }
impl<T> HaveYAndMinusOne<T> for Vector3<T> where T: MinusOne + Zero { const MINUS_Y: Self = Vector3::new(T::ZERO, T::MINUS_ONE, T::ZERO); }

impl<T> HaveZ<T> for Vector3<T>
{
    fn iter_xyz<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a {
        self.array().as_slice()[0..=Self::Z_INDEX].iter()
    }

    fn iter_xyz_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> where T: 'a {
        self.array_mut().as_mut_slice()[0..=Self::Z_INDEX].iter_mut()
    }
}
impl<T> HaveZAndOne<T> for Vector3<T> where T: One + Zero { const Z: Self = Vector3::new(T::ZERO, T::ZERO, T::ONE); }
impl<T> HaveZAndMinusOne<T> for Vector3<T> where T: MinusOne + Zero { const MINUS_Z: Self = Vector3::new(T::ZERO, T::ZERO, T::MINUS_ONE); }

impl<T> From<Vector1<T>> for Vector3<T> where T: Default { fn from(value: Vector1<T>) -> Self { value.resize() } }
impl<T> From<Vector2<T>> for Vector3<T> where T: Default { fn from(value: Vector2<T>) -> Self { value.resize() } }
impl<T> From<Vector4<T>> for Vector3<T> { fn from(value: Vector4<T>) -> Self { let [x, y, z, ..] = value.to_array(); Self::new(x, y, z) } }

pub type Vector3Iter<T> = VectorIter<Vector3<T>, 3>;


// Based on the glam crate
impl<T> Vector3<T> where T:Numeric
{
    /// Computes the cross product of `self` and `rhs`.
    pub fn cross(self, rhs: Self) -> Self
    {
        Self::new(
            self.y * rhs.z - rhs.y * self.z,
             self.z * rhs.x - rhs.z * self.x,
             self.x * rhs.y - rhs.x * self.y
        )
    }
}


pub(crate) mod prelude
{
    pub use super::
    {
        SplatCoord3,
        Vector3,vector3,
        Vec3,vec3,
        Bool3,bool3,
        Point3,point3,
    };
}