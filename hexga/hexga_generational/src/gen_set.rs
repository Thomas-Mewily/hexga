use crate::gen_seq::GenSeq;
use super::*;

pub mod prelude
{
    pub use super::{GenBTreeSet, GenHashSet};
}

pub type GenHashSet<K, S = RandomState> = GenSetOf<K, Generation, HashMap<K, GenIDOf<Generation>, S>>;
pub type GenBTreeSet<K> = GenSetOf<K, Generation, BTreeMap<K, GenIDOf<Generation>>>;

#[derive(Clone)]
pub struct GenSetOf<K, Gen = Generation, S = HashMap<K, GenIDOf<Gen>>>
where
    K: Clone,
    Gen: IGeneration,
{
    values: GenSeq<K, Gen>,
    search: S,
}

impl<K, Gen, S> PartialEq for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool { self.values == other.values }
}

impl<K, Gen, S> Eq for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    K: Eq,
{
}

impl<K, Gen, S> Hash for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    K: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) { self.values.hash(state); }
}

#[cfg(feature = "serde")]
impl<K, Gen, St> Serialize for GenSetOf<K, Gen, St>
where
    K: Clone,
    Gen: IGeneration,
    K: Serialize,
    Gen: Serialize,
    GenSeq<K, Gen>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.values.serialize(serializer)
    }
}

impl<K, Gen, S> From<GenSeq<K, Gen>> for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Default + Insert<K, GenIDOf<Gen>>,
{
    fn from(values: GenSeq<K, Gen>) -> Self
    {
        let mut search = S::default();

        for (id, key) in values.iter()
        {
            let prev = search.insert(key.clone(), id);
            assert!(
                prev.is_none(),
                "duplicate key found during GenSetOf conversion from GenSeq"
            );
        }

        Self { values, search }
    }
}

#[cfg(feature = "serde")]
impl<'de, K, Gen, St> Deserialize<'de> for GenSetOf<K, Gen, St>
where
    K: Clone,
    Gen: IGeneration,
    K: Deserialize<'de>,
    Gen: Deserialize<'de>,
    St: Default + Insert<K, GenIDOf<Gen>>,
    GenSeq<K, Gen>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = GenSeq::<K, Gen>::deserialize(deserializer)?;
        let mut search = St::default();

        for (id, key) in values.iter()
        {
            if search.insert(key.clone(), id).is_some()
            {
                return Err(serde::de::Error::custom(
                    "duplicate key found during GenSetOf deserialization",
                ));
            }
        }

        Ok(Self { values, search })
    }
}

impl<K, Gen, S> AsRef<GenSeq<K, Gen>> for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
{
    fn as_ref(&self) -> &GenSeq<K, Gen> { &self.values }
}

impl<K, Gen, S> Default for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Default,
{
    fn default() -> Self { Self { values: Default::default(), search: Default::default() } }
}

impl<K, Gen, S> Debug for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    K: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.debug_set().entries(self.values.iter()).finish() }
}

impl<K, Gen, S> Collection for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
}

impl<K, Gen, S> CollectionBijective for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
}

impl<K, Gen, S> GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
{
    pub fn new() -> Self
    where
        S: Default,
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
    pub fn get<Idx>(&self, index: Idx) -> Option<&K>
    where
        Self: Get<Idx, Output = K>,
    {
        <Self as Get<Idx>>::get(self, index)
    }

    #[inline(always)]
    pub fn try_get<Idx>(&self, index: Idx) -> Result<&K, <Self as TryGet<Idx>>::Error>
    where
        Self: TryGet<Idx, Output = K>,
    {
        <Self as TryGet<Idx>>::try_get(self, index)
    }

    #[inline(always)]
    pub fn insert(&mut self, key: K) -> Option<()>
    where
        Self: Insert<K, ()>,
    {
        <Self as Insert<K, ()>>::insert(self, key, ())
    }

    #[inline(always)]
    pub fn remove<Idx>(&mut self, index: Idx) -> Option<<Self as Remove<Idx>>::Output>
    where
        Self: Remove<Idx>,
    {
        <Self as Remove<Idx>>::remove(self, index)
    }

    pub fn iter(&self) -> Iter<'_, K, Gen> { self.into_iter() }
    pub fn ids(&self) -> impl Iterator<Item = GenIDOf<Gen>> + '_ { self.into_iter().map(|(id, _k)| id) }
    pub fn values(&self) -> impl Iterator<Item = &K> + '_ { self.into_iter().map(|(_id, k)| k) }
}

impl<K, Gen, S> WithCapacity for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: WithCapacity,
    <S as WithCapacity>::Param: Default,
{
    type Param = ();
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self
    {
        Self {
            values: GenSeq::with_capacity(capacity),
            search: S::with_capacity(capacity),
        }
    }
}

impl<K, Gen, S> Capacity for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Capacity,
{
    fn capacity(&self) -> usize { self.values.capacity() / 2 + self.search.capacity() / 2 }
}

impl<K, Gen, S> Reserve for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Reserve + Length,
{
    fn reserve(&mut self, additional: usize)
    {
        let total = self.capacity() + additional;
        self.values.reserve_total(total);
        self.search.reserve_total(total);
    }

    fn reserve_exact(&mut self, additional: usize)
    {
        let total = self.capacity() + additional;
        self.values.reserve_total_exact(total);
        self.search.reserve_total_exact(total);
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError>
    {
        let total = self.capacity() + additional;
        let r = self.values.try_reserve_total(total);
        self.search.try_reserve_total(total)?;
        r
    }

    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError>
    {
        let total = self.capacity() + additional;
        let r = self.values.try_reserve_total_exact(total);
        self.search.try_reserve_total_exact(total)?;
        r
    }
}

impl<K, Gen, S> Shrink for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Shrink,
{
    fn shrink_to_fit(&mut self)
    {
        self.values.shrink_to_fit();
        self.search.shrink_to_fit();
    }

    fn shrink_to(&mut self, min_capacity: usize)
    {
        self.values.shrink_to(min_capacity);
        self.search.shrink_to(min_capacity);
    }
}

impl<K, Gen, S> Length for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    fn len(&self) -> usize { self.values.len() }
}

impl<'a, Q, K, Gen, S> Get<&'a Q> for GenSetOf<K, Gen, S>
where
    Q: ?Sized,
    K: Clone + Borrow<Q>,
    Gen: IGeneration,
    S: Get<&'a Q, Output = GenIDOf<Gen>>,
{
    type Output = K;

    fn get(&self, index: &'a Q) -> Option<&Self::Output>
    {
        let id = *self.search.get(index)?;
        self.values.get(id)
    }
}

impl<'a, Q, K, Gen, S> TryGet<&'a Q> for GenSetOf<K, Gen, S>
where
    Q: ?Sized + Clone,
    K: Clone + Borrow<Q>,
    Gen: IGeneration,
    S: Get<&'a Q, Output = GenIDOf<Gen>>,
{
    type Error = MissingKey<Q>;
    fn try_get(&self, index: &'a Q) -> Result<&Self::Output, Self::Error>
    {
        self.get(index).ok_or_else(|| MissingKey::new(index.clone()))
    }
}

impl<K, Gen, S> Get<GenIDOf<Gen>> for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Output = K;
    fn get(&self, index: GenIDOf<Gen>) -> Option<&Self::Output> { self.values.get(index) }
    unsafe fn get_unchecked(&self, index: GenIDOf<Gen>) -> &Self::Output
    {
        unsafe { self.values.get_unchecked(index) }
    }
}

impl<K, Gen, S> TryGet<GenIDOf<Gen>> for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Collection,
{
    type Error = <GenSeq<K, Gen> as TryGet<GenIDOf<Gen>>>::Error;
    fn try_get(&self, index: GenIDOf<Gen>) -> Result<&Self::Output, Self::Error>
    {
        self.values.try_get(index)
    }
}

impl<K, Gen, S> Insert<K, ()> for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Insert<K, GenIDOf<Gen>> + for<'a> Remove<&'a K, Output = GenIDOf<Gen>>,
{
    fn insert(&mut self, key: K, _value: ()) -> Option<()>
    {
        let existed = self
            .search
            .remove(&key)
            .and_then(|old_id| self.values.remove(old_id))
            .is_some();
        let id = self.values.insert(key.clone());
        self.search.insert(key, id);
        existed.then_some(())
    }
}

impl<'a, Q, K, Gen, S> Remove<&'a Q> for GenSetOf<K, Gen, S>
where
    Q: ?Sized,
    K: Clone + Borrow<Q>,
    Gen: IGeneration,
    S: Remove<&'a Q, Output = GenIDOf<Gen>>,
{
    type Output = ();
    fn remove(&mut self, index: &'a Q) -> Option<Self::Output>
    {
        let id = self.search.remove(index)?;
        self.values.remove(id).map(|_| ())
    }
}

impl<K, Gen, S> Remove<GenIDOf<Gen>> for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: for<'a> Remove<&'a K, Output = GenIDOf<Gen>>,
{
    type Output = K;
    fn remove(&mut self, index: GenIDOf<Gen>) -> Option<Self::Output>
    {
        let key = self.values.remove(index)?;
        self.search.remove(&key);
        Some(key)
    }
}

impl<K, Gen, S> Extend<K> for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Insert<K, GenIDOf<Gen>> + for<'a> Remove<&'a K, Output = GenIDOf<Gen>>,
{
    fn extend<I: IntoIterator<Item = K>>(&mut self, iter: I)
    {
        for k in iter
        {
            let _ = Insert::insert(self, k, ());
        }
    }
}

impl<K, Gen, S> FromIterator<K> for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
    S: Default + Insert<K, GenIDOf<Gen>> + for<'a> Remove<&'a K, Output = GenIDOf<Gen>>,
{
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self
    {
        let mut out = Self::default();
        out.extend(iter);
        out
    }
}


#[derive(Debug, Clone)]
pub struct IntoIter<K, Gen>
where
    Gen: IGeneration,
{
    iter: crate::gen_seq::IntoIter<std::vec::IntoIter<crate::gen_seq::Entry<K, Gen>>, Gen>,
}

impl<K, Gen> Iterator for IntoIter<K, Gen>
where
    Gen: IGeneration,
{
    type Item = (GenIDOf<Gen>, K);
    fn next(&mut self) -> Option<Self::Item>
    {
        let (id, key) = self.iter.next()?;
        Some((id, key))
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a, K, Gen>
where
    Gen: IGeneration,
{
    iter: crate::gen_seq::Iter<'a, K, Gen>,
}

impl<'a, K, Gen> Iterator for Iter<'a, K, Gen>
where
    Gen: IGeneration,
{
    type Item = (GenIDOf<Gen>, &'a K);
    fn next(&mut self) -> Option<Self::Item>
    {
        let (id, key) = self.iter.next()?;
        Some((id, key))
    }
}

impl<'a, K, Gen> FusedIterator for Iter<'a, K, Gen> where Gen: IGeneration {}
impl<'a, K, Gen> ExactSizeIterator for Iter<'a, K, Gen>
where
    Gen: IGeneration,
{
    fn len(&self) -> usize { self.iter.len() }
}

impl<K, Gen, S> IntoIterator for GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
{
    type Item = (GenIDOf<Gen>, K);
    type IntoIter = IntoIter<K, Gen>;

    fn into_iter(self) -> Self::IntoIter
    {
        IntoIter {
            iter: self.values.into_iter(),
        }
    }
}

impl<'a, K, Gen, S> IntoIterator for &'a GenSetOf<K, Gen, S>
where
    K: Clone,
    Gen: IGeneration,
{
    type Item = (GenIDOf<Gen>, &'a K);
    type IntoIter = Iter<'a, K, Gen>;

    fn into_iter(self) -> Self::IntoIter
    {
        Iter { iter: self.values.iter() }
    }
}
