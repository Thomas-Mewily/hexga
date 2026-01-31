use crate::gen_seq::GenSeq;
use super::*;

pub mod prelude
{

}


/*
pub type GenHashMap =
pub type GenBTreeMap =

same for set...
*/

// TODO continue it

pub type GenMapID = GenMapIDOf<Generation>;
pub type GenMapIDOf<G> = GenIDOf<G>;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entry<K,V>
{
    pub key : K,
    pub value: V,
}
impl<K,V> Debug for Entry<K,V>
    where K: Debug, V: Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.key).field(&self.value).finish()
    }
}
impl<K,V> From<(K,V)> for Entry<K,V>
{
    fn from((key, value): (K,V)) -> Self {
        Self::new(key, value)
    }
}
impl<K,V> From<Entry<K,V>> for (K,V)
{
    fn from(kv: Entry<K,V>) -> Self {
        (kv.key, kv.value)
    }
}
impl<K,V> Entry<K,V>
{
    pub const fn new(key: K, value: V) -> Self { Self{ key, value }}
}


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GenMapOf<K,V,Gen/*=Generation*/,S>//=HashMap<K,V>>
    where
    K: Clone,
    Gen: IGeneration
{
    values: GenSeq<Entry<K,V>,Gen>,
    search: S, // HashMap<K,GenMapID>
}
impl<K,V,Gen,S> Default for GenMapOf<K,V,Gen,S>
    where
    K: Clone,
    Gen: IGeneration,
    S: Default
{
    fn default() -> Self {
        Self { values: Default::default(), search: Default::default() }
    }
}
impl<K,V,Gen,S> Debug for GenMapOf<K,V,Gen,S>
    where
    K: Clone,
    Gen: IGeneration,
    K: Debug, V: Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.values.iter()).finish()
    }
}
impl<K,V,Gen,S> Collection for GenMapOf<K,V,Gen,S>
    where
    K: Clone,
    Gen: IGeneration,
    S: Collection
{

}
impl<K,V,Gen,S> GenMapOf<K,V,Gen,S>
    where
    K: Clone,
    Gen: IGeneration,
{
    pub fn new() -> Self where S: Default
    {
        Self::___()
    }
}
impl<K,V,Gen,S> WithCapacity for GenMapOf<K,V,Gen,S>
    where
    K: Clone,
    Gen: IGeneration,
    S: WithCapacity, <S as WithCapacity>::Param : Default
{
    type Param=();
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self {
        Self{ values: GenSeq::with_capacity(capacity), search: S::with_capacity(capacity) }
    }
}
impl<K,V,Gen,S> Capacity for GenMapOf<K,V,Gen,S>
    where
    K: Clone,
    Gen: IGeneration,
    S: Capacity
{
    fn capacity(&self) -> usize { self.values.capacity() / 2 + self.search.capacity() / 2 }
}
impl<K,V,Gen,S> Reserve for GenMapOf<K,V,Gen,S>
    where
    K: Clone,
    Gen: IGeneration,
    S: Reserve + Length
{
    fn reserve(&mut self, additional: usize) {
        let total = self.capacity() + additional;
        self.values.reserve_total(total);
        self.search.reserve_total(total);
    }

    fn reserve_exact(&mut self, additional: usize) {
        let total = self.capacity() + additional;
        self.values.reserve_total_exact(total);
        self.search.reserve_total_exact(total);
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        let total = self.capacity() + additional;
        self.values.try_reserve_total(total)?;
        self.search.try_reserve_total(total)
    }

    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        let total = self.capacity() + additional;
        self.values.try_reserve_total_exact(total)?;
        self.search.try_reserve_total_exact(total)
    }
}


/*
Don't delete this comment

impl the trait

// Already Done
Capacity,

// Todo:
Extend, Truncate
Get, TryGet, GetMut, TryGetMut, GetManyMut,
Len,
IntoIter (for self, &self, &mut Self)
Insert
Remove,
*/