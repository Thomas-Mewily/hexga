use super::*;

pub mod prelude
{
    pub use super::
    {
        Geometry,GeometryTriangle,
        GeometryVertex,TriangleVertex,
        TriangleVertexIndex,
    };
}

pub type TriangleVertex<const V:usize>=GeometryTriangle<Vertex<V>>;
pub type GeometryVertex<const V:usize, const N:usize> = Geometry<Vertex<V>,N>; 

pub type GeometryTriangle<T> = Geometry<T,3>; 
pub type TriangleVertexIndex=GeometryTriangle<VertexIndex>;

impl<T> Geometry<T,2>
{
    pub const fn new(a : T, b: T) -> Self { Self::from_array([a,b]) }
}
impl<T> Geometry<T,3>
{
    pub const fn new(a : T, b: T, c:T) -> Self { Self::from_array([a,b,c]) }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, PartialOrd)]
pub struct Geometry<T,const N:usize>
{
    pub points : [T;N]
}
impl<T,const N:usize> Geometry<T,N>
{
    pub const fn from_array(points: [T;N]) -> Self { Self { points }}
}

impl<T, const N:usize> From<[T;N]> for Geometry<T,N>
{
    fn from(value: [T;N]) -> Self { Self::from_array(value) }
}

impl<T,const N:usize> IntoIterator for Geometry<T,N>
{
    type Item=<[T;N] as IntoIterator>::Item;
    type IntoIter=<[T;N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { self.points.into_iter() }
}
impl<'a, T,const N:usize> IntoIterator for &'a Geometry<T,N>
{
    type Item=<&'a [T;N] as IntoIterator>::Item;
    type IntoIter=<&'a [T;N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { (&self.points).into_iter() }
}
impl<'a, T,const N:usize> IntoIterator for &'a mut Geometry<T,N>
{
    type Item=<&'a mut [T;N] as IntoIterator>::Item;
    type IntoIter=<&'a mut [T;N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { (&mut self.points).into_iter() }
}
