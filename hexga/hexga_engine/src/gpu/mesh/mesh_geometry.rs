use super::*;

pub mod prelude
{
    pub use super::{MeshGeometry,MeshTriangle};
}

pub type MeshTriangle<const V:usize=3>=MeshGeometry<V, 3>;
pub type MeshGeometry<const V:usize, const N:usize> = MeshGeometryOf<Vertex<V>,N>; 

impl<T> MeshGeometryOf<T,2>
{
    pub const fn new(a : T, b: T) -> Self { Self::from_array([a,b]) }
}
impl<T> MeshGeometryOf<T,3>
{
    pub const fn new(a : T, b: T, c:T) -> Self { Self::from_array([a,b,c]) }
}

#[derive(Clone, Copy, PartialEq, Debug, PartialOrd)]
pub struct MeshGeometryOf<T,const N:usize>
{
    pub points : [T;N]
}
impl<T,const N:usize> MeshGeometryOf<T,N>
{
    pub const fn from_array(points: [T;N]) -> Self { Self { points }}
}

impl<T,const N:usize> IntoIterator for MeshGeometryOf<T,N>
{
    type Item=<[T;N] as IntoIterator>::Item;
    type IntoIter=<[T;N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { self.points.into_iter() }
}
impl<'a, T,const N:usize> IntoIterator for &'a MeshGeometryOf<T,N>
{
    type Item=<&'a [T;N] as IntoIterator>::Item;
    type IntoIter=<&'a [T;N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { (&self.points).into_iter() }
}
impl<'a, T,const N:usize> IntoIterator for &'a mut MeshGeometryOf<T,N>
{
    type Item=<&'a mut [T;N] as IntoIterator>::Item;
    type IntoIter=<&'a mut [T;N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { (&mut self.points).into_iter() }
}
