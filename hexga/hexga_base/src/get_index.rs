use crate::*;

/// The collection have a quick way to access each element, where the index is copyable
pub trait GetIndex<Idx> //: Index<Idx>
{
    type Output : ?Sized;
    fn get(&self, idx : Idx) -> Option<&Self::Output>;
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { self.get(idx).expect("invalid index") }
}
pub trait GetIndexMut<Idx> : GetIndex<Idx> where Idx : Copy
{
    //fn try_get(&self, idx : Idx) -> Result<&Self::Output, Self::Error>;
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output>;
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { self.get_mut(idx).expect("invalid index") }

    /// Do nothings if the value don't exist
    fn set(&mut self, index : Idx, value : Self::Output) -> bool where Self::Output : Sized { self.get_mut(index).map(|v| *v = value).is_some() }
    
    /// Panics if the value don't exist
    fn set_or_panic(&mut self, index : Idx, value : Self::Output) -> &mut Self where Self::Output : Sized { *self.get_mut(index).unwrap() = value; self }
    
    fn replace(&mut self, index : Idx, value : Self::Output) -> Option<Self::Output> where Self::Output : Sized { self.get_mut(index).map(|v| std::mem::replace(v, value)) }
    /// Panics if the value don't exist
    fn replace_or_panic(&mut self, index : Idx, value : Self::Output) -> Self::Output where Self::Output : Sized { std::mem::replace(self.get_mut(index).expect("invalid index"), value) }
    
    
    // fn try_set(...) -> Result<(), ()>
    // fn try_replace(...) -> Result<(), ()>


    /* 
    /// Panics if the component don't exist
    fn set_or_panic(&mut self, idx : Idx, val : Self::Output) -> &mut Self where Self::Output : Sized { *self.get_mut(idx).unwrap() = val; self }
    /// Do nothings if the component don't exist
    fn set(&mut self, idx : Idx, val : Self::Output) -> bool where Self::Output : Sized { self.get_mut(idx).map(|v| *v = val).is_some() }
    */
    // Todo : add disjoint_mut when core::slice::GetDisjointMutIndex will be stable
    //fn get_disjoint_mut<const N: usize>(&mut self, indices: [Idx; N]) -> Option<[&mut Self::Output;N]>; where Idx : core::slice::GetDisjointMutIndex;
}


impl<Idx,T> GetIndex<Idx> for [T] where Idx : SliceIndex<[T]> + Copy
{
    type Output = <Self as Index<Idx>>::Output;
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.get(idx) }
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.get_unchecked(idx) } }
    
}
impl<Idx,T> GetIndexMut<Idx> for [T] where Idx : SliceIndex<[T]> + Copy
{
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.get_mut(idx) }
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.get_unchecked_mut(idx) } }
}


impl<Idx,T,const N : usize> GetIndex<Idx> for [T;N] where for<'a> &'a [T] : GetIndex<Idx>, Idx : SliceIndex<[T]> + Copy
{
    type Output = <Self as Index<Idx>>::Output;
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.as_slice().get(idx) }
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.as_slice().get_unchecked(idx) } }
}
impl<Idx,T,const N : usize> GetIndexMut<Idx> for [T;N] where for<'a> &'a mut[T] : GetIndexMut<Idx>, for<'a> &'a[T] : GetIndex<Idx>, Idx : SliceIndex<[T]> + Copy
{
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.as_mut_slice().get_mut(idx) }
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.as_mut_slice().get_unchecked_mut(idx) } }
}


impl<Idx,T> GetIndex<Idx> for Vec<T> where Idx : SliceIndex<[T]> + Copy
{
    type Output = <Self as Index<Idx>>::Output;
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.as_slice().get(idx) }
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.as_slice().get_unchecked(idx) } }
}
impl<Idx,T> GetIndexMut<Idx> for Vec<T> where Idx : SliceIndex<[T]> + Copy
{
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.as_mut_slice().get_mut(idx) }
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.as_mut_slice().get_unchecked_mut(idx) } }
}


impl<T> GetIndex<usize> for VecDeque<T>
{
    type Output = <Self as Index<usize>>::Output;
    fn get(&self, idx : usize) -> Option<&Self::Output> { self.get(idx) }
}
impl<T> GetIndexMut<usize> for VecDeque<T>
{
    fn get_mut(&mut self, idx : usize) -> Option<&mut Self::Output> { self.get_mut(idx) }
}

impl<Idx> GetIndex<Idx> for str where Idx : SliceIndex<str> + Copy
{
    type Output = <Self as Index<Idx>>::Output;
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.get(idx) }
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.get_unchecked(idx) } }
}
impl<Idx> GetIndexMut<Idx> for str where Idx : SliceIndex<str> + Copy
{
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.get_mut(idx) }
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.get_unchecked_mut(idx) } }
}



impl<K,V,S,Q> GetIndex<&Q> for HashMap<K,V,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    type Output = V;
    fn get(&self, k: &Q) -> Option<&Self::Output> { self.get(k) }
}

impl<K,V,S,Q> GetIndexMut<&Q> for HashMap<K,V,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    fn get_mut(&mut self, k: &Q) -> Option<&mut Self::Output> where K : Borrow<Q> { self.get_mut(k) }
}

impl<K,V,Q> GetIndex<&Q> for BTreeMap<K,V> where K : Borrow<Q>, Q : ?Sized + Ord, K: Ord
{
    type Output = V;
    fn get(&self, k: &Q) -> Option<&Self::Output> where K : Borrow<Q> { self.get(k) }
}

impl<K,V,Q> GetIndexMut<&Q> for BTreeMap<K,V> where K : Borrow<Q>, Q : ?Sized + Ord, K: Ord
{
    fn get_mut(&mut self, k: &Q) -> Option<&mut Self::Output> where K : Borrow<Q> { self.get_mut(k) }
}

impl<K,S,Q> GetIndex<&Q> for HashSet<K,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    type Output = K;
    fn get(&self, k: &Q) -> Option<&Self::Output> { self.get(k) }
}

impl<K,Q> GetIndex<&Q> for BTreeSet<K> where K : Borrow<Q>, Q : ?Sized + Ord, K: Ord
{
    type Output = K;
    fn get(&self, k: &Q) -> Option<&Self::Output> { self.get(k) }
}