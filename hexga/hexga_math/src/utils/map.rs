use hexga_array::ArrayWithType;

use super::*;
use std::ops::RangeBounds;

// Todo: Make 2 trait ? Map, MapWith ? Output type ?
// + impl it for reference &Self ? (like iterator)

/// Similar to [Iterator::map], but for container-like types that keep their shape.
pub trait MapIntern
{
    type Item;
    /// Apply `f` to each item, producing a new container of the same type.
    fn map_intern<F>(self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item;
}

/// Elementwise mapping of two containers of the same type.
pub trait MapWithIntern : MapIntern
{
    /// Combine `self` and `other` elementwise with `f`.
    fn map_with_intern<F>(self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item;
}



/// Similar to [Iterator::map]. Contrary to [MapIntern], it allows changing the item type.
pub trait Map : MapIntern
{
    type WithType<R> : Map<Item = R>;
    /// Apply `f` to each item, returning a container of the mapped values.
    fn map<R,F>(self, f: F) -> Self::WithType<R> where F: FnMut(Self::Item) -> R;
}
/// Elementwise mapping of two containers with possibly different item types.
pub trait MapWith : Map + MapWithIntern
{
    /// Combine `self` and `other` elementwise with `f`, producing a new container
    /// of the mapped values.
    fn map_with<R, Item2, F>(self, other: Self::WithType<Item2>, f : F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R;
}


impl<T, const N:usize> MapIntern for [T;N]
{
    type Item=T;
    #[inline(always)]
    fn map_intern<F>(self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item
    {
        self.map(f)
    }
}
impl<T, const N:usize> MapWithIntern for [T;N]
{

    #[inline(always)]
    fn map_with_intern<F>(self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item {
        self.map_with(other, f)
    }
}
impl<T, const N:usize> Map for [T;N]
{
    type WithType<T2> = [T2;N];
    #[inline(always)]
    fn map<R,F>(self, f: F) -> Self::WithType<R> where F: FnMut(Self::Item) -> R { self.map(f) }
}
impl<T, const N:usize> MapWith for [T;N]
{
    #[inline(always)]
    fn map_with<R, Item2, F>(self, other : Self::WithType<Item2>, mut f : F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R {
        let mut t1 = self.into_iter();
        let mut t2 = other.into_iter();
        std::array::from_fn(|_| f(t1.next().unwrap(), t2.next().unwrap()))
    }
}


impl<T> MapIntern for Vec<T>
{
    type Item=T;
    fn map_intern<F>(self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item
    {
        self.map(f)
    }
}
impl<T> MapWithIntern for Vec<T>
{
    fn map_with_intern<F>(self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item
    {
        self.map_with(other, f)
    }
}
impl<T> Map for Vec<T>
{
    type WithType<T2> = Vec<T2>;

    fn map<T2,F>(self, mut f: F) -> Self::WithType<T2> where F: FnMut(Self::Item) -> T2 {
        self.into_iter().map(|v| f(v)).collect()
    }
}
impl<T> MapWith for Vec<T>
{
    fn map_with<R, Item2, F>(self, other : Self::WithType<Item2>, mut f : F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R {
        self.into_iter().zip(other).map(|(a,b)| f(a,b)).collect()
    }
}
