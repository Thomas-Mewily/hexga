use crate::*;

pub trait GetIndex<Idx> : Index<Idx>
{
    //fn try_get(&self, idx : Idx) -> Result<&Self::Output, Self::Error>;
    fn get(&self, idx : Idx) -> Option<&Self::Output>;
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { self.get(idx).expect("invalid index") }

    // Todo : add get_disjoint?
}
pub trait GetIndexMut<Idx> : GetIndex<Idx>
{
    //fn try_get(&self, idx : Idx) -> Result<&Self::Output, Self::Error>;
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output>;
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { self.get_mut(idx).expect("invalid index") }
}



impl<Idx,T> GetIndex<Idx> for [T] where Idx : SliceIndex<[T]>
{
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.get(idx) }
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.get_unchecked(idx) } }
}
impl<Idx,T> GetIndexMut<Idx> for [T] where Idx : SliceIndex<[T]>
{
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.get_mut(idx) }
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.get_unchecked_mut(idx) } }
}



impl<Idx,T,const N : usize> GetIndex<Idx> for [T;N] where for<'a> &'a [T] : GetIndex<Idx>, Idx : SliceIndex<[T]>
{
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.as_slice().get(idx) }
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.as_slice().get_unchecked(idx) } }
}
impl<Idx,T,const N : usize> GetIndexMut<Idx> for [T;N] where for<'a> &'a mut[T] : GetIndexMut<Idx>, for<'a> &'a[T] : GetIndex<Idx>, Idx : SliceIndex<[T]>
{
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.as_mut_slice().get_mut(idx) }
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.as_mut_slice().get_unchecked_mut(idx) } }
}



impl<Idx,T> GetIndex<Idx> for Vec<T> where Idx : SliceIndex<[T]>
{
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.as_slice().get(idx) }
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.as_slice().get_unchecked(idx) } }
}
impl<Idx,T> GetIndexMut<Idx> for Vec<T> where Idx : SliceIndex<[T]>
{
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.as_mut_slice().get_mut(idx) }
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.as_mut_slice().get_unchecked_mut(idx) } }
}



impl<Idx> GetIndex<Idx> for str where Idx : SliceIndex<str>
{
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.get(idx) }
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.get_unchecked(idx) } }
}
impl<Idx> GetIndexMut<Idx> for str where Idx : SliceIndex<str>
{
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.get_mut(idx) }
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.get_unchecked_mut(idx) } }
}


impl<K,V,S> GetIndex<&K> for HashMap<K,V,S> where K: Hash + Eq, S : BuildHasher
{
    fn get(&self, idx : &K) -> Option<&Self::Output> { self.get(idx) }
}
impl<K,V,S> GetIndexMut<&K> for HashMap<K,V,S> where K: Hash + Eq, S : BuildHasher
{
    fn get_mut(&mut self, idx : &K) -> Option<&mut Self::Output> { self.get_mut(idx) }
}