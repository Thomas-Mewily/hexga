use super::*;
use std::cmp::Ordering;



/// An array that is indexable, without any information if the type and the size are generic.
/// Check [`ArrayWithType`] and [ArrayWithSize] for editable generic type.
pub trait Array<T, const N : usize>:
    From <[T; N]> +
    Into <[T; N]> +
    Index<usize,Output = T> + IndexMut<usize,Output = T>
    // It's not using this, because it is not implemented for array
    // + AsRef<[T;N]> + AsMut<[T;N]>
{
    const DIMENSION : usize = N;

    fn array(&self) -> &[T; N];
    fn array_mut(&mut self) -> &mut[T; N];

    /// Fill non existing component with [`Default`]
    fn to_array_resized<const M:usize>(self) -> [T; M] where T: Default
    {
        let mut it = <Self as Into<[T;N]>>::into(self).into_iter();
        <[T; M]>::from_fn(|_| it.next().unwrap_or_default())
    }

    /// Fill non existing component with the [`Clone`]d given value
    fn to_array_resized_with_value<const M:usize>(self, fill : T) -> [T; M] where T: Clone
    {
        let mut it = <Self as Into<[T;N]>>::into(self).into_iter();
        <[T; M]>::from_fn(|_| it.next().unwrap_or_else(|| fill.clone()))
    }
}
impl<T,const N:usize> Array<T,N> for [T;N]
{
    fn array(&self) -> &[T; N] { self }
    fn array_mut(&mut self) -> &mut[T; N] { self }
}

/// An type equivalent to an array with a generic size that can be changed
///
/// ex, `[T;N]` is an array with a generic size, but `struct RGBA([T;4])` don't have a generic size (always 4 components)
pub trait ArrayWithSize<T, const N : usize>: Array<T,N>
{
    type WithSize<const M:usize> : ArrayWithSize<T, M>;

    /// Resize the array to the given size, filling non existing component with [`Default`]
    /// Fill non existing component with [`Default`]
    fn resize<const M:usize>(self) -> Self::WithSize<M> where T: Default
    {
        Self::WithSize::from_array(self.to_array_resized())
    }

    /// Fill non existing component with the [`Clone`]d given value
    fn resize_with_value<const M:usize>(self, value: T) -> Self::WithSize<M> where T: Clone
    {
        Self::WithSize::from_array(self.to_array_resized_with_value(value))
    }
}
impl<T, const N:usize> ArrayWithSize<T,N> for [T;N]
{
    type WithSize<const M:usize> = [T;M];
}

/// An array with a generic type that can be changed
///
/// ex, `[T;N]` is an array with a generic type, but `struct VecF32<const N: usize>([f32;N])` don't have a generic type (4 components)
pub trait ArrayWithType<T, const N: usize>: Array<T,N> +
{
    type WithType<T2> : ArrayWithType<T2, N>;
}
impl<T, const N:usize> ArrayWithType<T,N> for [T;N]
{
    type WithType<T2> = [T2;N];
}




impl<S,T,const N:usize> ArrayMin<T,N> for S where S:Array<T,N> {}
/// Assuming the size N != 0
pub trait ArrayMin<T,const N:usize> : Array<T,N>
{
    fn min_element_by<F>(&self, mut compare: F) -> &T where F: FnMut(&T, &T) -> Ordering { self.array().iter().min_by(|a,b| compare(a, b)).expect("size can't be empty")  }
    fn min_element_by_key<F,K>(&self, mut f: F) -> &T where F: FnMut(&T) -> K, K: Ord  { self.min_element_by(|a, b| f(a).cmp(&f(b))) }
    fn min_element_mut_by<F>(&mut self, mut compare: F) -> &mut T where F: FnMut(&T, &T) -> Ordering { self.array_mut().iter_mut().min_by(|a,b| compare(a, b)).expect("size can't be empty") }
    fn min_element_mut_by_key<F,K>(&mut self, mut f: F) -> &mut T where F: FnMut(&T) -> K, K: Ord { self.min_element_mut_by(|a, b| f(a).cmp(&f(b))) }
    fn min_element(&self) -> &T where T: PartialOrd { self.array().iter().min_by(|a, b| a.partial_cmp(b).unwrap()).expect("size can't be empty") }
    fn min_element_mut(&mut self) -> &mut T where T: PartialOrd { self.array_mut().iter_mut().min_by(|a, b| a.partial_cmp(b).unwrap()).expect("size can't be empty") }
    fn min_element_idx(&self) -> usize where T: PartialOrd { self.array().iter().enumerate().min_by(|(_,a), (_,b)| a.partial_cmp(b).unwrap()).map(|(idx,_)| idx).unwrap() }
}


/// Assuming the size N != 0
impl<S,T,const N:usize> ArrayMax<T,N> for S where S:Array<T,N> {}
pub trait ArrayMax<T,const N:usize> : Array<T,N>
{
    fn max_element_by<F>(&self, mut compare: F) -> &T where F: FnMut(&T, &T) -> Ordering { self.array().iter().max_by(|a,b| compare(a, b)).expect("size can't be empty")  }
    fn max_element_by_key<F,K>(&self, mut f: F) -> &T where F: FnMut(&T) -> K, K: Ord  { self.max_element_by(|a, b| f(a).cmp(&f(b))) }
    fn max_element_mut_by<F>(&mut self, mut compare: F) -> &mut T where F: FnMut(&T, &T) -> Ordering { self.array_mut().iter_mut().max_by(|a,b| compare(a, b)).expect("size can't be empty") }
    fn max_element_mut_by_key<F,K>(&mut self, mut f: F) -> &mut T where F: FnMut(&T) -> K, K: Ord { self.max_element_mut_by(|a, b| f(a).cmp(&f(b))) }
    fn max_element(&self) -> &T where T: PartialOrd { self.array().iter().max_by(|a, b| a.partial_cmp(b).unwrap()).expect("size can't be empty") }
    fn max_element_mut(&mut self) -> &mut T where T: PartialOrd { self.array_mut().iter_mut().max_by(|a, b| a.partial_cmp(b).unwrap()).expect("size can't be empty") }
    fn max_element_idx(&self) -> usize where T: PartialOrd { self.array().iter().enumerate().max_by(|(_,a), (_,b)| a.partial_cmp(b).unwrap()).map(|(idx,_)| idx).unwrap() }
}