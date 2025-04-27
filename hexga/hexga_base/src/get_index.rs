use crate::*;

/// The collection have a quick way to access each element, where the index is copyable
pub trait GetIndex<Idx> : Index<Idx> + LookUp<Idx,Idx,LookUpOutput = <Self as Index::<Idx>>::Output>
{
    fn get(&self, idx : Idx) -> Option<&Self::Output>;
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { self.get(idx).expect("invalid index") }
}
pub trait GetIndexMut<Idx> : GetIndex<Idx> + LookUpMut<Idx,Idx,LookUpOutput = <Self as Index::<Idx>>::Output> where Idx : Copy
{
    //fn try_get(&self, idx : Idx) -> Result<&Self::Output, Self::Error>;
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output>;
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { self.get_mut(idx).expect("invalid index") }
}


impl<Idx,T> LookUp<Idx>    for [T] where Idx : SliceIndex<[T]> + Copy { type LookUpOutput = <Self as Index::<Idx>>::Output; fn lookup(&self, k: Idx) -> Option<&Self::LookUpOutput> where Idx : Borrow<Idx> { self.get(k) } }
impl<Idx,T> LookUpMut<Idx> for [T] where Idx : SliceIndex<[T]> + Copy { fn lookup_mut(&mut self, k: Idx) -> Option<&mut Self::LookUpOutput> where Idx : Borrow<Idx> { self.get_mut(k) } }

impl<Idx,T> GetIndex<Idx> for [T] where Idx : SliceIndex<[T]> + Copy
{
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.get(idx) }
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.get_unchecked(idx) } }
}
impl<Idx,T> GetIndexMut<Idx> for [T] where Idx : SliceIndex<[T]> + Copy
{
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.get_mut(idx) }
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.get_unchecked_mut(idx) } }
}


impl<Idx,T,const N : usize> LookUp<Idx>    for [T;N] where Idx : SliceIndex<[T]> + Copy { type LookUpOutput = <Self as Index::<Idx>>::Output; fn lookup(&self, k: Idx) -> Option<&Self::LookUpOutput> where Idx : Borrow<Idx> { self.get(k) } }
impl<Idx,T,const N : usize> LookUpMut<Idx> for [T;N] where Idx : SliceIndex<[T]> + Copy { fn lookup_mut(&mut self, k: Idx) -> Option<&mut Self::LookUpOutput> where Idx : Borrow<Idx> { self.get_mut(k) } }

impl<Idx,T,const N : usize> GetIndex<Idx> for [T;N] where for<'a> &'a [T] : GetIndex<Idx>, Idx : SliceIndex<[T]> + Copy
{
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.as_slice().get(idx) }
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.as_slice().get_unchecked(idx) } }
}
impl<Idx,T,const N : usize> GetIndexMut<Idx> for [T;N] where for<'a> &'a mut[T] : GetIndexMut<Idx>, for<'a> &'a[T] : GetIndex<Idx>, Idx : SliceIndex<[T]> + Copy
{
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.as_mut_slice().get_mut(idx) }
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.as_mut_slice().get_unchecked_mut(idx) } }
}


impl<Idx,T> LookUp<Idx>    for Vec<T> where Idx : SliceIndex<[T]> + Copy { type LookUpOutput = <Self as Index::<Idx>>::Output; fn lookup(&self, k: Idx) -> Option<&Self::LookUpOutput> where Idx : Borrow<Idx> { self.get(k) } }
impl<Idx,T> LookUpMut<Idx> for Vec<T> where Idx : SliceIndex<[T]> + Copy { fn lookup_mut(&mut self, k: Idx) -> Option<&mut Self::LookUpOutput> where Idx : Borrow<Idx> { self.get_mut(k) } }

impl<Idx,T> GetIndex<Idx> for Vec<T> where Idx : SliceIndex<[T]> + Copy
{
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.as_slice().get(idx) }
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.as_slice().get_unchecked(idx) } }
}
impl<Idx,T> GetIndexMut<Idx> for Vec<T> where Idx : SliceIndex<[T]> + Copy
{
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.as_mut_slice().get_mut(idx) }
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.as_mut_slice().get_unchecked_mut(idx) } }
}


impl<T> LookUp<usize,usize>    for VecDeque<T> { type LookUpOutput = <Self as Index::<usize>>::Output; fn lookup(&self, k:  usize) -> Option<&Self::LookUpOutput> { self.get(k) } }
impl<T> LookUpMut<usize,usize> for VecDeque<T> { fn lookup_mut(&mut self, k: usize) -> Option<&mut Self::LookUpOutput> { self.get_mut(k) } }

impl<T> GetIndex<usize> for VecDeque<T>
{
    fn get(&self, idx : usize) -> Option<&Self::Output> { self.get(idx) }
}
impl<T> GetIndexMut<usize> for VecDeque<T>
{
    fn get_mut(&mut self, idx : usize) -> Option<&mut Self::Output> { self.get_mut(idx) }
}

impl<Idx> LookUp<Idx>    for str where Idx : SliceIndex<str> + Copy { type LookUpOutput = <Self as Index::<Idx>>::Output; fn lookup(&self, k: Idx) -> Option<&Self::LookUpOutput> where Idx : Borrow<Idx> { self.get(k) } }
impl<Idx> LookUpMut<Idx> for str where Idx : SliceIndex<str> + Copy { fn lookup_mut(&mut self, k: Idx) -> Option<&mut Self::LookUpOutput> where Idx : Borrow<Idx> { self.get_mut(k) } }

impl<Idx> GetIndex<Idx> for str where Idx : SliceIndex<str> + Copy
{
    fn get(&self, idx : Idx) -> Option<&Self::Output> { self.get(idx) }
    unsafe fn get_unchecked(&self, idx : Idx) -> &Self::Output { unsafe { self.get_unchecked(idx) } }
}
impl<Idx> GetIndexMut<Idx> for str where Idx : SliceIndex<str> + Copy
{
    fn get_mut(&mut self, idx : Idx) -> Option<&mut Self::Output> { self.get_mut(idx) }
    unsafe fn get_unchecked_mut(&mut self, idx : Idx) -> &mut Self::Output { unsafe { self.get_unchecked_mut(idx) } }
}