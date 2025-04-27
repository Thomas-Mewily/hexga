use crate::*;

// Eq is imply by Ord, but I prefer to make sure this is visible
/// A key that can be used in an HashMap (Hash + Eq), but also in a BTreeMap (Ord + Eq)
pub trait UniversalKey : Hash + Eq + Ord {}
impl<T> UniversalKey for T where T: Hash + Eq + Ord {}

/// HashMap, BTreeMap... but also impl for GetIndexMut like Vector
pub trait LookUp<K, Q=K>
{
    type LookUpOutput : ?Sized;
    fn lookup(&self, k: Q) -> Option<&Self::LookUpOutput> ;
}

pub trait LookUpMut<K, Q=K> : LookUp<K, Q>
{
    fn lookup_mut(&mut self, k: Q) -> Option<&mut Self::LookUpOutput>;

    
    
    /// Do nothings if the value don't exist
    fn set(&mut self, k : Q, value : Self::LookUpOutput) -> bool where Self::LookUpOutput : Sized { self.lookup_mut(k).map(|v| *v = value).is_some() }
    
    /// Panics if the value don't exist
    fn set_or_panic(&mut self, k : Q, value : Self::LookUpOutput) -> &mut Self where Self::LookUpOutput : Sized { *self.lookup_mut(k).unwrap() = value; self }
    
    fn replace(&mut self, k : Q, value : Self::LookUpOutput) -> Option<Self::LookUpOutput> where Self::LookUpOutput : Sized { self.lookup_mut(k).map(|v| std::mem::replace(v, value)) }
    /// Panics if the value don't exist
    fn replace_or_panic(&mut self, k : Q, value : Self::LookUpOutput) -> Self::LookUpOutput where Self::LookUpOutput : Sized { std::mem::replace(self.lookup_mut(k).expect("invalid index/key"), value) }
    
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

/* 
impl<T,Idx> LoopUp<Idx,Idx> for T where T : GetIndex<Idx>, Idx : Copy
{
    type Output=<Self as Index::<Idx>>::Output;
    fn get(&self, k: &Idx) -> Option<&Self::Output> where Idx : Borrow<Idx> { self.get(*k) }
}

impl<T,Idx> LoopUpMut<Idx,Idx> for T where T : GetIndexMut<Idx>, Idx : Copy
{
    fn get_mut(&mut self, k: &Idx) -> Option<&mut Self::Output> where Idx : Borrow<Idx> { self.get_mut(*k) }
}
*/


impl<K,V,S,Q> LookUp<K, &Q> for HashMap<K,V,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    type LookUpOutput=V;
    fn lookup(&self, k: &Q) -> Option<&Self::LookUpOutput> { self.get(k) }
}

impl<K,V,S,Q> LookUpMut<K, &Q> for HashMap<K,V,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    fn lookup_mut(&mut self, k: &Q) -> Option<&mut Self::LookUpOutput> where K : Borrow<Q> { self.get_mut(k) }
}

impl<K,V,Q> LookUp<K, &Q> for BTreeMap<K,V> where K : Borrow<Q>, Q : ?Sized + Ord, K: Ord
{
    type LookUpOutput=V;
    fn lookup(&self, k: &Q) -> Option<&Self::LookUpOutput> where K : Borrow<Q> { self.get(k) }
}

impl<K,V,Q> LookUpMut<K, &Q> for BTreeMap<K,V> where K : Borrow<Q>, Q : ?Sized + Ord, K: Ord
{
    fn lookup_mut(&mut self, k: &Q) -> Option<&mut Self::LookUpOutput> where K : Borrow<Q> { self.get_mut(k) }
}

impl<K,S,Q> LookUp<K, &Q> for HashSet<K,S> where K : Borrow<Q>, Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    type LookUpOutput=K;
    fn lookup(&self, k: &Q) -> Option<&Self::LookUpOutput> where K : Borrow<Q> { self.get(k) }
}

impl<K,Q> LookUp<K, &Q> for BTreeSet<K> where K : Borrow<Q>, Q : ?Sized + Ord, K: Ord
{
    type LookUpOutput=K;
    fn lookup(&self, k: &Q) -> Option<&Self::LookUpOutput> where K : Borrow<Q> { self.get(k) }
}
