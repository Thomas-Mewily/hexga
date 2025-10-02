use hexga_array::ArrayWithType;

use super::*;
use std::ops::RangeBounds;

pub trait Map
{   
    type Item;
    fn map_intern<F>(self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item;
    fn map_with_intern<F>(self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item;
}
pub trait MapGeneric : Map
{
    type WithType<R> : MapGeneric<Item = R>;
    fn map<R,F>(self, f: F) -> Self::WithType<R> where F: FnMut(Self::Item) -> R;
    fn map_with<R, Item2, F>(self, other: Self::WithType<Item2>, f : F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R;
}

impl<T, const N:usize> Map for [T;N]
{
    type Item=T;
    fn map_intern<F>(self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item 
    { 
        self.map(f)
    }
    
    fn map_with_intern<F>(self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item {
        self.map_with(other, f)
    }
}
impl<T, const N:usize> MapGeneric for [T;N]
{
    type WithType<T2> = [T2;N];
    fn map<R,F>(self, f: F) -> Self::WithType<R> where F: FnMut(Self::Item) -> R { self.map(f) }
    
    fn map_with<R, Item2, F>(self, other : Self::WithType<Item2>, mut f : F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R {
        let mut t1 = self.into_iter();
        let mut t2 = other.into_iter();
        std::array::from_fn(|_| f(t1.next().unwrap(), t2.next().unwrap()))
    }
}

impl<T> Map for Vec<T>
{
    type Item=T;
    fn map_intern<F>(self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item 
    { 
        self.map(f)
    }
    fn map_with_intern<F>(self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item 
    {
        self.map_with(other, f)
    }
}
impl<T> MapGeneric for Vec<T>
{
    type WithType<T2> = Vec<T2>;

    fn map<T2,F>(self, mut f: F) -> Self::WithType<T2> where F: FnMut(Self::Item) -> T2 {
        self.into_iter().map(|v| f(v)).collect()
    }
    
    fn map_with<R, Item2, F>(self, other : Self::WithType<Item2>, mut f : F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R {
        self.into_iter().zip(other).map(|(a,b)| f(a,b)).collect()
    }
}

