pub mod prelude
{
    pub use super::{Composite,CompositeGeneric};
}

pub trait Composite
{   
    type Inside;
    fn map_intern<F>(self, f: F) -> Self where F: FnMut(Self::Inside) -> Self::Inside;
}
// Can't auto impl Composte for every S that have GenericComposite, because can't express that Self::WithType<Self::Inside> = Self
pub trait CompositeGeneric
{
    type WithType<T2> : CompositeGeneric<Inside = T2>;
    type Inside;
    fn map<T2,F>(self, f: F) -> Self::WithType<T2> where F: FnMut(Self::Inside) -> T2;
}

/* 
// Zero, Prefix: Kilo Mega
pub trait CompositeConstant
{
    pub const fn splat(T: Copy) -> Self;
}
*/







impl<T, const N:usize> Composite for [T;N]
{
    type Inside=T;
    fn map_intern<F>(self, f: F) -> Self where F: FnMut(Self::Inside) -> Self::Inside 
    { 
        self.map(f)
    }
}
impl<T, const N:usize> CompositeGeneric for [T;N]
{
    type WithType<T2> = [T2;N];
    type Inside = T;
    fn map<T2,F>(self, f: F) -> Self::WithType<T2> where F: FnMut(Self::Inside) -> T2 { self.map(f) }
}

/* 
...

conflicting implementation for `[_; _]`
   |
   = note: downstream crates may implement trait `std::iter::FromIterator<<[_; _] as std::iter::IntoIterator>::Item>` for type `[_; _]`

impl<T,S> Composite for S where S:IntoIterator + FromIterator<S::Item>
{
    /* ... */
}
*/

impl<T> Composite for Vec<T>
{
    type Inside=T;
    fn map_intern<F>(self, mut f: F) -> Self where F: FnMut(Self::Inside) -> Self::Inside 
    { 
        self.into_iter().map(|v| f(v)).collect()
    }
}
impl<T> CompositeGeneric for Vec<T>
{
    type WithType<T2> = Vec<T2>;
    type Inside = T;

    fn map<T2,F>(self, mut f: F) -> Self::WithType<T2> where F: FnMut(Self::Inside) -> T2 {
        self.into_iter().map(|v| f(v)).collect()
    }
}
