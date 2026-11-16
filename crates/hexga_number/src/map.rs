use hexga_array::ArrayWithType;

use super::*;
use std::ops::RangeBounds;

/// Similar to [`Iterator::map`], but for container-like types that keep their shape.
pub trait MapIntern : Sized
{
    type Item;

    /// Apply `f` to each item, producing a new container of the same type.
    fn map_intern_elementwise<F, const N: usize>(f: F, this : [Self; N]) -> Self
    where
        F: FnMut([Self::Item; N]) -> Self::Item;

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
    fn map_intern_elementwise<F, const M: usize>(mut f: F, this : [Self; M]) -> Self
    where
        F: FnMut([Self::Item; M]) -> Self::Item 
    {
        Self::map_elementwise(f, this)
    }
}
impl<T,const N: usize> Map for [T;N]
{
    type WithType<R> = [R;N];

    fn map_elementwise<F, R, const M: usize>(f: F, this : [Self; M]) -> Self::WithType<R>
    where
        F: FnMut([Self::Item; M]) -> R {
        
        let this = core::mem::ManuallyDrop::new(this);

        core::array::from_fn(|i| {
            f(core::array::from_fn(|j| unsafe {
                // SAFETY: `this` is ManuallyDrop, so its elements are never
                // dropped by the compiler. We read `this[j][i]` exactly once for each (j, i) pair.
                // The pointer is valid because `this` is a live ManuallyDrop<[Self; N]>.
                core::ptr::read(&this[j][i])
            }))
        })
    }
}

impl<T> MapIntern for Vec<T>
{
    type Item=T;
    fn map_intern_elementwise<F, const M: usize>(mut f: F, this : [Self; M]) -> Self
    where
        F: FnMut([Self::Item; M]) -> Self::Item 
    {
        Self::map_elementwise(f, this)
    }
}
impl<T> Map for Vec<T>
{
    type WithType<R> = Vec<R>;

    fn map_elementwise<F, R, const M: usize>(f: F, this : [Self; M]) -> Self::WithType<R>
    where
        F: FnMut([Self::Item; M]) -> R {

        let mut len = 0;
        let mut iters = this.map(|v| { len = max(v.len(), 0); v.into_iter() });
        Vec::from_fn(len, |_| f(core::array::from_fn(|i| iters[i].next().unwrap())))
    }
}


/* 
/// Similar to [`Iterator::map`], but for container-like types that keep their shape.
pub trait MapIntern
{
    type Item;
    /// Apply `f` to each item, producing a new container of the same type.
    fn map_intern<F>(self, f: F) -> Self
    where
        F: FnMut(Self::Item) -> Self::Item;
}


/// Elementwise mapping of two containers of the same type.
pub trait MapInternWith: MapIntern
{
    /// Combine `self` and `other` elementwise with `f`.
    fn map_intern_with<F>(self, other: Self, f: F) -> Self
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item;
}

/// Similar to [`Iterator::map`]. Contrary to [`MapIntern`], it allows changing the item type.
pub trait Map: MapIntern
{
    type WithType<R>: Map<Item = R>;
    /// Apply `f` to each item, returning a container of the mapped values.
    fn map<R, F>(self, f: F) -> Self::WithType<R>
    where
        F: FnMut(Self::Item) -> R;
}

/*
// Update it with try_map once stabilized ?
// https://doc.rust-lang.org/std/primitive.array.html#method.try_map
//
/// `TryMap` allows the mapping function to return a fallible result.
pub trait TryMap: Map + Sized {
    /// Apply `f` to each item. If any `Try` fails, propagate the failure.
    #[inline(always)]
    fn try_map<O,E,F>(self, f: F) -> Result<Self::WithType<O>,E> where F: FnMut(Self::Item) -> Result<O,E>
    {
        let f = self.map(f);
    }
}
*/

/// Elementwise mapping of two containers with possibly different item types.
pub trait MapWith: Map + MapInternWith
{
    /// Combine `self` and `other` elementwise with `f`, producing a new container
    /// of the mapped values.
    fn map_with<R, Item2, F>(self, other: Self::WithType<Item2>, f: F) -> Self::WithType<R>
    where
        F: FnMut(Self::Item, Item2) -> R;
}

impl<T, const N: usize> MapIntern for [T; N]
{
    type Item = T;
    #[inline(always)]
    fn map_intern<F>(self, f: F) -> Self
    where
        F: FnMut(Self::Item) -> Self::Item,
    {
        self.map(f)
    }
}
impl<T, const N: usize> MapInternWith for [T; N]
{
    #[inline(always)]
    fn map_intern_with<F>(self, other: Self, f: F) -> Self
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        self.map_with(other, f)
    }
}
impl<T, const N: usize> Map for [T; N]
{
    type WithType<T2> = [T2; N];
    #[inline(always)]
    fn map<R, F>(self, f: F) -> Self::WithType<R>
    where
        F: FnMut(Self::Item) -> R,
    {
        self.map(f)
    }
}
impl<T, const N: usize> MapWith for [T; N]
{
    #[inline(always)]
    fn map_with<R, Item2, F>(self, other: Self::WithType<Item2>, mut f: F) -> Self::WithType<R>
    where
        F: FnMut(Self::Item, Item2) -> R,
    {
        let mut t1 = self.into_iter();
        let mut t2 = other.into_iter();
        std::array::from_fn(|_| f(unsafe { t1.next().unwrap_unchecked() }, unsafe { t2.next().unwrap_unchecked() }))
    }
}

impl<T> MapIntern for Vec<T>
{
    type Item = T;
    fn map_intern<F>(self, f: F) -> Self
    where
        F: FnMut(Self::Item) -> Self::Item,
    {
        self.map(f)
    }
}
impl<T> MapInternWith for Vec<T>
{
    fn map_intern_with<F>(self, other: Self, f: F) -> Self
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        self.map_with(other, f)
    }
}
impl<T> Map for Vec<T>
{
    type WithType<T2> = Vec<T2>;

    fn map<T2, F>(self, f: F) -> Self::WithType<T2>
    where
        F: FnMut(Self::Item) -> T2,
    {
        self.into_iter().map(f).collect()
    }
}
impl<T> MapWith for Vec<T>
{
    fn map_with<R, Item2, F>(self, other: Self::WithType<Item2>, mut f: F) -> Self::WithType<R>
    where
        F: FnMut(Self::Item, Item2) -> R,
    {
        self.into_iter().zip(other).map(|(a, b)| f(a, b)).collect()
    }
}
*/