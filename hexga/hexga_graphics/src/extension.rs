use crate::*;


impl<S, T, const N : usize> ArrayLikeColorExtension<T,N> for S where S : Array<T,N> {}
pub trait ArrayLikeColorExtension<T, const N : usize> : Array<T,N>
{
    fn to_rgba(self) -> ColorRgbaOf<T> where T: Default { ColorRgbaOf::from_array(self.to_array_with_size()) }
    fn to_hlsa(self) -> ColorHslaOf<T> where T: Default { ColorHslaOf::from_array(self.to_array_with_size()) }
}
