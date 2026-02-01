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
pub struct GenMapOf<K,V,Gen=Generation,S=HashMap<K,V>>
    where
    K: Clone,
    Gen: IGeneration
{
    values: GenSeq<Entry<K,V>,Gen>,
    search: S, // HashMap<K,GenMapID>
}


impl<K,V,Gen,S> AsRef<GenSeq<Entry<K,V>,Gen>> for GenMapOf<K,V,Gen,S>
where
    K: Clone,
    Gen: IGeneration
{
    fn as_ref(&self) -> &GenSeq<Entry<K,V>,Gen> {
        &self.values
    }
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
        let r = self.values.try_reserve_total(total);
        self.search.try_reserve_total(total)?;
        r
    }

    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        let total = self.capacity() + additional;
        let r = self.values.try_reserve_total_exact(total);
        self.search.try_reserve_total_exact(total)?;
        r
    }
}

impl<K, V, Gen, S> Length for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    #[inline(always)]
    fn len(&self) -> usize { self.values.len() }
}

impl<'a, K, V, Gen, S> Get<&'a K> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Get<&'a K, Output = GenMapIDOf<Gen>>,
{
    type Output = V;

    fn get(&self, index: &'a K) -> Option<&Self::Output>
    {
        let id = *self.search.get(index)?;
        self.values.get(id).map(|entry| &entry.value)
    }
}

impl<'a, K, V, Gen, S> TryGet<&'a K> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Get<&'a K, Output = GenMapIDOf<Gen>>,
{
    type Error = MissingKey<K>;

    fn try_get(&self, index: &'a K) -> Result<&Self::Output, Self::Error>
    {
        self.get(index)
            .ok_or_else(|| MissingKey::new(index.clone()))
    }
}

impl<K, V, Gen, S> Get<GenMapIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Output = V;

    fn get(&self, index: GenMapIDOf<Gen>) -> Option<&Self::Output>
    {
        self.values.get(index).map(|entry| &entry.value)
    }

    #[inline(always)]
    unsafe fn get_unchecked(&self, index: GenMapIDOf<Gen>) -> &Self::Output
    {
        &unsafe { self.values.get_unchecked(index) }.value
    }
}

impl<K, V, Gen, S> TryGet<GenMapIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Error = <GenSeq<Entry<K, V>, Gen> as TryGet<GenMapIDOf<Gen>>>::Error;

    fn try_get(&self, index: GenMapIDOf<Gen>) -> Result<&Self::Output, Self::Error>
    {
        self.values.try_get(index).map(|entry| &entry.value)
    }
}

impl<'a, K, V, Gen, S> GetMut<&'a K> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Get<&'a K, Output = GenMapIDOf<Gen>>,
{
    fn get_mut(&mut self, index: &'a K) -> Option<&mut Self::Output>
    {
        let id = *self.search.get(index)?;
        self.values.get_mut(id).map(|entry| &mut entry.value)
    }
}

impl<'a, K, V, Gen, S> TryGetMut<&'a K> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Get<&'a K, Output = GenMapIDOf<Gen>>,
{
    fn try_get_mut(&mut self, index: &'a K) -> Result<&mut Self::Output, Self::Error>
    {
        self.get_mut(index)
            .ok_or_else(|| MissingKey::new(index.clone()))
    }
}

impl<K, V, Gen, S> GetMut<GenMapIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    fn get_mut(&mut self, index: GenMapIDOf<Gen>) -> Option<&mut Self::Output>
    {
        self.values.get_mut(index).map(|entry| &mut entry.value)
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, index: GenMapIDOf<Gen>) -> &mut Self::Output
    {
        &mut unsafe { self.values.get_unchecked_mut(index) }.value
    }
}

impl<K, V, Gen, S> TryGetMut<GenMapIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    fn try_get_mut(&mut self, index: GenMapIDOf<Gen>) -> Result<&mut Self::Output, Self::Error>
    {
        self.values
            .try_get_mut(index)
            .map(|entry| &mut entry.value)
    }
}

impl<K, V, Gen, S> GetManyMut<GenMapIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    fn try_get_many_mut<const N: usize>(
        &mut self,
        indices: [GenMapIDOf<Gen>; N],
    ) -> Result<[&mut Self::Output; N], ManyMutError>
    {
        self.values
            .try_get_many_mut(indices)
            .map(|entries| entries.map(|entry| &mut entry.value))
    }

    #[inline(always)]
    unsafe fn get_many_unchecked_mut<const N: usize>(
        &mut self,
        indices: [GenMapIDOf<Gen>; N],
    ) -> [&mut Self::Output; N]
    {
        unsafe { self.values
            .get_many_unchecked_mut(indices) }
            .map(|entry| &mut entry.value)
    }
}

impl<K, V, Gen, S> Insert<K, V> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Insert<K, GenMapIDOf<Gen>> + for<'a> Remove<&'a K, Output = GenMapIDOf<Gen>>,
{
    fn insert(&mut self, key: K, value: V) -> Option<V>
    {
        let old = self
            .search
            .remove(&key)
            .and_then(|old_id| self.values.remove(old_id))
            .map(|old_entry| old_entry.value);

        let id = self.values.insert(Entry { key: key.clone(), value });
        self.search.insert(key, id);
        old
    }
}

impl<'a, K, V, Gen, S> Remove<&'a K> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Remove<&'a K, Output = GenMapIDOf<Gen>>,
{
    type Output = V;

    fn remove(&mut self, index: &'a K) -> Option<Self::Output>
    {
        let id = self.search.remove(index)?;
        let entry = self.values.remove(id)?;
        Some(entry.value)
    }
}

impl<K, V, Gen, S> Remove<GenMapIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: for<'a> Remove<&'a K, Output = GenMapIDOf<Gen>>,
{
    type Output = V;

    fn remove(&mut self, index: GenMapIDOf<Gen>) -> Option<Self::Output>
    {
        let entry = self.values.remove(index)?;
        self.search.remove(&entry.key);
        Some(entry.value)
    }
}

impl<K, V, Gen, S> Extend<(K, V)> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Insert<K, GenMapIDOf<Gen>> + for<'a> Remove<&'a K, Output = GenMapIDOf<Gen>>,
{
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I)
    {
        for (k, v) in iter
        {
            Insert::insert(self, k, v);
        }
    }
}

pub struct IntoIter<K, V, Gen>
where
    Gen: IGeneration,
{
    iter: crate::gen_seq::IntoIter<std::vec::IntoIter<crate::gen_seq::Entry<Entry<K, V>, Gen>>, Gen>,
}

impl<K, V, Gen> Iterator for IntoIter<K, V, Gen>
where
    Gen: IGeneration,
{
    type Item = (GenMapIDOf<Gen>, K, V);

    fn next(&mut self) -> Option<Self::Item>
    {
        let (id, entry) = self.iter.next()?;
        Some((id, entry.key, entry.value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<K, V, Gen> DoubleEndedIterator for IntoIter<K, V, Gen>
where
    Gen: IGeneration,
{
    fn next_back(&mut self) -> Option<Self::Item>
    {
        let (id, entry) = self.iter.next_back()?;
        Some((id, entry.key, entry.value))
    }
}

impl<K, V, Gen> FusedIterator for IntoIter<K, V, Gen> where Gen: IGeneration {}
impl<K, V, Gen> ExactSizeIterator for IntoIter<K, V, Gen>
where
    Gen: IGeneration,
{
    fn len(&self) -> usize { self.iter.len() }
}

pub struct Iter<'a, K, V, Gen>
where
    Gen: IGeneration,
{
    iter: crate::gen_seq::Iter<'a, Entry<K, V>, Gen>,
}

impl<'a, K, V, Gen> Iterator for Iter<'a, K, V, Gen>
where
    Gen: IGeneration,
{
    type Item = (GenMapIDOf<Gen>, &'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item>
    {
        let (id, entry) = self.iter.next()?;
        Some((id, &entry.key, &entry.value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<'a, K, V, Gen> DoubleEndedIterator for Iter<'a, K, V, Gen>
where
    Gen: IGeneration,
{
    fn next_back(&mut self) -> Option<Self::Item>
    {
        let (id, entry) = self.iter.next_back()?;
        Some((id, &entry.key, &entry.value))
    }
}

impl<'a, K, V, Gen> FusedIterator for Iter<'a, K, V, Gen> where Gen: IGeneration {}
impl<'a, K, V, Gen> ExactSizeIterator for Iter<'a, K, V, Gen>
where
    Gen: IGeneration,
{
    fn len(&self) -> usize { self.iter.len() }
}

pub struct IterMut<'a, K, V, Gen>
where
    Gen: IGeneration,
{
    iter: crate::gen_seq::IterMut<'a, Entry<K, V>, Gen>,
}

impl<'a, K, V, Gen> Iterator for IterMut<'a, K, V, Gen>
where
    Gen: IGeneration,
{
    type Item = (GenMapIDOf<Gen>, &'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item>
    {
        let (id, entry) = self.iter.next()?;
        Some((id, &entry.key, &mut entry.value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<'a, K, V, Gen> DoubleEndedIterator for IterMut<'a, K, V, Gen>
where
    Gen: IGeneration,
{
    fn next_back(&mut self) -> Option<Self::Item>
    {
        let (id, entry) = self.iter.next_back()?;
        Some((id, &entry.key, &mut entry.value))
    }
}

impl<'a, K, V, Gen> FusedIterator for IterMut<'a, K, V, Gen> where Gen: IGeneration {}
impl<'a, K, V, Gen> ExactSizeIterator for IterMut<'a, K, V, Gen>
where
    Gen: IGeneration,
{
    fn len(&self) -> usize { self.iter.len() }
}

impl<K, V, Gen, S> IntoIterator for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
{
    type Item = (GenMapIDOf<Gen>, K, V);
    type IntoIter = IntoIter<K, V, Gen>;

    fn into_iter(self) -> Self::IntoIter
    {
        IntoIter {
            iter: self.values.into_iter(),
        }
    }
}

impl<'a, K, V, Gen, S> IntoIterator for &'a GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
{
    type Item = (GenMapIDOf<Gen>, &'a K, &'a V);
    type IntoIter = Iter<'a, K, V, Gen>;

    fn into_iter(self) -> Self::IntoIter
    {
        Iter {
            iter: self.values.iter(),
        }
    }
}

impl<'a, K, V, Gen, S> IntoIterator for &'a mut GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
{
    type Item = (GenMapIDOf<Gen>, &'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V, Gen>;

    fn into_iter(self) -> Self::IntoIter
    {
        IterMut {
            iter: self.values.iter_mut(),
        }
    }
}


/*
Don't delete this comment

impl the trait

// Already Done
Capacity,
Get, TryGet, GetMut, TryGetMut, GetManyMut,
Length,
IntoIter (for self, &self, &mut Self)

// Todo:
Extend, Truncate
Insert
Remove,
*/