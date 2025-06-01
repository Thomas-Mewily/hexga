use std::ops::{Index, IndexMut};
use std::array;
use crate::*;

pub trait ArrayLike<T, const N : usize> :
    From <[T; N]> +
    Into <[T; N]> +
    Index<usize,Output = T> + IndexMut<usize,Output = T>
{
    const DIMENSION : usize = N;
    type WithType<T2> : ArrayLike<T2, N>;

    #[inline(always)]
    fn splat(value : T) -> Self where T: Clone { Self::from(array::from_fn(|_| value.clone())) }

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
    fn any<P>(&self, p : P) -> bool where P : FnMut(&T) -> bool { Iterator::any(&mut self.array().iter(), p) }

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
    fn all<P>(&self, p : P) -> bool where P : FnMut(&T) -> bool { Iterator::all(&mut self.array().iter(), p) }

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
}


impl<T, const N : usize> ArrayLike<T,N> for [T; N]
{
    type WithType<T2> = [T2; N];

    #[inline(always)]
    fn array(&self) -> &[T; N] { self }
    #[inline(always)]
    fn array_mut(&mut self) -> &mut[T; N] { self }
}

#[cfg(test)]
mod test_extension
{
    use super::*;

    #[test]
    fn map_with() {
        assert_eq!([1,2,3].map_with([3,2,1],|a,b| a+b), [4,4,4]);
    }
}

