use hexga_array::ArrayWithType;

use super::*;
use std::ops::RangeBounds;

// Update it and add `try_map` once stabilized ?
// https://doc.rust-lang.org/std/primitive.array.html#method.try_map

/// Similar to [`Iterator::map`], but for container-like types that keep their shape/type.
pub trait MapIntern : Sized
{
    type Item;

    /// Apply `f` to each item, producing a new container of the same type.
    fn map_intern_elementwise<F, const M: usize>(f: F, this : [Self; M]) -> Self
    where
        F: FnMut([Self::Item; M]) -> Self::Item;

    /// Apply `f` to each item, producing a new container of the same type.
    fn map_intern<F>(self, mut f: F) -> Self where F: FnMut(Self::Item) -> Self::Item { Self::map_intern_elementwise(|[a]| f(a), [self]) }

    /// Combine `self` and `other` elementwise with `f`.
    fn map_intern_with<F>(self, other: Self, mut f: F) -> Self
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item { Self::map_intern_elementwise(|[a, b]| f(a,b), [self, other]) }
}

/// Similar to [`Iterator::map`]. Contrary to [`MapIntern`], it allows changing the item type.
pub trait Map : MapIntern
{
    type WithType<R>: Map<Item = R>;

    /// Apply `f` to each item, returning a container of the mapped values.
    fn map_elementwise<F, R, const M: usize>(f: F, this : [Self; M]) -> Self::WithType<R>
    where
        F: FnMut([Self::Item; M]) -> R;

    /// Apply `f` to each item, producing a new container of the mapped values.
    fn map<F, R>(self, mut f: F) -> Self::WithType<R> where F: FnMut(Self::Item) -> R { Self::map_elementwise(|[a]| f(a), [self]) }

    /// Combine `self` and `other` elementwise with `f`, producing a new container
    /// of the mapped values.
    fn map_with<F, R>(self, other: Self, mut f: F) -> Self::WithType<R>
    where
        F: FnMut(Self::Item, Self::Item) -> R { Self::map_elementwise(|[a, b]| f(a,b), [self, other]) }
}

impl<T,const N: usize> MapIntern for [T;N]
{
    type Item=T;
    fn map_intern_elementwise<F, const M: usize>(f: F, this : [Self; M]) -> Self
    where
        F: FnMut([Self::Item; M]) -> Self::Item 
    {
        Self::map_elementwise(f, this)
    }
}
impl<T,const N: usize> Map for [T;N]
{
    type WithType<R> = [R;N];

    fn map_elementwise<F, R, const M: usize>(mut f: F, this : [Self; M]) -> Self::WithType<R>
    where
        F: FnMut([Self::Item; M]) -> R 
    {
        let mut iters = this.map(|v| v.into_iter());
        core::array::from_fn(|_| f(core::array::from_fn(|i| unsafe { iters[i].next().unwrap_unchecked() })))    
        /* 
        let this = core::mem::ManuallyDrop::new(this);

        core::array::from_fn(|i| {
            f(core::array::from_fn(|j| unsafe {
                // SAFETY: `this` is ManuallyDrop, so its elements are never
                // dropped by the compiler. We read `this[j][i]` exactly once for each (j, i) pair.
                // The pointer is valid because `this` is a live ManuallyDrop<[Self; N]>.
                core::ptr::read(&this[j][i])
            }))
        })
        */
    }
}

impl<T> MapIntern for Vec<T>
{
    type Item=T;
    fn map_intern_elementwise<F, const M: usize>(mut f: F, mut this : [Self; M]) -> Self
    where
        F: FnMut([Self::Item; M]) -> Self::Item 
    {
        if M == 0 { return Vec::new(); }

        Todo
// Reuse allocation here of this[0] + update GRID 

        let len = this.iter().map(Vec::len).min().unwrap();
        this[0].truncate(len);

        let mut len = usize::MAX;
        let mut iters = this.map(|v| { len = min(v.len(), len); v.into_iter() });
        Vec::from_fn(len, |_| f(core::array::from_fn(|i| unsafe { iters[i].next().unwrap_unchecked() })))
    }
}
impl<T> Map for Vec<T>
{
    type WithType<R> = Vec<R>;

    fn map_elementwise<F, R, const M: usize>(mut f: F, this : [Self; M]) -> Self::WithType<R>
    where
        F: FnMut([Self::Item; M]) -> R 
    {
        if M == 0 { return Vec::new(); }

        let mut len = usize::MAX;
        let mut iters = this.map(|v| { len = min(v.len(), len); v.into_iter() });
        Vec::from_fn(len, |_| f(core::array::from_fn(|i| unsafe { iters[i].next().unwrap_unchecked() })))
    }
}
