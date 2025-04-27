use crate::*;

// Eq is imply by Ord, but I prefer to make sure this is visible
/// A key that can be used in an HashMap (Hash + Eq), but also in a BTreeMap (Ord + Eq)
pub trait UniversalKey : Hash + Eq + Ord {}
impl<T> UniversalKey for T where T: Hash + Eq + Ord {}

/// HashMap, BTreeMap... but also impl for GetIndexMut like Vector
pub trait LookUp<K, Q=K> where Q : ?Sized
{
    type LookUpOutput : ?Sized;
    fn lookup(&self, k: &Q) -> Option<&Self::LookUpOutput> where K : Borrow<Q>;
}

pub trait LookUpMut<K, Q=K> : LookUp<K, Q> where Q : ?Sized
{
    fn lookup_mut(&mut self, k: &Q) -> Option<&mut Self::LookUpOutput> where K : Borrow<Q>;
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


impl<K,V,S,Q> LookUp<K, Q> for HashMap<K,V,S> where Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    type LookUpOutput=V;
    fn lookup(&self, k: &Q) -> Option<&Self::LookUpOutput> where K : Borrow<Q> { self.get(k) }
}

impl<K,V,S,Q> LookUpMut<K, Q> for HashMap<K,V,S> where Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    fn lookup_mut(&mut self, k: &Q) -> Option<&mut Self::LookUpOutput> where K : Borrow<Q> { self.get_mut(k) }
}

impl<K,V,Q> LookUp<K, Q> for BTreeMap<K,V> where Q : ?Sized + Ord, K: Ord
{
    type LookUpOutput=V;
    fn lookup(&self, k: &Q) -> Option<&Self::LookUpOutput> where K : Borrow<Q> { self.get(k) }
}

impl<K,V,Q> LookUpMut<K, Q> for BTreeMap<K,V> where Q : ?Sized + Ord, K: Ord
{
    fn lookup_mut(&mut self, k: &Q) -> Option<&mut Self::LookUpOutput> where K : Borrow<Q> { self.get_mut(k) }
}

impl<K,S,Q> LookUp<K, Q> for HashSet<K,S> where Q : ?Sized + Hash + Eq, K: Eq + Hash, S: BuildHasher
{
    type LookUpOutput=K;
    fn lookup(&self, k: &Q) -> Option<&Self::LookUpOutput> where K : Borrow<Q> { self.get(k) }
}

impl<K,Q> LookUp<K, Q> for BTreeSet<K> where Q : ?Sized + Ord, K: Ord
{
    type LookUpOutput=K;
    fn lookup(&self, k: &Q) -> Option<&Self::LookUpOutput> where K : Borrow<Q> { self.get(k) }
}
