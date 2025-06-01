use crate::*;


impl<S, T, const N : usize> ArrayLikeColorExtension<T,N> for S where S : ArrayLikeExtension<T,N> {}
pub trait ArrayLikeColorExtension<T, const N : usize> : ArrayLikeExtension<T,N>
{
    fn to_rgba(self) -> ColorRgbaOf<T> where T: Default { ColorRgbaOf::from_array(self.into_array4()) }
    fn to_hlsa(self) -> ColorHslaOf<T> where T: Default { ColorHslaOf::from_array(self.into_array4()) }
}
