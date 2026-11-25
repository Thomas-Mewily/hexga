//! Generalisation over collection, such as `get`, `get_mut`, `get_many_mut`, `swap`, `replace`, `set`...
//!
//! Each trait follow this convention when adding a new function `foo`  :
//!
//! `fn try_get_foo(...) -> Result<O,E>`
//!
//! `fn foo(...) -> Option<O>` (or `bool` instead of `Option<()>` when `try_get_foo` return a `Result<(), E>`)
//!
//! `fn foo_or_panic(...) -> O`
//!
//! `unsafe fn foo_unchecked(...) -> O`

use super::*;

/// A collection have a quick way to access each element.
pub trait Get<Idx> : CollectionIndex<Idx>
{
    /// Returns a reference to the value.
    fn get<'a>(&'a self, index: Idx) -> Option<Self::Ref<'a>> where Self::Output: 'a;

    /// Returns a reference to the value.
    #[inline(always)]
    #[track_caller]
    fn get_or_panic<'a>(&'a self, index: Idx) -> Self::Ref<'a> where Self::Output: 'a { self.get(index).expect("invalid index") }
    /// Returns a reference to the value.
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked<'a>(&'a self, index: Idx) -> Self::Ref<'a> where Self::Output: 'a { self.get(index).expect("invalid index") }

    /// True if `get(index)` return [Some], false otherwise.
    #[inline(always)]
    fn is_index_valid(&self, index: Idx) -> bool { self.get(index).is_some() }
    /// True if `get(index)` return [None], false otherwise.
    #[inline(always)]
    fn is_index_invalid(&self, index: Idx) -> bool { self.get(index).is_none() }

    fn contains(&self, index: Idx) -> bool { self.get(index).is_some() }
}

pub trait TryGet<Idx>: Get<Idx>
{
    type Error;
    /// Returns a reference to the value.
    fn try_get<'a>(&'a self, index: Idx) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a;
}

pub trait GetMut<Idx>: Get<Idx> + CollectionIndexMut<Idx>
{
    /// Returns a mutable reference to the value.
    fn get_mut<'a>(&'a mut self, index: Idx) -> Option<Self::RefMut<'a>> where Self::Output: 'a;
    #[inline(always)]
    #[track_caller]
    fn get_mut_or_panic<'a>(&'a mut self, index: Idx) -> Self::RefMut<'a> where Self::Output: 'a { self.get_mut(index).expect("invalid index") }
    /// Returns a mutable reference to the value.
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut<'a>(&'a mut self, index: Idx) -> Self::RefMut<'a> where Self::Output: 'a { self.get_mut(index).expect("invalid index") }

    /// Replace the value and return the old one.
    ///
    /// This operation is an [`involution`](https://en.wikipedia.org/wiki/Involution_(mathematics)):
    /// Replacing a value twice (first with a new value, then with the previously returned value) leaves the collection unchanged.
    #[inline(always)]
    fn replace(&mut self, index: Idx, value: Self::Output) -> Option<Self::Output>
    where
        Self::Output: Sized,
    {
        self.get_mut(index).map(|mut dest| core::mem::replace(&mut *dest, value))
    }
    /// Replace the value and return the old one.
    ///
    /// This operation is an [`involution`](https://en.wikipedia.org/wiki/Involution_(mathematics)):
    /// Replacing a value twice (first with a new value, then with the previously returned value) leaves the collection unchanged.
    #[inline(always)]
    #[track_caller]
    fn replace_or_panic(&mut self, index: Idx, value: Self::Output) -> Self::Output
    where
        Self::Output: Sized,
    {
        self.replace(index, value).expect("invalid index")
    }
    /// Replace the value and return the old one.
    ///
    /// This operation is an [`involution`](https://en.wikipedia.org/wiki/Involution_(mathematics)):
    /// Replacing a value twice (first with a new value, then with the previously returned value) leaves the collection unchanged.
    #[inline(always)]
    #[track_caller]
    unsafe fn replace_unchecked(&mut self, index: Idx, value: Self::Output) -> Self::Output
    where
        Self::Output: Sized,
    {
        let mut dest = unsafe { self.get_unchecked_mut(index) };
        core::mem::replace(&mut *dest, value)
    }

    /// Set the value and drop the previous one.
    #[inline(always)]
    fn set(&mut self, index: Idx, value: Self::Output) -> bool
    where
        Self::Output: Sized,
    {
        self.replace(index, value).map(|_| ()).is_some()
    }
    /// Set the value and drop the previous one.
    #[inline(always)]
    #[track_caller]
    fn set_or_panic(&mut self, index: Idx, value: Self::Output) -> &mut Self
    where
        Self::Output: Sized,
    {
        assert!(self.set(index, value), "invalid index");
        self
    }
    #[inline(always)]
    #[track_caller]
    unsafe fn set_unchecked(&mut self, index: Idx, value: Self::Output) -> &mut Self
    where
        Self::Output: Sized,
    {
        unsafe { self.replace_unchecked(index, value) };
        self
    }
}

pub trait TryGetMut<Idx>: TryGet<Idx> + GetMut<Idx>
{
    // Should TryGetMut have it own error type ?

    /// Returns a mutable reference to the value.
    fn try_get_mut<'a>(&'a mut self, index: Idx) -> Result<Self::RefMut<'a>, Self::Error> where Self::Output: 'a;

    /// Replace the value and return the old one.
    ///
    /// This operation is an [`involution`](https://en.wikipedia.org/wiki/Involution_(mathematics)):
    /// Replacing a value twice (first with a new value, then with the previously returned value) leaves the collection unchanged.
    #[inline(always)]
    fn try_replace(&mut self, index: Idx, value: Self::Output) -> Result<Self::Output, Self::Error>
    where
        Self::Output: Sized,
    {
        self.try_get_mut(index).map(|mut dest| core::mem::replace(&mut *dest, value))
    }

    /// Set the value and drop the previous one.
    #[inline(always)]
    fn try_set(&mut self, index: Idx, value: Self::Output) -> Result<(), Self::Error>
    where
        Self::Output: Sized,
    {
        self.try_replace(index, value).map(|_| ())
    }
}

pub type ManyMutError = core::slice::GetDisjointMutError;

pub trait GetManyMut<Idx>: GetMut<Idx>
{
    /// Returns mutable guards for multiple values.
    /// All values that can be accessed with the indices must be disjoint.
    #[doc(alias = "get_disjoint_mut")]
    fn get_many_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> Option<[Self::RefMut<'a>; N]>
    where Self::Output: 'a
    { self.try_get_many_mut(indices).ok() }

    fn try_get_many_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> Result<[Self::RefMut<'a>; N], ManyMutError>
    where Self::Output: 'a;

    /// Returns mutable guards for multiple values.
    /// All values that can be accessed with the indices must be disjoint.
    #[inline(always)]
    #[track_caller]
    #[doc(alias = "get_disjoint_mut_or_panic")]
    fn get_many_mut_or_panic<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> [Self::RefMut<'a>; N]
    where Self::Output: 'a
    { self.get_many_mut(indices).expect("invalid index") }

    /// Returns mutable guards for multiple values.
    /// All values that can be accessed with the indices must be disjoint.
    #[inline(always)]
    #[track_caller]
    #[doc(alias = "get_disjoint_unchecked_mut")]
    unsafe fn get_many_unchecked_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> [Self::RefMut<'a>; N]
    where Self::Output: 'a
    {
        self.get_many_mut(indices).expect("invalid index")
    }

    /// Swaps the values at two mutable locations, without deinitializing either one.
    ///
    /// Swap is symmetric: `foo.try_swap(a, b)` is equivalent to `foo.try_swap(b, a)` and vis versa
    ///
    /// Do nothings if some value  overlap or don't exist.
    #[inline(always)]
    fn swap(&mut self, a: Idx, b: Idx) -> bool
    where
        Self::Output: Sized,
    {
        self.get_many_mut([a, b]).map(|[mut a, mut b]| core::mem::swap(&mut *a, &mut *b)).is_some()
    }
    /// Swaps the values at two mutable locations, without deinitializing either one.
    ///
    /// Swap is symmetric: `foo.try_swap(a, b)` is equivalent to `foo.try_swap(b, a)` and vis versa
    ///
    /// Panics if any value overlap or don't exist
    #[inline(always)]
    #[track_caller]
    fn swap_or_panic(&mut self, a: Idx, b: Idx)
    where
        Self::Output: Sized,
    {
        assert!(self.swap(a, b), "invalid index")
    }
    /// Swaps the values at two mutable locations, without deinitializing either one.
    ///
    /// Swap is symmetric: `foo.try_swap(a, b)` is equivalent to `foo.try_swap(b, a)` and vis versa
    ///
    /// Do nothings if some value  overlap or don't exist.
    #[inline(always)]
    #[track_caller]
    unsafe fn swap_unchecked(&mut self, a: Idx, b: Idx)
    where
        Self::Output: Sized,
    {
        let [mut a, mut b] = unsafe { self.get_many_unchecked_mut([a, b]) };
        core::mem::swap(&mut *a, &mut *b);
    }
}

mod wrap_std_nightly_only_feature_to_stay_in_stable
{
    use super::{GetManyMut, ManyMutError};

    /*
    /// To wrap [core::slice::GetDisjointMutIndex](https://doc.rust-lang.org/core/slice/trait.GetDisjointMutIndex.html) because it's nightly-only experimental API...
    pub(crate) unsafe trait DisjointIndex: Clone
    {
        fn is_in_bounds(&self, len: usize) -> bool;
        fn is_overlapping(&self, other: &Self) -> bool;
    }

    // SAFETY: We implement `is_in_bounds()` and `is_overlapping()` correctly.
    unsafe impl DisjointIndex for usize {
        #[inline]
        fn is_in_bounds(&self, len: usize) -> bool {
            *self < len
        }

        #[inline]
        fn is_overlapping(&self, other: &Self) -> bool {
            *self == *other
        }
    }

    // SAFETY: We implement `is_in_bounds()` and `is_overlapping()` correctly.
    unsafe impl DisjointIndex for core::ops::Range<usize> {
        #[inline]
        fn is_in_bounds(&self, len: usize) -> bool {
            (self.start <= self.end) & (self.end <= len)
        }

        #[inline]
        fn is_overlapping(&self, other: &Self) -> bool {
            (self.start < other.end) & (other.start < self.end)
        }
    }

    // SAFETY: We implement `is_in_bounds()` and `is_overlapping()` correctly.
    unsafe impl DisjointIndex for core::ops::RangeInclusive<usize> {
        #[inline]
        fn is_in_bounds(&self, len: usize) -> bool {
            (self.start() <= self.end()) & (*self.end() < len)
        }

        #[inline]
        fn is_overlapping(&self, other: &Self) -> bool {
            (self.start() <= other.end()) & (other.start() <= self.end())
        }
    }

    // SAFETY: We implement `is_in_bounds()` and `is_overlapping()` correctly.
    unsafe impl DisjointIndex for core::range::Range<usize> {
        #[inline]
        fn is_in_bounds(&self, len: usize) -> bool {
            core::ops::Range::from(*self).is_in_bounds(len)
        }

        #[inline]
        fn is_overlapping(&self, other: &Self) -> bool {
            core::ops::Range::from(*self).is_overlapping(&core::ops::Range::from(*other))
        }
    }

    // SAFETY: We implement `is_in_bounds()` and `is_overlapping()` correctly.
    unsafe impl DisjointIndex for core::range::RangeInclusive<usize> {
        #[inline]
        fn is_in_bounds(&self, len: usize) -> bool {
            core::ops::RangeInclusive::from(*self).is_in_bounds(len)
        }

        #[inline]
        fn is_overlapping(&self, other: &Self) -> bool {
            core::ops::RangeInclusive::from(*self).is_overlapping(&core::ops::RangeInclusive::from(*other))
        }
    }
    */

    type StdOpsRange = core::ops::Range<usize>;
    type StdRangeRange = core::range::Range<usize>;
    type StdOpsRangeInclusive = core::ops::RangeInclusive<usize>;
    type StdRangeRangeInclusive = core::range::RangeInclusive<usize>;

    hexga_map_on::map_on!(
        (
            StdOpsRange,
            StdRangeRange,
            StdOpsRangeInclusive,
            StdRangeRangeInclusive,
            usize
        ),
        ($type_name:ty) =>
        {
            impl<T> GetManyMut<$type_name> for [T]
            {
                #[inline(always)]
                fn get_many_mut<'a, const N: usize>(&'a mut self, indices: [$type_name; N]) -> Option<[Self::RefMut<'a>; N]>
                 where Self::Output: 'a{
                    <[T]>::get_disjoint_mut(self, indices).ok()
                }

                #[inline(always)]
                fn try_get_many_mut<'a, const N: usize>(&'a mut self, indices: [$type_name; N]) -> Result<[Self::RefMut<'a>; N], ManyMutError>
                 where Self::Output: 'a{
                    <[T]>::get_disjoint_mut(self, indices)
                }

                #[inline(always)]
                #[track_caller]
                unsafe fn get_many_unchecked_mut<'a, const N: usize>(&'a mut self, indices: [$type_name; N]) -> [Self::RefMut<'a>; N]
                where Self::Output: 'a {
                    unsafe { <[T]>::get_disjoint_unchecked_mut(self, indices) }
                }
            }
        }
    );
}

/*
pub struct TypedIndex<T,Idx>
{
    pub index: Idx,
    target: PhantomData<T>,
}
impl<T,Idx> Clone for TypedIndex<T,Idx>
    where Idx: Clone
{
    fn clone(&self) -> Self { Self { index: self.index.clone(), target: PhantomData } }
}
// type parameter `C` must be covered by another type when it appears before the first local type ...
impl<T,Idx,C> Index<TypedIndex<T,Idx>> for C
    where C: Index<Idx,Output = T>
{

}
impl<T,Idx,C> Get<TypedIndex<T,Idx>> for C
    where C: Get<Idx,Output = T>
{
    type Output=T;
    fn get(&self, index: TypedIndex<T,Idx>) -> Option<Self::Guard<'a>> where Self::Output: 'a { self.get(index.index) }
}
*/

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct IndexInvalid<Idx = usize>
{
    pub index: Idx,
}
impl<Idx> IndexInvalid<Idx>
{
    pub fn new(index: Idx) -> Self { Self { index } }
}

pub type IndexOutOfRange<Idx = usize, B = core::ops::Range<Idx>> = IndexOutOfBounds<Idx, B>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct IndexOutOfBounds<Idx = usize, B = core::ops::Range<Idx>>
{
    pub index: Idx,
    pub bound: B,
}
impl<Idx, B> IndexOutOfBounds<Idx, B>
{
    pub fn new(index: Idx, bound: B) -> Self { Self { index, bound } }
}

/// Index for the `Ok` payload of a `Result`.
///
/// Bracket indexing panics if the result is `Err`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResultOk;

/// Index for the `Err` payload of a `Result`.
///
/// Bracket indexing panics if the result is `Ok`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResultErr;

impl<T, E> CollectionIndex<ResultOk> for Result<T, E>
{
    type Output = T;
    type Ref<'a> = &'a T where Self: 'a;
}
impl<T, E> CollectionIndexMut<ResultOk> for Result<T, E>
{
    type RefMut<'a> = &'a mut T where Self: 'a;
}
impl<T, E> Get<ResultOk> for Result<T, E>
{
    fn get<'a>(&'a self, _: ResultOk) -> Option<Self::Ref<'a>> where T: 'a { self.as_ref().ok() }
}
impl<T, E> GetMut<ResultOk> for Result<T, E>
{
    fn get_mut<'a>(&'a mut self, _: ResultOk) -> Option<Self::RefMut<'a>> where T: 'a { self.as_mut().ok() }
}
impl<T, E> Index<ResultOk> for Result<T, E>
{
    type Output = T;

    #[track_caller]
    fn index(&self, index: ResultOk) -> &T { Get::get_or_panic(self, index) }
}
impl<T, E> core::ops::IndexMut<ResultOk> for Result<T, E>
{
    #[track_caller]
    fn index_mut(&mut self, index: ResultOk) -> &mut T { GetMut::get_mut_or_panic(self, index) }
}

impl<T, E> CollectionIndex<ResultErr> for Result<T, E>
{
    type Output = E;
    type Ref<'a> = &'a E where Self: 'a;
}
impl<T, E> CollectionIndexMut<ResultErr> for Result<T, E>
{
    type RefMut<'a> = &'a mut E where Self: 'a;
}
impl<T, E> Get<ResultErr> for Result<T, E>
{
    fn get<'a>(&'a self, _: ResultErr) -> Option<Self::Ref<'a>> where E: 'a { self.as_ref().err() }
}
impl<T, E> GetMut<ResultErr> for Result<T, E>
{
    fn get_mut<'a>(&'a mut self, _: ResultErr) -> Option<Self::RefMut<'a>> where E: 'a { self.as_mut().err() }
}
impl<T, E> Index<ResultErr> for Result<T, E>
{
    type Output = E;

    #[track_caller]
    fn index(&self, index: ResultErr) -> &E { Get::get_or_panic(self, index) }
}
impl<T, E> core::ops::IndexMut<ResultErr> for Result<T, E>
{
    #[track_caller]
    fn index_mut(&mut self, index: ResultErr) -> &mut E { GetMut::get_mut_or_panic(self, index) }
}

impl<Idx, T> Get<Idx> for [T]
where
    Idx: SliceIndex<[T]>,
    [T]: Collection<Idx::Output>,
{
    #[inline(always)]
    fn get<'a>(&'a self, index: Idx) -> Option<Self::Ref<'a>> where Self::Output: 'a { self.get(index) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked<'a>(&'a self, index: Idx) -> Self::Ref<'a> where Self::Output: 'a { unsafe { self.get_unchecked(index) } }
}
impl<Idx, T> Get<Idx> for &[T]
where
    Idx: SliceIndex<[T]>,
    [T]: Collection<Idx::Output>,
{
    #[inline(always)]
    fn get<'a>(&'a self, index: Idx) -> Option<Self::Ref<'a>> where Self::Output: 'a { (*self).get(index) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked<'a>(&'a self, index: Idx) -> Self::Ref<'a> where Self::Output: 'a { unsafe { (*self).get_unchecked(index) } }
}
impl<Idx, T> Get<Idx> for &mut [T]
where
    Idx: SliceIndex<[T]>,
    [T]: Collection<Idx::Output>,
{
    #[inline(always)]
    fn get<'a>(&'a self, index: Idx) -> Option<Self::Ref<'a>> where Self::Output: 'a { (**self).get(index) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked<'a>(&'a self, index: Idx) -> Self::Ref<'a> where Self::Output: 'a { unsafe { (**self).get_unchecked(index) } }
}
impl<Idx, T> TryGet<Idx> for [T]
where
    Idx: SliceIndex<[T]> + Clone,
    [T]: Collection<Idx::Output>,
{
    type Error = IndexOutOfBounds<Idx, Range<usize>>;
    #[inline(always)]
    fn try_get<'a>(&'a self, index: Idx) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a { self.get(index.clone()).ok_or(IndexOutOfBounds { index, bound: 0..self.len() }) }
}
impl<Idx, T> TryGet<Idx> for &[T]
where
    Idx: SliceIndex<[T]> + Clone,
    [T]: Collection<Idx::Output>,
{
    type Error = IndexOutOfBounds<Idx, Range<usize>>;
    #[inline(always)]
    fn try_get<'a>(&'a self, index: Idx) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a { (*self).try_get(index) }
}
impl<Idx, T> TryGet<Idx> for &mut [T]
where
    Idx: SliceIndex<[T]> + Clone,
    [T]: Collection<Idx::Output>,
{
    type Error = IndexOutOfBounds<Idx, Range<usize>>;
    #[inline(always)]
    fn try_get<'a>(&'a self, index: Idx) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a { (**self).try_get(index) }
}

impl<Idx, T> GetMut<Idx> for [T]
where
    Idx: SliceIndex<[T]>,
    [T]: CollectionMut<Idx::Output>,
{
    #[inline(always)]
    fn get_mut<'a>(&'a mut self, index: Idx) -> Option<Self::RefMut<'a>> where Self::Output: 'a { self.get_mut(index) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut<'a>(&'a mut self, index: Idx) -> Self::RefMut<'a> where Self::Output: 'a { unsafe { self.get_unchecked_mut(index) } }
}
impl<Idx, T> GetMut<Idx> for &mut [T]
where
    Idx: SliceIndex<[T]>,
    [T]: CollectionMut<Idx::Output>,
{
    #[inline(always)]
    fn get_mut<'a>(&'a mut self, index: Idx) -> Option<Self::RefMut<'a>> where Self::Output: 'a { (*self).get_mut(index) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut<'a>(&'a mut self, index: Idx) -> Self::RefMut<'a> where Self::Output: 'a { unsafe { (*self).get_unchecked_mut(index) } }
}
impl<Idx, T> TryGetMut<Idx> for [T]
where
    Idx: SliceIndex<[T]> + Clone,
    [T]: CollectionMut<Idx::Output>,
{
    #[inline(always)]
    fn try_get_mut<'a>(&'a mut self, index: Idx) -> Result<Self::RefMut<'a>, Self::Error> where Self::Output: 'a
    {
        let len = self.len();
        self.get_mut(index.clone()).ok_or(IndexOutOfBounds { index, bound: 0..len })
    }
}
/*
impl<Idx, T> TryGetMut<Idx> for &mut [T]
where
    Idx: SliceIndex<[T]> + Clone,
    [T]: CollectionMut<Idx::Output>,
{
    #[inline(always)]
    fn try_get_mut<'a>(&'a mut self, index: Idx) -> Result<Self::RefMut<'a>, Self::Error> where Self::Output: 'a { (**self).try_get_mut(index) }
}
*/
impl<'s, Idx, T> TryGetMut<Idx> for &'s mut [T]
where
    Idx: SliceIndex<[T]> + Clone,
    [T]: CollectionMut<Idx::Output>,
{
    #[inline(always)]
    fn try_get_mut<'a>(&'a mut self, index: Idx) -> Result<Self::RefMut<'a>, Self::Error> where Self::Output: 'a { (**self).try_get_mut(index) }
}
/*
impl<Idx, T> GetManyMut<Idx> for [T]
where
    Idx: SliceIndex<[T]> + DisjointIndex,
    [T]: CollectionMut<Idx::Output>,
{
    #[inline(always)]
    fn get_many_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> Option<[Self::RefMut<'a>; N]>  where Self::Output: 'a{ self.get_disjoint_mut(indices).ok() }

    #[inline(always)]
    fn try_get_many_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> Result<[Self::RefMut<'a>; N], ManyMutError>  where Self::Output: 'a{ self.get_disjoint_mut(indices) }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> [Self::RefMut<'a>; N]
    where Self::Output: 'a {
        unsafe { self.get_disjoint_unchecked_mut(indices) }
    }
}
*/

impl<Idx, T> GetManyMut<Idx> for &mut [T]
where
    Idx: SliceIndex<[T]>,
    [T]: GetManyMut<Idx> + CollectionMut<Idx::Output>,
{
    #[inline(always)]
    fn get_many_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> Option<[Self::RefMut<'a>; N]>  where Self::Output: 'a{ <[T] as GetManyMut<Idx>>::get_many_mut(*self, indices) }

    #[inline(always)]
    fn try_get_many_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> Result<[Self::RefMut<'a>; N], ManyMutError>  where Self::Output: 'a{ (*self).try_get_many_mut(indices) }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> [Self::RefMut<'a>; N]
    where Self::Output: 'a {
        unsafe { (*self).get_many_unchecked_mut(indices) }
    }
}

impl<Idx, T, const N: usize> TryGet<Idx> for [T; N]
where
    [T]: TryGet<Idx>,
{
    type Error = <[T] as TryGet<Idx>>::Error;
    #[inline(always)]
    fn try_get<'a>(&'a self, index: Idx) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a { TryGet::try_get(self.as_slice(), index) }
}

impl<T, const N: usize, Output: ?Sized> Collection<Output> for [T; N] where [T]: Collection<Output>
{
    type Ref<'a> = <[T] as Collection<Output>>::Ref<'a> where Self: 'a, Output: 'a;
}
impl<T, const N: usize, Output: ?Sized> CollectionMut<Output> for [T; N] where [T]: CollectionMut<Output>
{
    type RefMut<'a> = <[T] as CollectionMut<Output>>::RefMut<'a> where Self: 'a, Output: 'a;
}

impl<Idx, T, const N: usize> CollectionIndex<Idx> for [T; N]
where
    [T]: CollectionIndex<Idx>,
{
    type Output = <[T] as CollectionIndex<Idx>>::Output;
    type Ref<'a> = <[T] as CollectionIndex<Idx>>::Ref<'a> where Self: 'a, Self::Output: 'a;
}
impl<Idx, T, const N: usize> Get<Idx> for [T; N]
where
    [T]: Get<Idx>,
{
    #[inline(always)]
    fn get<'a>(&'a self, index: Idx) -> Option<Self::Ref<'a>> where Self::Output: 'a { Get::get(self.as_slice(), index) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked<'a>(&'a self, index: Idx) -> Self::Ref<'a> where Self::Output: 'a { unsafe { Get::get_unchecked(self.as_slice(), index) } }
}

impl<Idx, T, const N: usize> TryGetMut<Idx> for [T; N]
where
    [T]: TryGetMut<Idx>,
{
    #[inline(always)]
    fn try_get_mut<'a>(&'a mut self, index: Idx) -> Result<Self::RefMut<'a>, Self::Error> where Self::Output: 'a { TryGetMut::try_get_mut(self.as_mut_slice(), index) }
}

impl<Idx, T, const N: usize> CollectionIndexMut<Idx> for [T; N]
where
    [T]: CollectionIndexMut<Idx>,
{
    type RefMut<'a> = <[T] as CollectionIndexMut<Idx>>::RefMut<'a> where Self: 'a, Self::Output: 'a;
}
impl<Idx, T, const N: usize> GetMut<Idx> for [T; N]
where
    [T]: GetMut<Idx>,
{
    #[inline(always)]
    fn get_mut<'a>(&'a mut self, index: Idx) -> Option<Self::RefMut<'a>> where Self::Output: 'a { GetMut::get_mut(self.as_mut_slice(), index) }
    #[inline(always)]
    unsafe fn get_unchecked_mut<'a>(&'a mut self, index: Idx) -> Self::RefMut<'a> where Self::Output: 'a { unsafe { GetMut::get_unchecked_mut(self.as_mut_slice(), index) } }
}
impl<Idx, T, const N: usize> GetManyMut<Idx> for [T; N]
where
    [T]: GetManyMut<Idx>,
{
    #[inline(always)]
    fn get_many_mut<'a, const N2: usize>(&'a mut self, indices: [Idx; N2]) -> Option<[Self::RefMut<'a>; N2]>
     where Self::Output: 'a{
        GetManyMut::get_many_mut(self.as_mut_slice(), indices)
    }

    #[inline(always)]
    fn try_get_many_mut<'a, const N2: usize>(&'a mut self, indices: [Idx; N2]) -> Result<[Self::RefMut<'a>; N2], ManyMutError>
     where Self::Output: 'a{
        GetManyMut::try_get_many_mut(self.as_mut_slice(), indices)
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<'a, const N2: usize>(&'a mut self, indices: [Idx; N2]) -> [Self::RefMut<'a>; N2]
    where Self::Output: 'a {
        unsafe { GetManyMut::get_many_unchecked_mut(self.as_mut_slice(), indices) }
    }
}

impl<Idx, T, A: Allocator> TryGet<Idx> for Vec<T, A>
where
    [T]: TryGet<Idx>,
{
    type Error = <[T] as TryGet<Idx>>::Error;
    #[inline(always)]
    fn try_get<'a>(&'a self, index: Idx) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a { TryGet::try_get(self.as_slice(), index) }
}
impl<T, Output: ?Sized, A: Allocator> Collection<Output> for Vec<T, A> where [T]: Collection<Output>
{
    type Ref<'a> = <[T] as Collection<Output>>::Ref<'a> where Self: 'a, Output: 'a;
}
impl<T, Output: ?Sized, A: Allocator> CollectionMut<Output> for Vec<T, A> where [T]: CollectionMut<Output>
{
    type RefMut<'a> = <[T] as CollectionMut<Output>>::RefMut<'a> where Self: 'a, Output: 'a;
}

impl<Idx, T, A: Allocator> CollectionIndex<Idx> for Vec<T, A>
where
    [T]: CollectionIndex<Idx>,
{
    type Output = <[T] as CollectionIndex<Idx>>::Output;
    type Ref<'a> = <[T] as CollectionIndex<Idx>>::Ref<'a> where Self: 'a, Self::Output: 'a;
}
impl<Idx, T, A: Allocator> Get<Idx> for Vec<T, A>
where
    [T]: Get<Idx>,
{
    #[inline(always)]
    fn get<'a>(&'a self, index: Idx) -> Option<Self::Ref<'a>> where Self::Output: 'a { Get::get(self.as_slice(), index) }
    #[track_caller]
    #[inline(always)]
    unsafe fn get_unchecked<'a>(&'a self, index: Idx) -> Self::Ref<'a> where Self::Output: 'a { unsafe { Get::get_unchecked(self.as_slice(), index) } }
}
impl<Idx, T, A: Allocator> TryGetMut<Idx> for Vec<T, A>
where
    [T]: TryGetMut<Idx>,
{
    #[inline(always)]
    fn try_get_mut<'a>(&'a mut self, index: Idx) -> Result<Self::RefMut<'a>, Self::Error> where Self::Output: 'a { TryGetMut::try_get_mut(self.as_mut_slice(), index) }
}
impl<Idx, T, A: Allocator> CollectionIndexMut<Idx> for Vec<T, A>
where
    [T]: CollectionIndexMut<Idx>,
{
    type RefMut<'a> = <[T] as CollectionIndexMut<Idx>>::RefMut<'a> where Self: 'a, Self::Output: 'a;
}
impl<Idx, T, A: Allocator> GetMut<Idx> for Vec<T, A>
where
    [T]: GetMut<Idx>,
{
    #[inline(always)]
    fn get_mut<'a>(&'a mut self, index: Idx) -> Option<Self::RefMut<'a>> where Self::Output: 'a { GetMut::get_mut(self.as_mut_slice(), index) }
    #[inline(always)]
    unsafe fn get_unchecked_mut<'a>(&'a mut self, index: Idx) -> Self::RefMut<'a> where Self::Output: 'a { unsafe { GetMut::get_unchecked_mut(self.as_mut_slice(), index) } }
}
impl<Idx, T, A: Allocator> GetManyMut<Idx> for Vec<T, A>
where
    [T]: GetManyMut<Idx>,
{
    #[track_caller]
    #[inline(always)]
    unsafe fn get_many_unchecked_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> [Self::RefMut<'a>; N]
    where Self::Output: 'a {
        unsafe { GetManyMut::get_many_unchecked_mut(self.as_mut_slice(), indices) }
    }

    #[inline(always)]
    fn try_get_many_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> Result<[Self::RefMut<'a>; N], ManyMutError>
     where Self::Output: 'a{
        GetManyMut::try_get_many_mut(self.as_mut_slice(), indices)
    }
    #[inline(always)]
    fn get_many_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> Option<[Self::RefMut<'a>; N]>  where Self::Output: 'a{ GetManyMut::get_many_mut(self.as_mut_slice(), indices) }
}

impl<T> TryGet<usize> for VecDeque<T>
{
    type Error = IndexOutOfBounds<usize, Range<usize>>;
    #[inline(always)]
    fn try_get<'a>(&'a self, index: usize) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a { self.get(index).ok_or(IndexOutOfBounds { index, bound: 0..self.len() }) }
}
impl<T> Collection<T> for VecDeque<T>
{
    type Ref<'a> = &'a T where Self: 'a, T: 'a;
}
impl<T> CollectionMut<T> for VecDeque<T>
{
    type RefMut<'a> = &'a mut T where Self: 'a, T: 'a;
}

impl<T> CollectionIndex<usize> for VecDeque<T>
{
    type Output = <Self as Index<usize>>::Output;
    type Ref<'a> = &'a Self::Output where Self: 'a, Self::Output: 'a;
}
impl<T> Get<usize> for VecDeque<T>
{
    #[inline(always)]
    fn get<'a>(&'a self, index: usize) -> Option<Self::Ref<'a>> where Self::Output: 'a { self.get(index) }
}
impl<T> TryGetMut<usize> for VecDeque<T>
{
    #[inline(always)]
    fn try_get_mut<'a>(&'a mut self, index: usize) -> Result<Self::RefMut<'a>, Self::Error> where Self::Output: 'a
    {
        let len = self.len();
        self.get_mut(index).ok_or(IndexOutOfBounds { index, bound: 0..len })
    }
}
impl<T> CollectionIndexMut<usize> for VecDeque<T>
{
    type RefMut<'a> = &'a mut Self::Output where Self: 'a, Self::Output: 'a;
}
impl<T> GetMut<usize> for VecDeque<T>
{
    #[inline(always)]
    fn get_mut<'a>(&'a mut self, index: usize) -> Option<Self::RefMut<'a>> where Self::Output: 'a { self.get_mut(index) }
}

impl<T> GetManyMut<usize> for VecDeque<T>
{
    #[track_caller]
    #[inline(always)]
    unsafe fn get_many_unchecked_mut<'a, const N: usize>(&'a mut self, indices: [usize; N]) -> [Self::RefMut<'a>; N]
    where Self::Output: 'a {
        unsafe {
            // Can probably be improved using `self.as_mut_slices()`
            self.make_contiguous().get_disjoint_unchecked_mut(indices)
        }
    }
    #[inline(always)]
    fn try_get_many_mut<'a, const N: usize>(&'a mut self, indices: [usize; N]) -> Result<[Self::RefMut<'a>; N], ManyMutError>
     where Self::Output: 'a{
        self.make_contiguous().try_get_many_mut(indices)
    }
    #[inline(always)]
    fn get_many_mut<'a, const N: usize>(&'a mut self, indices: [usize; N]) -> Option<[Self::RefMut<'a>; N]>  where Self::Output: 'a{ self.make_contiguous().get_many_mut(indices) }
}

impl Collection<char> for str
{
    type Ref<'a> = Identity<char> where Self: 'a, char: 'a;
}
impl Collection<str> for str
{
    type Ref<'a> = &'a str where Self: 'a, str: 'a;
}
impl CollectionMut<str> for str
{
    type RefMut<'a> = &'a mut str where Self: 'a, str: 'a;
}

/// A `usize` index counts Unicode scalar values from the start of the string.
impl CollectionIndex<usize> for str
{
    type Output = char;
    type Ref<'a> = Identity<char> where Self: 'a;
}

impl Get<usize> for str
{
    fn get<'a>(&'a self, index: usize) -> Option<Self::Ref<'a>> where Self::Output: 'a
    {
        self.chars().nth(index).map(Identity)
    }
}

impl TryGet<usize> for str
{
    type Error = IndexOutOfBounds;

    fn try_get<'a>(&'a self, index: usize) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a
    {
        Get::get(self, index).ok_or_else(|| IndexOutOfBounds::new(index, 0..self.chars().count()))
    }
}

// Range indices retain the standard UTF-8 byte boundaries.
macro_rules! impl_str_range_access
{
    ($($idx:ty),* $(,)?) => {$(
        impl CollectionIndex<$idx> for str
        {
            type Output = str;
            type Ref<'a> = &'a str where Self: 'a;
        }
        impl CollectionIndexMut<$idx> for str
        {
            type RefMut<'a> = &'a mut str where Self: 'a;
        }
        impl Get<$idx> for str
        {
            fn get<'a>(&'a self, index: $idx) -> Option<Self::Ref<'a>> where Self::Output: 'a { str::get(self, index) }

            #[track_caller]
            unsafe fn get_unchecked<'a>(&'a self, index: $idx) -> Self::Ref<'a> where Self::Output: 'a
            {
                unsafe { str::get_unchecked(self, index) }
            }
        }
        impl TryGet<$idx> for str
        {
            type Error = IndexOutOfBounds<$idx, Range<usize>>;

            fn try_get<'a>(&'a self, index: $idx) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a
            {
                str::get(self, index.clone()).ok_or(IndexOutOfBounds::new(index, 0..self.len()))
            }
        }
    )*};
}

impl_str_range_access!(
    core::ops::RangeFull,
    core::ops::Range<usize>,
    core::ops::RangeFrom<usize>,
    core::ops::RangeTo<usize>,
    core::ops::RangeInclusive<usize>,
    core::ops::RangeToInclusive<usize>,
    core::range::Range<usize>,
    core::range::RangeFrom<usize>,
    core::range::RangeInclusive<usize>,
    core::range::RangeToInclusive<usize>,
    (core::ops::Bound<usize>, core::ops::Bound<usize>)
);

impl<Output: ?Sized> Collection<Output> for String where str: Collection<Output>
{
    type Ref<'a> = <str as Collection<Output>>::Ref<'a> where Self: 'a, Output: 'a;
}
impl<Output: ?Sized> CollectionMut<Output> for String where str: CollectionMut<Output>
{
    type RefMut<'a> = <str as CollectionMut<Output>>::RefMut<'a> where Self: 'a, Output: 'a;
}

impl<Idx> CollectionIndex<Idx> for String where str: CollectionIndex<Idx>
{
    type Output = <str as CollectionIndex<Idx>>::Output;
    type Ref<'a> = <str as CollectionIndex<Idx>>::Ref<'a> where Self: 'a, Self::Output: 'a;
}

impl<Idx> CollectionIndexMut<Idx> for String where str: CollectionIndexMut<Idx>
{
    type RefMut<'a> = <str as CollectionIndexMut<Idx>>::RefMut<'a> where Self: 'a, Self::Output: 'a;
}

macro_rules! forward_str_access
{
    ($($ty:ty),* $(,)?) => {$(
        impl<Idx> Get<Idx> for $ty where str: Get<Idx>
        {
            fn get<'a>(&'a self, index: Idx) -> Option<Self::Ref<'a>> where Self::Output: 'a { Get::get(&**self, index) }

            #[track_caller]
            unsafe fn get_unchecked<'a>(&'a self, index: Idx) -> Self::Ref<'a> where Self::Output: 'a
            {
                unsafe { Get::get_unchecked(&**self, index) }
            }
        }

        impl<Idx> TryGet<Idx> for $ty where str: TryGet<Idx>
        {
            type Error = <str as TryGet<Idx>>::Error;

            fn try_get<'a>(&'a self, index: Idx) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a
            {
                TryGet::try_get(&**self, index)
            }
        }
    )*};
}

forward_str_access!(&str, &mut str, String);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub struct MissingKey<K = ()>
{
    pub key: K,
}
impl<K> From<K> for MissingKey<K>
{
    fn from(key: K) -> Self { MissingKey { key } }
}
impl<K> MissingKey<K>
{
    pub fn new(key: K) -> Self { Self { key } }
}

#[cfg(feature = "std")]
impl<K, V, S, Q> TryGet<&Q> for HashMap<K, V, S>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq + Clone,
    K: Eq + Hash,
    S: BuildHasher,
{
    type Error = MissingKey<Q>;

    fn try_get<'a>(&'a self, key: &Q) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a { self.get(key).ok_or_else(|| MissingKey::new(key.clone())) }
}

#[cfg(feature = "std")]
impl<K, V, S> Collection<V> for HashMap<K, V, S>
{
    type Ref<'a> = &'a V where Self: 'a, V: 'a;
}
#[cfg(feature = "std")]
impl<K, V, S> CollectionMut<V> for HashMap<K, V, S>
{
    type RefMut<'a> = &'a mut V where Self: 'a, V: 'a;
}

#[cfg(feature = "std")]
impl<K, V, S, Q> CollectionIndex<&Q> for HashMap<K, V, S>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    K: Eq + Hash,
    S: BuildHasher,
{
    type Output = V;
    type Ref<'a> = &'a V where Self: 'a, Self::Output: 'a;
}
#[cfg(feature = "std")]
impl<K, V, S, Q> Get<&Q> for HashMap<K, V, S>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    K: Eq + Hash,
    S: BuildHasher,
{
    #[inline(always)]
    fn get<'a>(&'a self, k: &Q) -> Option<Self::Ref<'a>> where Self::Output: 'a { self.get(k) }
}

#[cfg(feature = "std")]
impl<K, V, S, Q> TryGetMut<&Q> for HashMap<K, V, S>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq + Clone,
    K: Eq + Hash,
    S: BuildHasher,
{
    #[inline(always)]
    fn try_get_mut<'a>(&'a mut self, key: &Q) -> Result<Self::RefMut<'a>, Self::Error>
    where
        K: Borrow<Q>,
        Self::Output: 'a,
    {
        self.get_mut(key).ok_or_else(|| MissingKey::new(key.clone()))
    }
}
#[cfg(feature = "std")]
impl<K, V, S, Q> CollectionIndexMut<&Q> for HashMap<K, V, S>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    K: Eq + Hash,
    S: BuildHasher,
{
    type RefMut<'a> = &'a mut V where Self: 'a, Self::Output: 'a;
}
#[cfg(feature = "std")]
impl<K, V, S, Q> GetMut<&Q> for HashMap<K, V, S>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    K: Eq + Hash,
    S: BuildHasher,
{
    #[inline(always)]
    fn get_mut<'a>(&'a mut self, k: &Q) -> Option<Self::RefMut<'a>>
    where
        K: Borrow<Q>,
        Self::Output: 'a,
    {
        self.get_mut(k)
    }
}

#[cfg(feature = "std")]
impl<K, V, S, Q> GetManyMut<&Q> for HashMap<K, V, S>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    K: Eq + Hash,
    S: BuildHasher,
{
    #[inline(always)]
    fn try_get_many_mut<'a, const N: usize>(&'a mut self, keys: [&Q; N]) -> Result<[Self::RefMut<'a>; N], ManyMutError>
     where Self::Output: 'a{
        // I wanted to check this is the get_many_mut fail, but the borrow checker is complaining for no reason about self being already borrowed.
        for k in keys
        {
            if self.get_mut(k).is_none()
            {
                return Err(ManyMutError::IndexOutOfBounds);
            }
        }

        if let Some(r) = self.get_many_mut(keys)
        {
            return Ok(r);
        }

        Err(ManyMutError::OverlappingIndices)
    }

    fn get_many_mut<'a, const N: usize>(&'a mut self, keys: [&Q; N]) -> Option<[Self::RefMut<'a>; N]>
     where Self::Output: 'a{
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        // [Option<T>, N] to Option<[T;N]> (same for result)
        let r = self.get_disjoint_mut(keys);

        if r.iter().any(|x| x.is_none())
        {
            None
        }
        else
        {
            Some(r.map(|x| x.unwrap()))
        }
    }
}

impl<K, V, Q> TryGet<&Q> for BTreeMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord + Clone,
    K: Ord,
{
    type Error = MissingKey<Q>;
    fn try_get<'a>(&'a self, index: &Q) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a { self.get(index).ok_or_else(|| MissingKey::new(index.clone())) }
}
impl<K, V> Collection<V> for BTreeMap<K, V>
{
    type Ref<'a> = &'a V where Self: 'a, V: 'a;
}
impl<K, V> CollectionMut<V> for BTreeMap<K, V>
{
    type RefMut<'a> = &'a mut V where Self: 'a, V: 'a;
}

impl<K, V, Q> CollectionIndex<&Q> for BTreeMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord,
    K: Ord,
{
    type Output = V;
    type Ref<'a> = &'a V where Self: 'a, Self::Output: 'a;
}
impl<K, V, Q> Get<&Q> for BTreeMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord,
    K: Ord,
{
    #[inline(always)]
    fn get<'a>(&'a self, k: &Q) -> Option<Self::Ref<'a>>
    where
        K: Borrow<Q>,
        Self::Output: 'a,
    {
        self.get(k)
    }
}

impl<K, V, Q> TryGetMut<&Q> for BTreeMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord + Clone,
    K: Ord,
{
    fn try_get_mut<'a>(&'a mut self, index: &Q) -> Result<Self::RefMut<'a>, Self::Error> where Self::Output: 'a { self.get_mut(index).ok_or_else(|| MissingKey::new(index.clone())) }
}
impl<K, V, Q> CollectionIndexMut<&Q> for BTreeMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord,
    K: Ord,
{
    type RefMut<'a> = &'a mut V where Self: 'a, Self::Output: 'a;
}
impl<K, V, Q> GetMut<&Q> for BTreeMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord,
    K: Ord,
{
    #[inline(always)]
    fn get_mut<'a>(&'a mut self, k: &Q) -> Option<Self::RefMut<'a>>
    where
        K: Borrow<Q>,
        Self::Output: 'a,
    {
        self.get_mut(k)
    }
}

#[cfg(feature = "std")]
impl<K, S, Q> TryGet<&Q> for HashSet<K, S>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq + Clone,
    K: Eq + Hash,
    S: BuildHasher,
{
    type Error = MissingKey<Q>;
    fn try_get<'a>(&'a self, index: &Q) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a { self.get(index).ok_or_else(|| MissingKey::new(index.clone())) }
}
#[cfg(feature = "std")]
impl<K, S> Collection<K> for HashSet<K, S>
{
    type Ref<'a> = &'a K where Self: 'a, K: 'a;
}

#[cfg(feature = "std")]
impl<K, S, Q> CollectionIndex<&Q> for HashSet<K, S>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    K: Eq + Hash,
    S: BuildHasher,
{
    type Output = K;
    type Ref<'a> = &'a K where Self: 'a, Self::Output: 'a;
}
#[cfg(feature = "std")]
impl<K, S, Q> Get<&Q> for HashSet<K, S>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    K: Eq + Hash,
    S: BuildHasher,
{
    #[inline(always)]
    fn get<'a>(&'a self, k: &Q) -> Option<Self::Ref<'a>> where Self::Output: 'a { self.get(k) }
}

impl<K, Q> TryGet<&Q> for BTreeSet<K>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord + Clone,
    K: Ord,
{
    type Error = MissingKey<Q>;
    fn try_get<'a>(&'a self, index: &Q) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a { self.get(index).ok_or_else(|| MissingKey::new(index.clone())) }
}
impl<K> Collection<K> for BTreeSet<K>
{
    type Ref<'a> = &'a K where Self: 'a, K: 'a;
}

impl<K, Q> CollectionIndex<&Q> for BTreeSet<K>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord,
    K: Ord,
{
    type Output = K;
    type Ref<'a> = &'a K where Self: 'a, Self::Output: 'a;
}
impl<K, Q> Get<&Q> for BTreeSet<K>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord,
    K: Ord,
{
    #[inline(always)]
    fn get<'a>(&'a self, k: &Q) -> Option<Self::Ref<'a>> where Self::Output: 'a { self.get(k) }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn result_indices_select_the_matching_payload()
    {
        let mut ok: Result<i32, &str> = Ok(1);
        assert_eq!(Get::get(&ok, ResultOk), Some(&1));
        assert_eq!(GetMut::get_mut(&mut ok, ResultErr), None);
        ok[ResultOk] = 2;
        assert_eq!(ok[ResultOk], 2);

        let mut err: Result<i32, &str> = Err("first");
        assert_eq!(Get::get(&err, ResultOk), None);
        *GetMut::get_mut(&mut err, ResultErr).unwrap() = "second";
        assert_eq!(err[ResultErr], "second");
        err[ResultErr] = "third";
        assert_eq!(err, Err("third"));

        let mut same_type: Result<i32, i32> = Err(3);
        assert_eq!(Get::get(&same_type, ResultOk), None);
        assert_eq!(Get::get(&same_type, ResultErr), Some(&3));
        *GetMut::get_mut(&mut same_type, ResultErr).unwrap() = 4;
        assert_eq!(same_type, Err(4));
    }

    #[test]
    fn string_indices_return_characters_and_ranges_return_slices()
    {
        let string = String::from("aé🦀");
        assert_eq!(*Get::get(&string, 2).unwrap(), '🦀');
        assert_eq!(Get::get(&string, 1..3), Some("é"));
        assert_eq!(Get::get(&string, 1..2), None);
        assert_eq!(TryGet::try_get(string.as_str(), 3), Err(IndexOutOfBounds::new(3, 0..3)));
    }

    #[test]
    fn mutable_sequence_indices_return_elements_and_slices()
    {
        let mut values = [1, 2, 3];
        *GetMut::get_mut(&mut values, 0).unwrap() = 4;
        GetMut::get_mut(&mut values, 1..).unwrap().copy_from_slice(&[5, 6]);
        assert_eq!(values, [4, 5, 6]);
    }
}
