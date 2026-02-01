use crate::gen_seq::GenSeq;
use super::*;

pub mod prelude
{
    pub use super::{GenHashMap, GenBTreeMap};
}

pub type GenHashMap<K,V,S=RandomState> = GenMapOf<K,V,Generation,S>;
pub type GenBTreeMap<K,V> = GenMapOf<K,V,Generation,BTreeMap<K,V>>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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


#[derive(Clone)]
pub struct GenMapOf<K,V,Gen=Generation,S=HashMap<K,V>>
    where
    K: Clone,
    Gen: IGeneration
{
    values: GenSeq<Entry<K,V>,Gen>,
    search: S, // HashMap<K,GenMapID>
}

impl<K,V,Gen,S> PartialEq for GenMapOf<K,V,Gen,S>
where
    K: Clone,
    Gen: IGeneration,
    K: PartialEq,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<K,V,Gen,S> Eq for GenMapOf<K,V,Gen,S>
where
    K: Clone,
    Gen: IGeneration,
    K: Eq,
    V: Eq,
{

}

impl<K,V,Gen,S> Hash for GenMapOf<K,V,Gen,S>
where
    K: Clone,
    Gen: IGeneration,
    K: Hash,
    V: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.values.hash(state);
    }
}

#[cfg(feature = "serde")]
impl<K, V, Gen, St> Serialize for GenMapOf<K, V, Gen, St>
where
    K: Clone,
    Gen: IGeneration,
    K: Serialize,
    V: Serialize,
    Gen: Serialize,
    GenSeq<Entry<K, V>, Gen>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.values.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, K, V, Gen, St> Deserialize<'de> for GenMapOf<K, V, Gen, St>
where
    K: Clone,
    Gen: IGeneration,
    K: Deserialize<'de>,
    V: Deserialize<'de>,
    Gen: Deserialize<'de>,
    St: Default + Insert<K, GenIDOf<Gen>>,
    GenSeq<Entry<K, V>, Gen>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = GenSeq::<Entry<K, V>, Gen>::deserialize(deserializer)?;
        let mut search = St::default();

        for (id, entry) in values.iter()
        {
            if search.insert(entry.key.clone(), id).is_some()
            {
                return Err(serde::de::Error::custom(
                    "duplicate key found during GenMapOf deserialization",
                ));
            }
        }

        Ok(Self { values, search })
    }
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

impl<K,V,Gen,S> CollectionBijective for GenMapOf<K,V,Gen,S>
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

    #[inline(always)]
    pub fn len(&self) -> usize
    where
        Self: Length,
    {
        <Self as Length>::len(self)
    }

    #[inline(always)]
    pub fn get<Idx>(&self, index: Idx) -> Option<&V>
    where
        Self: Get<Idx, Output = V>,
    {
        <Self as Get<Idx>>::get(self, index)
    }

    #[inline(always)]
    pub fn get_mut<Idx>(&mut self, index: Idx) -> Option<&mut V>
    where
        Self: GetMut<Idx, Output = V>,
    {
        <Self as GetMut<Idx>>::get_mut(self, index)
    }

    #[inline(always)]
    pub fn try_get<Idx>(&self, index: Idx) -> Result<&V, <Self as TryGet<Idx>>::Error>
    where
        Self: TryGet<Idx, Output = V>,
    {
        <Self as TryGet<Idx>>::try_get(self, index)
    }

    #[inline(always)]
    pub fn try_get_mut<Idx>(&mut self, index: Idx) -> Result<&mut V, <Self as TryGet<Idx>>::Error>
    where
        Self: TryGetMut<Idx, Output = V>,
    {
        <Self as TryGetMut<Idx>>::try_get_mut(self, index)
    }

    #[inline(always)]
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        Self: Insert<K, V>,
    {
        <Self as Insert<K, V>>::insert(self, key, value)
    }

    #[inline(always)]
    pub fn remove<Idx>(&mut self, index: Idx) -> Option<V>
    where
        Self: Remove<Idx, Output = V>,
    {
        <Self as Remove<Idx>>::remove(self, index)
    }

    #[inline(always)]
    pub fn truncate(&mut self, len: usize)
    where
        Self: Truncate,
    {
        <Self as Truncate>::truncate(self, len)
    }

    pub fn iter(&self) -> Iter<'_, K, V, Gen> { self.into_iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V, Gen> { self.into_iter() }

    pub fn ids(&self) -> impl Iterator<Item = GenIDOf<Gen>> + '_
    {
        self.into_iter().map(|(id, _k, _v)| id)
    }

    pub fn values(&self) -> impl Iterator<Item = &V> + '_
    {
        self.into_iter().map(|(_id, _k, v)| v)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> + '_
    {
        self.into_iter().map(|(_id, _k, v)| v)
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

impl<K,V,Gen,S> Shrink for GenMapOf<K,V,Gen,S>
    where
    K: Clone,
    Gen: IGeneration,
    S: Shrink
{
    fn shrink_to_fit(&mut self) {
        self.values.shrink_to_fit();
        self.search.shrink_to_fit();
    }

    fn shrink_to(&mut self, min_capacity: usize) {
        self.values.shrink_to(min_capacity);
        self.search.shrink_to(min_capacity);
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
    S: Get<&'a K, Output = GenIDOf<Gen>>,
{
    type Output = V;

    #[inline(always)]
    fn get(&self, index: &'a K) -> Option<&Self::Output>
    {
        let id = *self.search.get(index)?;
        self.values.get(id).map(|entry| &entry.value)
    }
}

impl<'a, K, V, Gen, S> std::ops::Index<&'a K> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Get<&'a K, Output = GenIDOf<Gen>>,
{
    type Output = V;

    #[inline(always)]
    #[track_caller]
    fn index(&self, index: &'a K) -> &Self::Output { <Self as Get<&'a K>>::get_or_panic(self, index) }
}

impl<'a, K, V, Gen, S> TryGet<&'a K> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Get<&'a K, Output = GenIDOf<Gen>>,
{
    type Error = MissingKey<K>;

    fn try_get(&self, index: &'a K) -> Result<&Self::Output, Self::Error>
    {
        self.get(index)
            .ok_or_else(|| MissingKey::new(index.clone()))
    }
}

impl<K, V, Gen, S> Get<GenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Output = V;

    #[inline(always)]
    fn get(&self, index: GenIDOf<Gen>) -> Option<&Self::Output>
    {
        self.values.get(index).map(|entry| &entry.value)
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, index: GenIDOf<Gen>) -> &Self::Output
    {
        &unsafe { self.values.get_unchecked(index) }.value
    }
}

impl<K, V, Gen, S> std::ops::Index<GenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Output = V;

    #[inline(always)]
    #[track_caller]
    fn index(&self, index: GenIDOf<Gen>) -> &Self::Output
    {
        <Self as Get<GenIDOf<Gen>>>::get_or_panic(self, index)
    }
}

impl<K, V, Gen, S> TryGet<GenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Error = <GenSeq<Entry<K, V>, Gen> as TryGet<GenIDOf<Gen>>>::Error;

    fn try_get(&self, index: GenIDOf<Gen>) -> Result<&Self::Output, Self::Error>
    {
        self.values.try_get(index).map(|entry| &entry.value)
    }
}

impl<'a, K, V, Gen, S> GetMut<&'a K> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Get<&'a K, Output = GenIDOf<Gen>>,
{
    #[inline(always)]
    fn get_mut(&mut self, index: &'a K) -> Option<&mut Self::Output>
    {
        let id = *self.search.get(index)?;
        self.values.get_mut(id).map(|entry| &mut entry.value)
    }
}

impl<'a, K, V, Gen, S> std::ops::IndexMut<&'a K> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Get<&'a K, Output = GenIDOf<Gen>>,
{
    #[inline(always)]
    #[track_caller]
    fn index_mut(&mut self, index: &'a K) -> &mut Self::Output
    {
        <Self as GetMut<&'a K>>::get_mut_or_panic(self, index)
    }
}

impl<'a, K, V, Gen, S> TryGetMut<&'a K> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Get<&'a K, Output = GenIDOf<Gen>>,
{
    fn try_get_mut(&mut self, index: &'a K) -> Result<&mut Self::Output, Self::Error>
    {
        self.get_mut(index)
            .ok_or_else(|| MissingKey::new(index.clone()))
    }
}

impl<K, V, Gen, S> GetMut<GenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    #[inline(always)]
    fn get_mut(&mut self, index: GenIDOf<Gen>) -> Option<&mut Self::Output>
    {
        self.values.get_mut(index).map(|entry| &mut entry.value)
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(&mut self, index: GenIDOf<Gen>) -> &mut Self::Output
    {
        &mut unsafe { self.values.get_unchecked_mut(index) }.value
    }
}

impl<K, V, Gen, S> std::ops::IndexMut<GenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    #[inline(always)]
    #[track_caller]
    fn index_mut(&mut self, index: GenIDOf<Gen>) -> &mut Self::Output
    {
        <Self as GetMut<GenIDOf<Gen>>>::get_mut_or_panic(self, index)
    }
}

impl<K, V, Gen, S> TryGetMut<GenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    fn try_get_mut(&mut self, index: GenIDOf<Gen>) -> Result<&mut Self::Output, Self::Error>
    {
        self.values
            .try_get_mut(index)
            .map(|entry| &mut entry.value)
    }
}

impl<K, V, Gen, S> GetManyMut<GenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    fn try_get_many_mut<const N: usize>(
        &mut self,
        indices: [GenIDOf<Gen>; N],
    ) -> Result<[&mut Self::Output; N], ManyMutError>
    {
        self.values
            .try_get_many_mut(indices)
            .map(|entries| entries.map(|entry| &mut entry.value))
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<const N: usize>(
        &mut self,
        indices: [GenIDOf<Gen>; N],
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
    S: Insert<K, GenIDOf<Gen>> + for<'a> Remove<&'a K, Output = GenIDOf<Gen>>,
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
    S: Remove<&'a K, Output = GenIDOf<Gen>>,
{
    type Output = V;

    fn remove(&mut self, index: &'a K) -> Option<Self::Output>
    {
        let id = self.search.remove(index)?;
        let entry = self.values.remove(id).unwrap();
        Some(entry.value)
    }
}

impl<K, V, Gen, S> Remove<GenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: for<'a> Remove<&'a K, Output = GenIDOf<Gen>>,
{
    type Output = V;

    fn remove(&mut self, index: GenIDOf<Gen>) -> Option<Self::Output>
    {
        let entry = self.values.remove(index)?;
        self.search.remove(&entry.key).unwrap();
        Some(entry.value)
    }
}

impl<K, V, Gen, S> FromIterator<(K, V)> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Default + Insert<K, GenIDOf<Gen>> + for<'a> Remove<&'a K, Output = GenIDOf<Gen>>,
    Self: WithCapacity, <Self as WithCapacity>::Param: Default
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
    {
        let iter = iter.into_iter();
        let mut out = Self::with_capacity(iter.size_hint().0);
        out.extend(iter);
        out
    }
}

impl<K, V, Gen, S> From<GenSeq<Entry<K, V>, Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Default + Insert<K, GenIDOf<Gen>>
{
    fn from(values: GenSeq<Entry<K, V>, Gen>) -> Self
    {
        let mut search = S::default();
        for (id, entry) in values.iter()
        {
            search.insert(entry.key.clone(), id);
        }
        Self { values, search }
    }
}

impl<K, V, Gen, S> Extend<(K, V)> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Insert<K, GenIDOf<Gen>> + for<'a> Remove<&'a K, Output = GenIDOf<Gen>>,
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
    type Item = (GenIDOf<Gen>, K, V);

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
    type Item = (GenIDOf<Gen>, &'a K, &'a V);

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
    type Item = (GenIDOf<Gen>, &'a K, &'a mut V);

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
    type Item = (GenIDOf<Gen>, K, V);
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
    type Item = (GenIDOf<Gen>, &'a K, &'a V);
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
    type Item = (GenIDOf<Gen>, &'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V, Gen>;

    fn into_iter(self) -> Self::IntoIter
    {
        IterMut {
            iter: self.values.iter_mut(),
        }
    }
}
