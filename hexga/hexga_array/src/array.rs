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
    
    fn splat(value : T) -> Self where T : Clone { Self::from(array::from_fn(|_| value.clone())) }

    fn map<U, F>(self, f : F) -> Self::WithType<U> where F : FnMut(T) -> U {  self.to_array().map(f).into() }

    fn map_with<U, T2, F>(self, other : Self::WithType<T2>, mut f : F) -> Self::WithType<U>
        where F : FnMut(T, T2) -> U
    {
        let mut t1_iter = self.to_array().into_iter();
        let mut t2_iter = other.to_array().into_iter();
        Self::WithType::<U>::from_array(std::array::from_fn(|_| f(t1_iter.next().unwrap(), t2_iter.next().unwrap())))
    }

    
    /// True if the predicate is true for at least one component
    fn any<P>(&self, p : P) -> bool where P : FnMut(&T) -> bool { Iterator::any(&mut self.array().iter(), p) }
    /// True if the predicate is true for at least one component
    fn any_with<T2, P>(&self, other : Self::WithType<T2>, mut p : P) -> bool where P : FnMut(&T, &T2) -> bool 
    { 
        for i in 0..N
        {
            if p(&self[i], &other[i]) == true { return true; }
        }
        false
    }

    /// True if the predicate is true for all component
    fn all<P>(&self, p : P) -> bool where P : FnMut(&T) -> bool { Iterator::all(&mut self.array().iter(), p) }
    /// True if the predicate is true for all component
    fn all_with<T2, P>(&self, other : &Self::WithType<T2>, mut p : P) -> bool where P : FnMut(&T, &T2) -> bool 
    { 
        for i in 0..N
        {
            if p(&self[i], &other[i]) == false { return false; }
        }
        true
    }

    fn for_each<F>(&self, f : F) where F : FnMut(&T) { self.array().iter().for_each(f); }
    fn for_each_mut<F>(&mut self, f : F) where F : FnMut(&mut T) { self.array_mut().iter_mut().for_each(f); }

    fn array(&self) -> &[T; N];
    fn array_mut(&mut self) -> &mut[T; N];

    fn slice(&self) -> &[T] { self.array().as_slice() }
    fn slice_mut(&mut self) -> &mut [T] { self.array_mut().as_mut_slice() }

    fn have_idx(&self, idx : usize) -> bool { idx < N }
    
    fn get(&self, idx : usize) -> Option<&T> { self.slice().get(idx) }
    fn get_mut(&mut self, idx : usize) -> Option<&mut T> { self.slice_mut().get_mut(idx) }

    /// Panics if the component don't exist
    fn set_or_panic(&mut self, idx : usize, val : T) -> &mut Self { *self.get_mut(idx).unwrap() = val; self }
    /// Do nothings if the component don't exist
    fn set(&mut self, idx : usize, val : T) -> &mut Self { self.get_mut(idx).map(|v| *v = val); self }

    fn try_set(&mut self, idx : usize, val : T) -> Result<(), T> 
    {
        match self.get_mut(idx)
        {
            Some(v) => { *v = val; Ok(()) },
            None => Err(val),
        }
    }

    /// Panics if the component don't exist
    fn with_or_panic(mut self, idx : usize, val : T) -> Self { self.set_or_panic(idx, val); self }
    /// Do nothings if the component don't exist
    fn with(mut self, idx : usize, val : T) -> Self { self.set(idx, val); self }

    /// Replace the component and return the old component. Panic if the component don't exist
    fn replace(&mut self, idx : usize, val : T) -> T { std::mem::replace(self.array_mut().get_mut(idx).unwrap(),val) }
}


impl<T, const N : usize> ArrayLike<T,N> for [T; N]
{
    type WithType<T2> = [T2; N];
    
    fn array(&self) -> &[T; N] { self }
    fn array_mut(&mut self) -> &mut[T; N] { self }
}
