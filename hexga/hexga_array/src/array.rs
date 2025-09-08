use super::*;
use std::array;


pub mod prelude
{
    pub use super::{Array,ArrayWithGenericSize,ArrayWithGenericType,ArrayMin,ArrayMax};
}



/* 
/// An array where the type and the size are generic and can be changed
pub trait ArrayGeneric<T, const N: usize>: ArrayWithGenericSize<T,N> + ArrayWithGenericType<T,N> 
{
    type WithTypeAndSize<T2,const N2:usize> : ArrayGeneric<T2, N2>;
}
impl<S, T, const N: usize> ArrayGeneric<T,N> for S where S: ArrayWithGenericSize<T,N> + ArrayWithGenericType<T,N> 
{
    type WithTypeAndSize<T2,const N2:usize> = S::WithType<T2>::WithSize<N2>;
}
*/

/// An type equivalent to an array with a generic size that can be changed
/// 
/// ex, `[T;N]` is an array with a generic size, but `struct RGBA([T;N])` don't have a generic size (always 4 components)
pub trait ArrayWithGenericSize<T, const N : usize>: Array<T,N>
{
    type WithSize<const M:usize> : ArrayWithGenericSize<T, M>;

    /// Resize the array to the given size, filling non existing component with [Default]
    /// Fill non existing component with [Default]
    fn with_size<const M:usize>(self) -> Self::WithSize<M> where T: Default
    {
        Self::WithSize::from_array(self.to_array_with_size())
    }

    /// Resize the array to the given size, filling non existing component with [Default]
    /// 
    /// (Same as [ArrayWithGenericSize::with_size])
    fn to_nd<const M:usize>(self) -> Self::WithSize<M> where T: Default { self.with_size() }

    /// Fill non existing component with the [Clone]d given value
    fn with_size_filled_with_value<const M:usize>(self, value: T) -> Self::WithSize<M> where T: Clone
    {
        Self::WithSize::from_array(self.to_array_with_size_filled_with_value(value))
    }
}
impl<T, const N:usize> ArrayWithGenericSize<T,N> for [T;N]
{
    type WithSize<const M:usize> = [T;M];
}

/// An array with a generic type that can be changed
/// 
/// ex, `[T;N]` is an array with a generic type, but `struct VecF32<const N: usize>([f32;N])` don't have a generic type (4 components)
pub trait ArrayWithGenericType<T, const N: usize>: Array<T,N>
{
    type WithType<T2> : ArrayWithGenericType<T2, N>;

    #[inline(always)]
    fn map<U, F>(self, f : F) -> Self::WithType<U> where F : FnMut(T) -> U {  self.to_array().map(f).into() }

    #[inline(always)]
    fn map_with<U, T2, F>(self, other : Self::WithType<T2>, mut f : F) -> Self::WithType<U>
        where F : FnMut(T, T2) -> U
    {
        let mut t1_iter = self.to_array().into_iter();
        let mut t2_iter = other.to_array().into_iter();
        Self::WithType::<U>::from_array(std::array::from_fn(|_| f(t1_iter.next().unwrap(), t2_iter.next().unwrap())))
    }

    /// True if the predicate is true for at least one component
    #[inline(always)]
    fn any_with<T2, P>(&self, other : Self::WithType<T2>, mut p : P) -> bool where P : FnMut(&T, &T2) -> bool
    {
        for i in 0..N
        {
            if p(&self[i], &other[i]) == true { return true; }
        }
        false
    }

    /// True if the predicate is true for all component
    #[inline(always)]
    fn all_with<T2, P>(&self, other : &Self::WithType<T2>, mut p : P) -> bool where P : FnMut(&T, &T2) -> bool
    {
        for i in 0..N
        {
            if p(&self[i], &other[i]) == false { return false; }
        }
        true
    }

    
    /// Perform the min element component wise
    fn min_with(self, other: Self::WithType<T>) -> Self::WithType<T> where T:Copy + Ord
    {
        self.map_with(other, |a,b| a.min(b))
    }

    /// Perform the max element component wise
    fn max_with(self, other: Self::WithType<T>) -> Self::WithType<T> where T:Copy + Ord
    {
        self.map_with(other, |a,b| a.max(b))
    }
}
impl<T, const N:usize> ArrayWithGenericType<T,N> for [T;N]
{
    type WithType<T2> = [T2;N];
}



/// An array that is indexable, without any information if the type and the size are generic.
/// Check [ArrayWithGenericType] and [ArrayWithGenericSize] for editable generic type.
pub trait Array<T, const N : usize>:
    From <[T; N]> +
    Into <[T; N]> +
    Index<usize,Output = T> + IndexMut<usize,Output = T>
    // It's not using this, because it is not implemented for array
    // + AsRef<[T;N]> + AsMut<[T;N]>
{
    const DIMENSION : usize = N;

    #[inline(always)]
    fn splat(value : T) -> Self where T: Clone { Self::from(array::from_fn(|_| value.clone())) }

    
    /// True if the predicate is true for at least one component
    #[inline(always)]
    fn any<P>(&self, p : P) -> bool where P : FnMut(&T) -> bool { Iterator::any(&mut self.array().iter(), p) }

    
    /// True if the predicate is true for all component
    #[inline(always)]
    fn all<P>(&self, p : P) -> bool where P : FnMut(&T) -> bool { Iterator::all(&mut self.array().iter(), p) }


    #[inline(always)]
    fn for_each<F>(&self, f : F) where F : FnMut(&T) { self.array().iter().for_each(f); }
    #[inline(always)]
    fn for_each_mut<F>(&mut self, f : F) where F : FnMut(&mut T) { self.array_mut().iter_mut().for_each(f); }

    fn array(&self) -> &[T; N];
    fn array_mut(&mut self) -> &mut[T; N];

    #[inline(always)]
    fn slice(&self) -> &[T] { self.array().as_slice() }
    #[inline(always)]
    fn slice_mut(&mut self) -> &mut [T] { self.array_mut().as_mut_slice() }


    
    /// Fill non existing component with [Default]
    fn to_array_with_size<const M:usize>(self) -> [T; M] where T: Default
    {
        let mut it = <Self as Into<[T;N]>>::into(self).into_iter();
        <[T; M]>::from_fn(|_| it.next().unwrap_or_default())
    }

    /// Fill non existing component with the [Clone]d given value
    fn to_array_with_size_filled_with_value<const M:usize>(self, fill : T) -> [T; M] where T: Clone
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

impl<S,T,const N:usize> ArrayMin<T,N> for S where S:Array<T,N> {}
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