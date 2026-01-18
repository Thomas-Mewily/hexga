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


/// The collection have a quick way to access each element, where the index is copyable
pub trait Get<Idx> //where Idx : Borrow<Q> //: Index<Idx>
{
    type Output : ?Sized;

    /// Returns a reference to the value.
    fn get(&self, index : Idx) -> Option<&Self::Output>;
    /// Returns a reference to the value.
    #[inline(always)]
    #[track_caller]
    fn get_or_panic(&self, index : Idx) -> &Self::Output { self.get(index).expect("invalid index") }
    /// Returns a reference to the value.
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, index : Idx) -> &Self::Output { self.get(index).expect("invalid index") }

    /// True if `get(index)` return [Some], false otherwise.
    #[inline(always)]
    fn is_index_valid(&self, index : Idx) -> bool { self.get(index).is_some() }
    /// True if `get(index)` return [None], false otherwise.
    #[inline(always)]
    fn is_index_invalid(&self, index : Idx) -> bool { self.get(index).is_none() }
}

pub trait TryGet<Idx> : Get<Idx>
{
    // TODO : Should the Error have a lifetime parameter of self ? Then the caller call error.to_owned() to store it ?
    type Error;
    /// Returns a reference to the value.
    fn try_get(&self, index : Idx) -> Result<&Self::Output, Self::Error>;
}



pub trait GetMut<Idx> : Get<Idx>
{
    /// Returns a mutable reference to the value.
    fn get_mut(&mut self, index : Idx) -> Option<&mut Self::Output>;
    #[inline(always)]
    #[track_caller]
    fn get_mut_or_panic(&mut self, idx : Idx) -> &mut Self::Output { self.get_mut(idx).expect("invalid index") }
    /// Returns a mutable reference to the value.
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { self.get_mut(idx).expect("invalid index") }


    /// Replace the value and return the old one.
    ///
    /// This operation is an [`involution`](https://en.wikipedia.org/wiki/Involution_(mathematics)):
    /// Replacing a value twice (first with a new value, then with the previously returned value) leaves the collection unchanged.
    #[inline(always)]
    fn replace(&mut self, index : Idx, value : Self::Output) -> Option<Self::Output> where Self::Output : Sized { self.get_mut(index).map(|dest| std::mem::replace(dest, value)) }
    /// Replace the value and return the old one.
    ///
    /// This operation is an [`involution`](https://en.wikipedia.org/wiki/Involution_(mathematics)):
    /// Replacing a value twice (first with a new value, then with the previously returned value) leaves the collection unchanged.
    #[inline(always)]
    #[track_caller]
    fn replace_or_panic(&mut self, index : Idx, value : Self::Output) -> Self::Output where Self::Output : Sized { self.replace(index, value).expect("invalid index") }
    /// Replace the value and return the old one.
    ///
    /// This operation is an [`involution`](https://en.wikipedia.org/wiki/Involution_(mathematics)):
    /// Replacing a value twice (first with a new value, then with the previously returned value) leaves the collection unchanged.
    #[inline(always)]
    #[track_caller]
    unsafe fn replace_unchecked(&mut self, index : Idx, value : Self::Output) -> Self::Output where Self::Output : Sized { std::mem::replace(unsafe { self.get_unchecked_mut(index) }, value) }

    /// Set the value and drop the previous one.
    #[inline(always)]
    fn set(&mut self, index : Idx, value : Self::Output) -> bool where Self::Output : Sized { self.replace(index, value).map(|_| ()).is_some() }
    /// Set the value and drop the previous one.
    #[inline(always)]
    #[track_caller]
    fn set_or_panic(&mut self, index : Idx, value : Self::Output) -> &mut Self where Self::Output : Sized { assert!(self.set(index, value), "invalid index"); self }
    #[inline(always)]
    #[track_caller]
    unsafe fn set_unchecked(&mut self, index : Idx, value : Self::Output) -> &mut Self where Self::Output : Sized { unsafe { self.replace_unchecked(index, value) }; self }
}

pub trait TryGetMut<Idx> : TryGet<Idx>
{
    // Should TryGetMut have it own error type ?

    /// Returns a mutable reference to the value.
    fn try_get_mut(&mut self, index : Idx) -> Result<&mut Self::Output, Self::Error>;

    /// Replace the value and return the old one.
    ///
    /// This operation is an [`involution`](https://en.wikipedia.org/wiki/Involution_(mathematics)):
    /// Replacing a value twice (first with a new value, then with the previously returned value) leaves the collection unchanged.
    #[inline(always)]
    fn try_replace(&mut self, index : Idx, value : Self::Output) -> Result<Self::Output, Self::Error> where Self::Output : Sized { self.try_get_mut(index).map(|dest| std::mem::replace(dest, value)) }

    /// Set the value and drop the previous one.
    #[inline(always)]
    fn try_set(&mut self, index : Idx, value : Self::Output) -> Result<(), Self::Error> where Self::Output : Sized { self.try_replace(index, value).map(|_| ()) }
}

pub type ManyMutError = std::slice::GetDisjointMutError;

pub trait GetManyMut<Idx> : GetMut<Idx>
{
    /// Returns multiples mutables references to the values.
    /// All values that can be accessed with the indices must be disjoint.
    #[doc(alias = "get_disjoint_mut")]
    fn get_many_mut<const N: usize>(&mut self, indices: [Idx; N]) -> Option<[&mut Self::Output;N]> { self.try_get_many_mut(indices).ok() }

    fn try_get_many_mut<const N: usize>(&mut self, indices: [Idx; N]) -> Result<[&mut Self::Output;N], ManyMutError>;

    /// Returns multiples mutables references to the values.
    /// All values that can be accessed with the indices must be disjoint.
    #[inline(always)]
    #[track_caller]
    #[doc(alias = "get_disjoint_mut_or_panic")]
    fn get_many_mut_or_panic<const N: usize>(&mut self, indices: [Idx; N]) -> [&mut Self::Output;N] { self.get_many_mut(indices).expect("invalid index") }


    /// Returns multiples mutables references to the values.
    /// All values that can be accessed with the indices must be disjoint.
    #[inline(always)]
    #[track_caller]
    #[doc(alias = "get_disjoint_unchecked_mut")]
    unsafe fn get_many_unchecked_mut<const N: usize>(&mut self, indices: [Idx; N]) -> [&mut Self::Output;N] { self.get_many_mut(indices).expect("invalid index") }

    /// Swaps the values at two mutable locations, without deinitializing either one.
    ///
    /// Swap is symmetric : `foo.try_swap(a, b)` is equivalent to `foo.try_swap(b, a)` and vis versa
    ///
    /// Do nothings if some value  overlap or don't exist.
    #[inline(always)]
    fn swap(&mut self, a : Idx, b : Idx) -> bool where Self::Output : Sized { self.get_many_mut([a, b]).map(|[a,b]| std::mem::swap(a, b)).is_some() }
    /// Swaps the values at two mutable locations, without deinitializing either one.
    ///
    /// Swap is symmetric : `foo.try_swap(a, b)` is equivalent to `foo.try_swap(b, a)` and vis versa
    ///
    /// Panics if any value overlap or don't exist
    #[inline(always)]
    #[track_caller]
    fn swap_or_panic(&mut self, a : Idx, b : Idx) where Self::Output : Sized { assert!(self.swap(a, b), "invalid index") }
    /// Swaps the values at two mutable locations, without deinitializing either one.
    ///
    /// Swap is symmetric : `foo.try_swap(a, b)` is equivalent to `foo.try_swap(b, a)` and vis versa
    ///
    /// Do nothings if some value  overlap or don't exist.
    #[inline(always)]
    #[track_caller]
    unsafe fn swap_unchecked(&mut self, a : Idx, b : Idx) where Self::Output : Sized { let [a,b] = unsafe { self.get_many_unchecked_mut([a, b]) }; std::mem::swap(a, b); }
}









// TODO: impl it. Split the core crate : hexga_core : hexga_core_syntax, hexga_core_collection
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct IndexOutOfRange<Idx=usize, R=std::ops::Range<Idx>>
{
    pub index : Idx,
    pub range : R,
}
impl<Idx, R> IndexOutOfRange<Idx, R>
{
    pub fn new(index : Idx, range : R) -> Self { Self { index, range }}
}

impl<Idx,T> TryGet<Idx> for [T] where Idx : SliceIndex<[T]> + Clone
{
    type Error = IndexOutOfRange<Idx,Range<usize>>;
    #[inline(always)]
    fn try_get(&self, index : Idx) -> Result<&Self::Output, Self::Error> { self.get(index.clone()).ok_or(IndexOutOfRange{ index, range: 0..self.len() }) }
}
impl<Idx,T> Get<Idx> for [T] where Idx : SliceIndex<[T]>
{
    type Output = <Self as Index<Idx>>::Output;
    #[inline(always)]
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.get(idx) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.get_unchecked(idx) } }
}

impl<Idx,T> TryGetMut<Idx> for [T] where Idx : SliceIndex<[T]> + Clone
{
    #[inline(always)]
    fn try_get_mut(&mut self, idx : Idx) -> Result<&mut Self::Output, Self::Error> { let len = self.len(); self.get_mut(idx.clone()).ok_or(IndexOutOfRange{ index: idx, range: 0..len }) }
}
impl<Idx,T> GetMut<Idx> for [T] where Idx : SliceIndex<[T]>
{
    #[inline(always)]
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.get_mut(idx) }
    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.get_unchecked_mut(idx) } }
}
impl<Idx,T> GetManyMut<Idx> for [T] where Idx : SliceIndex<[T]> + GetDisjointMutIndex
{
    #[inline(always)]
    fn get_many_mut<const N: usize>(&mut self, indices: [Idx; N]) -> Option<[&mut Self::Output;N]> { self.get_disjoint_mut(indices).ok() }

    #[inline(always)]
    fn try_get_many_mut<const N: usize>(&mut self, indices: [Idx; N]) -> Result<[&mut Self::Output;N], ManyMutError> { self.get_disjoint_mut(indices) }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<const N: usize>(&mut self, indices: [Idx; N]) -> [&mut Self::Output;N]  { unsafe { self.get_disjoint_unchecked_mut(indices) } }


}

impl<Idx,T,const N : usize> TryGet<Idx> for [T;N] where [T] : TryGet<Idx>
{
    type Error = <[T] as TryGet<Idx>>::Error;
    #[inline(always)]
    fn try_get(&self, index : Idx) -> Result<&Self::Output, Self::Error> { TryGet::try_get(self.as_slice(), index) }
}

impl<Idx,T,const N : usize> Get<Idx> for [T;N] where [T] : Get<Idx>
{
    type Output =  <[T] as Get<Idx>>::Output;
    #[inline(always)]
    fn get(&self, idx : Idx) -> Option<&Self::Output> { Get::get(self.as_slice(), idx) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { Get::get_unchecked(self.as_slice(), idx) } }
}

impl<Idx,T,const N : usize> TryGetMut<Idx> for [T;N] where [T] : TryGetMut<Idx>
{
    #[inline(always)]
    fn try_get_mut(&mut self, index : Idx) -> Result<&mut Self::Output, Self::Error> { TryGetMut::try_get_mut(self.as_mut_slice(), index) }
}

impl<Idx,T,const N : usize> GetMut<Idx> for [T;N] where [T] : GetMut<Idx>
{
    #[inline(always)]
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { GetMut::get_mut(self.as_mut_slice(), idx) }
    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { GetMut::get_unchecked_mut(self.as_mut_slice(), idx) } }
}
impl<Idx,T,const N : usize> GetManyMut<Idx> for [T;N] where [T] : GetManyMut<Idx>
{
    #[inline(always)]
    fn get_many_mut<const N2: usize>(&mut self, indices: [Idx; N2]) -> Option<[&mut Self::Output;N2]> { GetManyMut::get_many_mut(self.as_mut_slice(), indices) }

    #[inline(always)]
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [Idx; N2]) -> Result<[&mut Self::Output;N2], ManyMutError> { GetManyMut::try_get_many_mut(self.as_mut_slice(), indices) }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<const N2: usize>(&mut self, indices: [Idx; N2]) -> [&mut Self::Output;N2] { unsafe { GetManyMut::get_many_unchecked_mut(self.as_mut_slice(), indices) } }


}


impl<Idx,T> TryGet<Idx> for Vec<T> where [T] : TryGet<Idx>
{
    type Error = <[T] as TryGet<Idx>>::Error;
    #[inline(always)]
    fn try_get(&self, index : Idx) -> Result<&Self::Output, Self::Error> { TryGet::try_get(self.as_slice(), index) }
}
impl<Idx,T> Get<Idx> for Vec<T> where [T] : Get<Idx>
{
    type Output = <[T] as Get<Idx>>::Output;
    #[inline(always)]
    fn get(&self, idx : Idx) ->Option<&Self::Output> { Get::get(self.as_slice(), idx) }
    #[track_caller]
    #[inline(always)]
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { Get::get_unchecked(self.as_slice(), idx) } }
}
impl<Idx,T> TryGetMut<Idx> for Vec<T> where [T] : TryGetMut<Idx>
{
    #[inline(always)]
    fn try_get_mut(&mut self, index : Idx) -> Result<&mut Self::Output, Self::Error> { TryGetMut::try_get_mut(self.as_mut_slice(), index) }
}
impl<Idx,T> GetMut<Idx> for Vec<T> where [T] : GetMut<Idx>
{
    #[inline(always)]
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { GetMut::get_mut(self.as_mut_slice(), idx) }
    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { GetMut::get_unchecked_mut(self.as_mut_slice(), idx) } }
}
impl<Idx,T> GetManyMut<Idx> for Vec<T> where [T] : GetManyMut<Idx>
{
    #[track_caller]
    #[inline(always)]
    unsafe fn get_many_unchecked_mut<const N: usize>(&mut self, indices: [Idx; N]) -> [&mut Self::Output;N] { unsafe { GetManyMut::get_many_unchecked_mut(self.as_mut_slice(), indices) } }

    #[inline(always)]
    fn try_get_many_mut<const N: usize>(&mut self, indices: [Idx; N]) -> Result<[&mut Self::Output;N], ManyMutError> { GetManyMut::try_get_many_mut(self.as_mut_slice(), indices) }
    #[inline(always)]
    fn get_many_mut<const N: usize>(&mut self, indices: [Idx; N]) -> Option<[&mut Self::Output;N]> { GetManyMut::get_many_mut(self.as_mut_slice(), indices) }
}


impl<T> TryGet<usize> for VecDeque<T>
{
    type Error = IndexOutOfRange<usize, std::ops::Range<usize>>;
    #[inline(always)]
    fn try_get(&self, index : usize) -> Result<&Self::Output, Self::Error>{ self.get(index).ok_or(IndexOutOfRange{ index, range: 0..self.len() }) }
}
impl<T> Get<usize> for VecDeque<T>
{
    type Output = <Self as Index<usize>>::Output;
    #[inline(always)]
    fn get(&self, idx : usize) -> Option<&Self::Output> { self.get(idx) }
}
impl<T> TryGetMut<usize> for VecDeque<T>
{
    #[inline(always)]
    fn try_get_mut(&mut self, index : usize) -> Result<&mut Self::Output, Self::Error>{ let len = self.len(); self.get_mut(index).ok_or(IndexOutOfRange{ index, range: 0..len }) }
}
impl<T> GetMut<usize> for VecDeque<T>
{
    #[inline(always)]
    fn get_mut(&mut self, idx : usize) -> Option<&mut Self::Output> { self.get_mut(idx) }
}

impl<T> GetManyMut<usize> for VecDeque<T>
{
    #[track_caller]
    #[inline(always)]
    unsafe fn get_many_unchecked_mut<const N: usize>(&mut self, indices : [usize; N]) -> [&mut Self::Output;N]
    {
        unsafe
        {
            // Can probably be improved using `self.as_mut_slices()`
            self.make_contiguous().get_disjoint_unchecked_mut(indices)
        }
    }
    #[inline(always)]
    fn try_get_many_mut<const N: usize>(&mut self, indices : [usize; N]) -> Result<[&mut Self::Output;N], ManyMutError> { self.make_contiguous().try_get_many_mut(indices) }
    #[inline(always)]
    fn get_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut Self::Output;N]> { self.make_contiguous().get_many_mut(indices) }
}


impl<Idx> TryGet<Idx> for str where Idx : SliceIndex<str> + Clone
{
    type Error = IndexOutOfRange<Idx,Range<usize>>;
    fn try_get(&self, index : Idx) -> Result<&Self::Output, Self::Error>
    {
        self.get(index.clone()).ok_or(IndexOutOfRange{ index, range: 0..self.len() })
    }
}

impl<Idx> Get<Idx> for str where Idx : SliceIndex<str>
{
    type Output = <Self as Index<Idx>>::Output;
    #[inline(always)]
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.get(idx) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.get_unchecked(idx) } }
}

// FIXME : I'm not sure how to delegate it to `str` like delegating the TryGet for `Vec<T>` into the TryGet of `[T]`
impl<Idx> TryGet<Idx> for String where Idx : SliceIndex<str> + Clone
{
    type Error = IndexOutOfRange<Idx,Range<usize>>;
    fn try_get(&self, index : Idx) -> Result<&Self::Output, Self::Error>
    {
        self.get(index.clone()).ok_or(IndexOutOfRange{ index, range: 0..self.len() })
    }
}
impl<Idx> Get<Idx> for String where Idx : SliceIndex<str>
{
    type Output = <str as Get<Idx>>::Output;
    #[inline(always)]
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.as_str().get(idx) }
    #[track_caller]
    #[inline(always)]
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.as_str().get_unchecked(idx) } }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct MissingKey<K>
{
    pub key : K,
}
impl<K> MissingKey<K>
{
    pub fn new(key: K) -> Self { Self { key }}
}

impl<K,V,S,Q> TryGet<&Q> for HashMap<K,V,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq + Clone, K: Eq + Hash, S: BuildHasher
{
    type Error = MissingKey<Q>;

    fn try_get(&self, key : &Q) -> Result<&Self::Output, Self::Error> {
        self.get(key).ok_or_else(|| MissingKey::new(key.clone()))
    }
}

impl<K,V,S,Q> Get<&Q> for HashMap<K,V,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    type Output = V;
    #[inline(always)]
    fn get(&self, k: &Q) -> Option<&Self::Output> { self.get(k) }
}


impl<K,V,S,Q> TryGetMut<&Q> for HashMap<K,V,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq + Clone, K: Eq + Hash, S: BuildHasher
{
    #[inline(always)]
    fn try_get_mut(&mut self, key: &Q) -> Result<&mut Self::Output, Self::Error> where K : Borrow<Q> {
        self.get_mut(key).ok_or_else(|| MissingKey::new(key.clone()))
    }
}
impl<K,V,S,Q> GetMut<&Q> for HashMap<K,V,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    #[inline(always)]
    fn get_mut(&mut self, k: &Q) -> Option<&mut Self::Output> where K : Borrow<Q> { self.get_mut(k) }
}

impl<K,V,S,Q> GetManyMut<&Q> for HashMap<K,V,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    #[inline(always)]
    fn try_get_many_mut<const N: usize>(&mut self, keys: [&Q; N]) -> Result<[&mut Self::Output;N], ManyMutError>
    {
        // I wanted to check this is the get_many_mut fail, but the borrow checker is complaining for no reason about self being already borrowed.
        for k in keys
        {
            if self.get_mut(k).is_none() { return Err(ManyMutError::IndexOutOfBounds) }
        }

        if let Some(r) = self.get_many_mut(keys)
        {
            return Ok(r);
        }

        Err(ManyMutError::OverlappingIndices)
    }

    fn get_many_mut<const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut Self::Output;N]>
    {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        // [Option<T>, N] to Option<[T;N]> (same for result)
        let r = self.get_disjoint_mut(keys);

        if r.iter().any(|x| x.is_none())
        {
            None
        } else
        {
            Some(r.map(|x| x.unwrap()))
        }
    }
}

impl<K,V,Q> TryGet<&Q> for BTreeMap<K,V> where K : Borrow<Q>, Q : ?Sized + Ord + Clone, K: Ord
{
    type Error = MissingKey<Q>;
    fn try_get(&self, index : &Q) -> Result<&Self::Output, Self::Error> {
        self.get(index).ok_or_else(|| MissingKey::new(index.clone()))
    }
}
impl<K,V,Q> Get<&Q> for BTreeMap<K,V> where K : Borrow<Q>, Q : ?Sized + Ord, K: Ord
{
    type Output = V;
    #[inline(always)]
    fn get(&self, k: &Q) -> Option<&Self::Output> where K : Borrow<Q> { self.get(k) }
}

impl<K,V,Q> TryGetMut<&Q> for BTreeMap<K,V> where K : Borrow<Q>, Q : ?Sized + Ord + Clone, K: Ord
{
    fn try_get_mut(&mut self, index : &Q) -> Result<&mut Self::Output, Self::Error> {
        self.get_mut(index).ok_or_else(|| MissingKey::new(index.clone()))
    }
}
impl<K,V,Q> GetMut<&Q> for BTreeMap<K,V> where K : Borrow<Q>, Q : ?Sized + Ord, K: Ord
{
    #[inline(always)]
    fn get_mut(&mut self, k: &Q) -> Option<&mut Self::Output> where K : Borrow<Q> { self.get_mut(k) }
}


impl<K,S,Q> TryGet<&Q> for HashSet<K,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq + Clone, K: Eq + Hash, S: BuildHasher
{
    type Error=MissingKey<Q>;
    fn try_get(&self, index : &Q) -> Result<&Self::Output, Self::Error> {
          self.get(index).ok_or_else(|| MissingKey::new(index.clone()))
    }
}
impl<K,S,Q> Get<&Q> for HashSet<K,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    type Output = K;
    #[inline(always)]
    fn get(&self, k: &Q) -> Option<&Self::Output> { self.get(k) }
}

impl<K,Q> TryGet<&Q> for BTreeSet<K> where K : Borrow<Q>, Q : ?Sized + Ord + Clone, K: Ord
{
    type Error=MissingKey<Q>;
    fn try_get(&self, index : &Q) -> Result<&Self::Output, Self::Error> {
          self.get(index).ok_or_else(|| MissingKey::new(index.clone()))
    }
}
impl<K,Q> Get<&Q> for BTreeSet<K> where K : Borrow<Q>, Q : ?Sized + Ord, K: Ord
{
    type Output = K;
    #[inline(always)]
    fn get(&self, k: &Q) -> Option<&Self::Output> { self.get(k) }
}