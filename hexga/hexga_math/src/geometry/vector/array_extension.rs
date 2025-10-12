use super::*;

impl<S, T, const N : usize> ToDimension<T,N> for S where S : ArrayWithSize<T,N> {}
pub trait ToDimension<T, const N : usize> : ArrayWithSize<T,N>
{
    fn to_1d(self) -> Self::WithSize<1> where T: Default { self.resize::<1>() }
    fn to_2d(self) -> Self::WithSize<2> where T: Default { self.resize::<2>() }
    fn to_3d(self) -> Self::WithSize<3> where T: Default { self.resize::<3>() }
    fn to_4d(self) -> Self::WithSize<4> where T: Default { self.resize::<4>() }
}
