use crate::gen_seq::GenSeq;
use super::*;

pub mod prelude
{
    pub use super::{CollectToGenMap, GenHashMap, GenBTreeMap};
}

pub type GenHashMap<K,V> = GenHashMapOf<K,V,RandomState>;
pub type GenHashMapOf<K,V,S=RandomState> = GenMapOf<K,V,Generation,HashMap<K,GenIDOf<V, Generation>,S>>;
pub type GenBTreeMap<K,V> = GenMapOf<K,V,Generation,BTreeMap<K,GenIDOf<V, Generation>>>;

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

/// A Generational Map Collection.
///
/// [`GenID`] stay stable, even after inserting the same key twice.
#[derive(Clone)]
pub struct GenMapOf<K,V,Gen=Generation,S=HashMap<K,V>>
    where
    K: Clone,
    Gen: IGeneration
{
    values: GenSeq<Entry<K,V>,Gen>,
    search: S, // Map<K,GenMapID>
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
    St: Default + Insert<K, GenIDOf<V, Gen>>,
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
            if search.insert(entry.key.clone(), id.typed()).is_some()
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
    pub fn insert(&mut self, key: K, value: V) -> (GenIDOf<V, Gen>, Option<V>)
    where
        S: Insert<K, GenIDOf<V, Gen>>,
    {
        self.insert_cyclic(key, |_| value)
    }

    #[track_caller]
    pub fn insert_cyclic<'s, F>(&'s mut self, key: K, init: F) -> (GenIDOf<V, Gen>, Option<V>)
    where
        F: FnOnce(GenIDOf<V, Gen>) -> V,
        S: Insert<K, GenIDOf<V, Gen>>,
    {
        let old_id = self.search.insert(key.clone(), GenIDOf::NULL);
        match old_id
        {
            Some(id) =>
            {
                let id = id.typed::<V>();
                let old_value = std::mem::replace(&mut self.values[id.typed()].value, init(id));
                self.search.insert(key, id);
                (id, Some(old_value))
            },
            None =>
            {
                let id = self.values.insert_cyclic(|id| Entry::new(key.clone(), init(id.typed())));
                let id = id.typed::<V>();
                self.search.insert(key, id);
                (id, None)
            },
        }
    }

    /*
    #[inline(always)]
    pub fn try_insert_cyclic<'s, F>(&'s mut self, key: K, init: F) -> Result<(GenIDOf<V, Gen>, Option<V>), ()>
    where
        F: FnOnce(GenIDOf<V, Gen>) -> V,
        S: Insert<K, GenIDOf<V, Gen>>,
    {
        let old_id = self.search.insert(key.clone(), GenIDOf::NULL);
        match old_id
        {
            Some(id) =>
            {
                let old_value = std::mem::replace(&mut self.values[id].value, init(id));
                self.search.insert(key, id);
                Ok((id, Some(old_value)))
            },
            None =>
            {
                let id = self.values.insert_cyclic(|id| Entry::new(key.clone(), init(id)));
                self.search.insert(key, id);
                Ok((id, None))
            },
        }
    }

    #[inline(always)]
    pub fn try_insert(&mut self, key: K, value: V) -> Result<(GenIDOf<V, Gen>, Option<V>), ()>
    where
        S: Insert<K, GenIDOf<V, Gen>>,
    {
        self.try_insert_cyclic(key, |_| value)
    }
    */

    #[inline(always)]
    pub fn remove<Idx>(&mut self, index: Idx) -> Option<V>
    where
        Self: Remove<Idx, Output = V>,
    {
        <Self as Remove<Idx>>::remove(self, index)
    }

    pub fn iter(&self) -> Iter<'_, K, V, Gen> { self.into_iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V, Gen> { self.into_iter() }

    pub fn ids(&self) -> impl Iterator<Item = GenIDOf<V, Gen>> + '_
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
    S: WithCapacity + Length, <S as WithCapacity>::Param : Default
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

impl<'a, Q, K, V, Gen, S> Get<&'a Q> for GenMapOf<K, V, Gen, S>
where
    Q: ?Sized,
    K: Clone + Borrow<Q>,
    Gen: IGeneration,
    S: Get<&'a Q, Output = GenIDOf<V, Gen>>,
{
    type Output = V;

    #[inline(always)]
    fn get(&self, index: &'a Q) -> Option<&Self::Output>
    {
        let id = *self.search.get(index)?;
        self.values.get(id.typed()).map(|entry| &entry.value)
    }
}

impl<'a, Q, K, V, Gen, S> Index<&'a Q> for GenMapOf<K, V, Gen, S>
where
    Q: ?Sized,
    K: Clone + Borrow<Q>,
    Gen: IGeneration,
    S: Get<&'a Q, Output = GenIDOf<V, Gen>>,
{
    type Output = V;

    #[inline(always)]
    #[track_caller]
    fn index(&self, index: &'a Q) -> &Self::Output { <Self as Get<&'a Q>>::get_or_panic(self, index) }
}

impl<'a, Q, K, V, Gen, S> TryGet<&'a Q> for GenMapOf<K, V, Gen, S>
where
    Q: ?Sized + Clone,
    K: Clone + Borrow<Q>,
    Gen: IGeneration,
    S: Get<&'a Q, Output = GenIDOf<V, Gen>>,
{
    type Error = MissingKey<Q>;

    fn try_get(&self, index: &'a Q) -> Result<&Self::Output, Self::Error>
    {
        self.get(index)
            .ok_or_else(|| MissingKey::new(index.clone()))
    }
}

impl<K, V, Gen, S> Get<UntypedGenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Output = V;

    #[inline(always)]
    fn get(&self, index: UntypedGenIDOf<Gen>) -> Option<&Self::Output>
    {
        self.get(index.typed())
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, index: UntypedGenIDOf<Gen>) -> &Self::Output
    {
        unsafe { self.get_unchecked(index.typed()) }
    }
}
impl<K, V, Gen, S> Get<GenIDOf<V, Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Output = V;

    #[inline(always)]
    fn get(&self, index: GenIDOf<V, Gen>) -> Option<&Self::Output>
    {
        self.values.get(index.typed()).map(|entry| &entry.value)
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, index: GenIDOf<V, Gen>) -> &Self::Output
    {
        &unsafe { self.values.get_unchecked(index.typed()) }.value
    }
}

impl<K, V, Gen, S> Index<GenIDOf<V, Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Output = V;

    #[inline(always)]
    #[track_caller]
    fn index(&self, index: GenIDOf<V, Gen>) -> &Self::Output
    {
        <Self as Get<GenIDOf<V, Gen>>>::get_or_panic(self, index)
    }
}
impl<K, V, Gen, S> Index<UntypedGenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Output = V;

    #[inline(always)]
    #[track_caller]
    fn index(&self, index: UntypedGenIDOf<Gen>) -> &Self::Output
    {
        self.index(index.typed())
    }
}

impl<K, V, Gen, S> TryGet<GenIDOf<V, Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Error = <GenSeq<Entry<K, V>, Gen> as TryGet<GenIDOf<Entry<K,V>, Gen>>>::Error;

    fn try_get(&self, index: GenIDOf<V, Gen>) -> Result<&Self::Output, Self::Error>
    {
        self.values.try_get(index.typed()).map(|entry| &entry.value)
    }
}
impl<K, V, Gen, S> TryGet<UntypedGenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Error = <GenSeq<Entry<K, V>, Gen> as TryGet<UntypedGenIDOf<Gen>>>::Error;

    fn try_get(&self, index: UntypedGenIDOf<Gen>) -> Result<&Self::Output, Self::Error>
    {
        self.try_get(index.typed())
    }
}

impl<'a, Q, K, V, Gen, S> GetMut<&'a Q> for GenMapOf<K, V, Gen, S>
where
    Q: ?Sized,
    K: Clone + Borrow<Q>,
    Gen: IGeneration,
    S: Get<&'a Q, Output = GenIDOf<V, Gen>>,
{
    #[inline(always)]
    fn get_mut(&mut self, index: &'a Q) -> Option<&mut Self::Output>
    {
        let id = *self.search.get(index)?;
        self.values.get_mut(id.typed()).map(|entry| &mut entry.value)
    }
}

impl<'a, Q, K, V, Gen, S> IndexMut<&'a Q> for GenMapOf<K, V, Gen, S>
where
    Q: ?Sized,
    K: Clone + Borrow<Q>,
    Gen: IGeneration,
    S: Get<&'a Q, Output = GenIDOf<V, Gen>>,
{
    #[inline(always)]
    #[track_caller]
    fn index_mut(&mut self, index: &'a Q) -> &mut Self::Output
    {
        <Self as GetMut<&'a Q>>::get_mut_or_panic(self, index)
    }
}

impl<'a, Q, K, V, Gen, S> TryGetMut<&'a Q> for GenMapOf<K, V, Gen, S>
where
    Q: ?Sized + Clone,
    K: Clone + Borrow<Q>,
    Gen: IGeneration,
    S: Get<&'a Q, Output = GenIDOf<V, Gen>>,
{
    fn try_get_mut(&mut self, index: &'a Q) -> Result<&mut Self::Output, Self::Error>
    {
        self.get_mut(index)
            .ok_or_else(|| MissingKey::new(index.clone()))
    }
}

impl<K, V, Gen, S> GetMut<GenIDOf<V, Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    #[inline(always)]
    fn get_mut(&mut self, index: GenIDOf<V, Gen>) -> Option<&mut Self::Output>
    {
        self.values.get_mut(index.typed()).map(|entry| &mut entry.value)
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(&mut self, index: GenIDOf<V, Gen>) -> &mut Self::Output
    {
        &mut unsafe { self.values.get_unchecked_mut(index.typed()) }.value
    }
}

impl<K, V, Gen, S> GetMut<UntypedGenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    #[inline(always)]
    fn get_mut(&mut self, index: UntypedGenIDOf<Gen>) -> Option<&mut Self::Output>
    {
        self.values.get_mut(index.typed()).map(|entry| &mut entry.value)
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(&mut self, index: UntypedGenIDOf<Gen>) -> &mut Self::Output
    {
        &mut unsafe { self.values.get_unchecked_mut(index.typed()) }.value
    }
}

impl<K, V, Gen, S> IndexMut<GenIDOf<V, Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    #[inline(always)]
    #[track_caller]
    fn index_mut(&mut self, index: GenIDOf<V, Gen>) -> &mut Self::Output
    {
        <Self as GetMut<GenIDOf<V, Gen>>>::get_mut_or_panic(self, index)
    }
}
impl<K, V, Gen, S> IndexMut<UntypedGenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    #[inline(always)]
    #[track_caller]
    fn index_mut(&mut self, index: UntypedGenIDOf<Gen>) -> &mut Self::Output
    {
        <Self as GetMut<UntypedGenIDOf<Gen>>>::get_mut_or_panic(self, index)
    }
}

impl<K, V, Gen, S> TryGetMut<GenIDOf<V, Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    fn try_get_mut(&mut self, index: GenIDOf<V, Gen>) -> Result<&mut Self::Output, Self::Error>
    {
        self.values
            .try_get_mut(index.typed())
            .map(|entry| &mut entry.value)
    }
}
impl<K, V, Gen, S> TryGetMut<UntypedGenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    #[inline(always)]
    fn try_get_mut(&mut self, index: UntypedGenIDOf<Gen>) -> Result<&mut Self::Output, Self::Error>
    {
        self.try_get_mut(index.typed())
    }
}

impl<K, V, Gen, S> GetManyMut<GenIDOf<V, Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    fn try_get_many_mut<const N: usize>(
        &mut self,
        indices: [GenIDOf<V, Gen>; N],
    ) -> Result<[&mut Self::Output; N], ManyMutError>
    {
        self.values
            .try_get_many_mut(indices.map(|i| i.typed()))
            .map(|entries| entries.map(|entry| &mut entry.value))
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<const N: usize>(
        &mut self,
        indices: [GenIDOf<V, Gen>; N],
    ) -> [&mut Self::Output; N]
    {
        unsafe { self.values
            .get_many_unchecked_mut(indices.map(|i| i.typed())) }
            .map(|entry| &mut entry.value)
    }
}
impl<K, V, Gen, S> GetManyMut<UntypedGenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    #[inline(always)]
    fn try_get_many_mut<const N: usize>(
        &mut self,
        indices: [UntypedGenIDOf<Gen>; N],
    ) -> Result<[&mut Self::Output; N], ManyMutError>
    {
        self.try_get_many_mut(indices.map(|i| i.typed()))
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<const N: usize>(
        &mut self,
        indices: [UntypedGenIDOf<Gen>; N],
    ) -> [&mut Self::Output; N]
    {
        unsafe { self.get_many_unchecked_mut(indices.map(|i| i.typed())) }
    }
}

impl<K, V, Gen, S> Insert<K, V> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Insert<K, GenIDOf<V, Gen>>,
{
    fn insert(&mut self, key: K, value: V) -> Option<V>
    {
        let (_, old) = self.insert_cyclic(key, |_| value);
        old
    }
}
/*
impl<K, V, Gen, S> Insert<K, V> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: for<'a> Remove<&'a K, Output = GenIDOf<V, Gen>> + Insert<K, GenIDOf<V, Gen>>,
{
    fn insert(&mut self, key: K, value: V) -> Option<V>
    {
        let (_, old) = self.insert_cyclic(key, |_| value);
        old
    }
}
*/

impl<'a, Q, K, V, Gen, S> Remove<&'a Q> for GenMapOf<K, V, Gen, S>
where
    Q: ?Sized,
    K: Clone + Borrow<Q>,
    Gen: IGeneration,
    S: Remove<&'a Q, Output = GenIDOf<V, Gen>>,
{
    type Output = V;

    fn remove(&mut self, index: &'a Q) -> Option<Self::Output>
    {
        let id = self.search.remove(index)?;
        let entry = self.values.remove(id.typed()).unwrap();
        Some(entry.value)
    }
}

impl<K, V, Gen, S> Remove<GenIDOf<V, Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: for<'a> Remove<&'a K, Output = GenIDOf<V, Gen>>,
{
    type Output = V;

    fn remove(&mut self, index: GenIDOf<V, Gen>) -> Option<Self::Output>
    {
        let entry = self.values.remove(index.typed())?;
        self.search.remove(&entry.key).unwrap();
        Some(entry.value)
    }
}
impl<K, V, Gen, S> Remove<UntypedGenIDOf<Gen>> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: for<'a> Remove<&'a K, Output = GenIDOf<V, Gen>>,
{
    type Output = V;

    fn remove(&mut self, index: UntypedGenIDOf<Gen>) -> Option<Self::Output>
    {
        self.remove(index.typed())
    }
}

impl<K, V, Gen, S> FromIterator<(K, V)> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Default + Insert<K, GenIDOf<V, Gen>>,
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
    S: Default + Insert<K, GenIDOf<V, Gen>>
{
    fn from(values: GenSeq<Entry<K, V>, Gen>) -> Self
    {
        let mut search = S::default();
        for (id, entry) in values.iter()
        {
            search.insert(entry.key.clone(), id.typed());
        }
        Self { values, search }
    }
}

impl<K, V, Gen, S> Extend<(K, V)> for GenMapOf<K, V, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Insert<K, GenIDOf<V, Gen>>,
    Self: Reserve,
{
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I)
    {
        let it = iter.into_iter();
        self.reserve(it.size_hint().0);
        for (k, v) in it
        {
            Insert::insert(self, k, v);
        }
    }
}

pub struct IntoIter<K, V, Gen>
where
    Gen: IGeneration,
{
    iter: crate::gen_seq::IntoIter<Entry<K,V>,std::vec::IntoIter<crate::gen_seq::Entry<Entry<K, V>, Gen>>, Gen>,
}

impl<K, V, Gen> Iterator for IntoIter<K, V, Gen>
where
    Gen: IGeneration,
{
    type Item = (GenIDOf<V, Gen>, K, V);

    fn next(&mut self) -> Option<Self::Item>
    {
        let (id, entry) = self.iter.next()?;
        Some((id.typed(), entry.key, entry.value))
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
        Some((id.typed(), entry.key, entry.value))
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
    type Item = (GenIDOf<V, Gen>, &'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item>
    {
        let (id, entry) = self.iter.next()?;
        Some((id.typed(), &entry.key, &entry.value))
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
        Some((id.typed(), &entry.key, &entry.value))
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
    type Item = (GenIDOf<V, Gen>, &'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item>
    {
        let (id, entry) = self.iter.next()?;
        Some((id.typed(), &entry.key, &mut entry.value))
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
        Some((id.typed(), &entry.key, &mut entry.value))
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
    type Item = (GenIDOf<V, Gen>, K, V);
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
    type Item = (GenIDOf<V, Gen>, &'a K, &'a V);
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
    type Item = (GenIDOf<V, Gen>, &'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V, Gen>;

    fn into_iter(self) -> Self::IntoIter
    {
        IterMut {
            iter: self.values.iter_mut(),
        }
    }
}

pub trait CollectToGenMap<K, V, S = RandomState>: Sized + IntoIterator<Item = (K, V)>
where
    K: Clone,
{
    fn to_genmap(self) -> GenMapOf<K, V, Generation, S>
    where
        GenMapOf<K, V, Generation, S>: WithCapacity,
        <GenMapOf<K, V, Generation, S> as WithCapacity>::Param: Default,
        S: Default + Insert<K, GenIDOf<V, Generation>>,
    {
        self.into_iter().collect()
    }
}
impl<I, K, V, S> CollectToGenMap<K, V, S> for I
where
    I: IntoIterator<Item = (K, V)>,
    K: Clone,
{}
