use super::*;
use crate::gen_vec::*;

pub mod prelude
{
    pub use super::{MultiMap,MultiMapID,CollectToMultiMap};
}


pub type MultiMap<K,V> = MultiMapOf<K,V,Generation>;
pub type MultiMapID<K,V> = MultiMapIDOf<K,V,Generation,RandomState>;


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entry<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    keys: Vec<K>,
    id: MultiMapIDOf<K,V,Gen,S>,
    value: V,
    phantom: PhantomData<S>,
}

#[cfg(feature = "hexga_io")]
impl<K, V, Gen, S> IoSave for Entry<K, V, Gen,S>
    where
        K: Serialize,
        V: Serialize,
        Gen: IGeneration,
        S: BuildHasher
{

}

#[cfg(feature = "hexga_io")]
impl<K, V, Gen, S> IoLoad for Entry<K, V, Gen, S>
    where
        K: for<'de> Deserialize<'de> + Hash + Eq,
        V: for<'de> Deserialize<'de>,
        Gen: IGeneration + for<'de> Deserialize<'de>,
        S: BuildHasher
{

}


#[cfg(feature = "serde")]
impl<K, V, Gen, St> Serialize for Entry<K, V, Gen, St>
where
    K: Serialize,
    V: Serialize,
    Gen: IGeneration,
    St: BuildHasher
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeTuple;

        let mut state = serializer.serialize_tuple(2)?;
        state.serialize_element(&self.keys)?;
        state.serialize_element(&self.value)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, K, V, Gen, S> Deserialize<'de> for Entry<K, V, Gen, S>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
    Gen: IGeneration,
    S: BuildHasher
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (keys,value) = <(Vec<K>, V)>::deserialize(deserializer)?;

        let mut seen = std::collections::HashSet::new();
        if keys.iter().any(|k| !seen.insert(k)) {
            return Err(serde::de::Error::custom("duplicate keys found in Entry"));
        }

        Ok(crate::multi_map::Entry {
            keys: keys,
            value: value,
            id: MultiMapIDOf::NULL,
            phantom: PhantomData,
        })
    }
}


impl<K,V,Gen,S> Entry<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    pub(crate) const fn new(keys : Vec<K>, value : V) -> Self
    {
        assert!(keys.len() >= 1);
        Self { value, keys, phantom: PhantomData, id: MultiMapIDOf::NULL }
    }

    pub fn value(&self) -> &V { &self.value }
    pub fn value_mut(&mut self) -> &mut V { &mut self.value }

    pub fn entry_id(&self) -> EntryID<'_,K,V,Gen,S> { EntryID { key: &self.keys, id: self.id } }
    pub fn value_mut_with_entry_id(&mut self) -> (EntryID<'_,K,V,Gen,S>, &mut V) { (EntryID{ key: &self.keys, id: self.id }, &mut self.value) }

    /// `[main_key, backward_compatibility_keys...]`
    pub fn keys(&self) -> &[K] { self.keys.as_slice() }
    pub const fn nb_keys(&self) -> usize { self.keys.len() }

    pub const fn id(&self) -> MultiMapIDOf<K,V,Gen,S> { self.id }

    pub fn main_key(&self) -> &K { self.keys.first().unwrap() }

    pub const fn have_backward_compatibility_keys(&self) -> bool { self.keys.len() > 1 }
    pub fn backward_compatibility_keys(&self) -> &[K]
    {
        if self.have_backward_compatibility_keys()
        {
            &self.keys.as_slice()[1..]
        }else
        {
            &self.keys.as_slice()[0..0]
        }
    }

    pub fn into_value(self) -> V { self.value }
    pub fn into_keys_and_value(self) -> (Vec<K>, V)
    {
        (self.keys, self.value)
    }
}
impl<K,V,Gen,S> Into<(Vec<K>, V)> for Entry<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    fn into(self) -> (Vec<K>, V)
    {
        self.into_keys_and_value()
    }
}

pub type MultiMapIDOf<K,V,Gen,S=RandomState> = GenIDOf<Entry<K,V,Gen,S>,Gen>;

/// A data structure similar to [`HashMap`], for managing items using persistant keys
///
/// Can be indexed using the Key, or by using an [`TableIDOf<K,V,Gen>`] (faster).
///
/// Each Entry:
///
/// - have value `V`,
/// - can have N number of keys (one (the main key), or multiple (the main keys then backward compatibility keys)),
/// - have an [`TableIDOf<K,V,Gen>`] for fast access.
#[derive(Clone, Debug)]
pub struct MultiMapOf<K,V,Gen=Generation,S=std::hash::RandomState> where Gen: IGeneration, S:BuildHasher
{
    values: GenVecOf<Entry<K,V,Gen,S>,Gen>,
    search: HashMap<K,MultiMapIDOf<K,V,Gen,S>,S>,
}

impl<K, V, Gen, S> Eq for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher, Entry<K,V,Gen,S>: Eq {}
impl<K, V, Gen, S> PartialEq for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher, Entry<K,V,Gen,S>: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

#[cfg(feature = "serde")]
impl<K, V, Gen, St> Serialize for MultiMapOf<K, V, Gen, St>
where
    K: Serialize,
    V: Serialize,
    Gen: IGeneration + Serialize,
    St :BuildHasher,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.values.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, K, V, Gen, S> Deserialize<'de> for MultiMapOf<K, V, Gen, S>
where
    K: Deserialize<'de> + Eq + Hash + Clone,
    V: Deserialize<'de>,
    Gen: IGeneration + Deserialize<'de>,
    S: BuildHasher,
    GenVecOf::<Entry<K, V, Gen, S>, Gen> : Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut values = GenVecOf::<Entry<K, V, Gen, S>, Gen>::deserialize(deserializer)?;
        let mut search = HashMap::<K, GenIDOf<Entry<K, V, Gen, S>, Gen>, S>::with_capacity(values.len());

        for (id, entry) in values.iter_mut()
        {
            entry.id = id;
            for key in entry.keys()
            {
                if search.insert(key.clone(), id).is_some() {
                    return Err(serde::de::Error::custom("duplicate key found during TableOf deserialization"));
                }
            }
        }

        Ok(MultiMapOf { values, search })
    }
}

impl<K,V,Gen,S> Default for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S: BuildHasher + Default
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K,V,Gen> MultiMapOf<K,V,Gen,RandomState> where Gen: IGeneration
{
    pub fn with_capacity(capacity : usize) -> Self { Self { values: GenVecOf::with_capacity(capacity), search: HashMap::with_capacity(capacity) } }
}
impl<K,V,Gen,S> MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    pub fn new() -> Self where S: Default
    {
        Self::with_hasher(___())
    }

    pub fn with_hasher(hash_builder: S) -> Self
    {
        Self { values: GenVecOf::new(), search: HashMap::with_hasher(hash_builder) }
    }

    pub fn get_entry(&self, id: MultiMapIDOf<K,V,Gen,S>) -> Option<&Entry<K,V,Gen,S>> { self.values.get(id) }
    pub fn get(&self, id: MultiMapIDOf<K,V,Gen,S>) -> Option<&V> { self.get_entry(id).map(|e| &e.value) }

    pub fn contains(&self, id: MultiMapIDOf<K,V,Gen,S>) -> bool { self.values.get(id).is_some() }

    pub fn entries(&self) -> impl Iterator<Item = &Entry<K,V,Gen,S>> { self.values.iter().map(|(_idx,val)| val) }
    pub fn ids(&self) -> impl Iterator<Item = MultiMapIDOf<K,V,Gen,S>> { self.values.ids() }

    pub fn values(&self) -> impl Iterator<Item = &V> { self.values.values().map(|e| e.value()) }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> { self.values.values_mut().map(|e| e.value_mut()) }

    pub fn into_entries(self) -> impl Iterator<Item = Entry<K,V,Gen,S>> { self.values.into_values() }
    pub fn into_values(self) -> impl Iterator<Item = V> { self.into_entries().map(|e| e.value) }

    /// Number of entries
    pub const fn len(&self) -> usize { self.values.len() }

    /// Clears the [`Table`], removing all elements and resetting all [`GenID`] values.
    ///
    /// After calling this method, any previous [`GenID`] is no longer valid (not enforced) and
    /// **must** not be used, as doing so may lead to undefined behavior.
    pub fn clear(&mut self)
    {
        self.values.clear();
        self.search.clear();
    }


    /// Removes all elements from the [`Table`] and invalidates all existing [`GenID`] (enforced).
    pub fn remove_all(&mut self)
    {
        self.values.remove_all();
        self.search.clear();
    }

    pub fn iter(&self) -> Iter<'_,K,V,Gen,S> { self.into_iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_,K,V,Gen,S> { self.into_iter() }

    pub(crate) fn get_entry_mut(&mut self, id: MultiMapIDOf<K,V,Gen,S>) -> Option<&mut Entry<K,V,Gen,S>> { self.values.get_mut(id) }
    pub fn get_mut(&mut self, id: MultiMapIDOf<K,V,Gen,S>) -> Option<&mut V> { self.get_entry_mut(id).map(|e| &mut e.value) }

    pub fn values_genvec(&self) -> &GenVecOf<Entry<K,V,Gen,S>,Gen>
    {
        &self.values
    }
}

impl<K,V,Gen,S> MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher, K : Eq + Hash
{
    pub fn key_to_id<Q : ?Sized>(&self, key : &Q) -> Option<MultiMapIDOf<K,V,Gen,S>> where K : Borrow<Q>, Q : Eq + Hash { self.search.get(key).copied() }

    pub fn contains_any_keys(&self, keys : &[K]) -> bool
    {
        keys.iter().any(|k| self.contains_key(k))
    }
    pub fn contains_key<Q : ?Sized>(&self, key : &Q) -> bool where K : Borrow<Q>, Q : Eq + Hash { self.get_entry_from_key(key).is_some() }

    pub fn get_entry_from_key<Q : ?Sized>(&self, key : &Q) -> Option<&Entry<K,V,Gen,S>> where K : Borrow<Q>, Q : Eq + Hash
    {
        let idx = self.key_to_id(key)?;
        self.get_entry(idx)
    }
    pub fn get_from_key<Q : ?Sized>(&self, key: &Q) -> Option<&V> where K : Borrow<Q>, Q : Eq + Hash
    {
        self.get_entry_from_key(key).map(|e| &e.value)
    }

    pub(crate) fn get_entry_mut_from_key<Q : ?Sized>(&mut self, key : &Q) -> Option<&mut Entry<K,V,Gen,S>> where K : Borrow<Q>, Q : Eq + Hash
    {
        let idx = self.key_to_id(key)?;
        self.get_entry_mut(idx)
    }
    pub fn get_mut_from_key<Q : ?Sized>(&mut self, key: &Q) -> Option<&mut V> where K : Borrow<Q>, Q : Eq + Hash
    {
        self.get_entry_mut_from_key(key).map(|e| &mut e.value)
    }

    pub fn remove_entry(&mut self, id: MultiMapIDOf<K,V,Gen,S>) -> Option<Entry<K,V,Gen,S>>
    {
        let v = self.values.remove(id)?;
        for id in v.keys.iter()
        {
            self.search.remove(&id).unwrap();
        }
        Some(v)
    }
    pub fn remove_entry_from_key<Q : ?Sized>(&mut self, key : &Q) -> Option<Entry<K,V,Gen,S>> where K : Borrow<Q> , Q : Eq + Hash
    {
        let idx = self.key_to_id(key)?;
        self.remove_entry(idx)
    }


    pub fn remove(&mut self, id: MultiMapIDOf<K,V,Gen,S>) -> Option<V>
    {
        self.remove_entry(id).map(|e| e.value)
    }
    pub fn remove_from_key<Q : ?Sized>(&mut self, key: &Q) -> Option<V> where K : Borrow<Q> , Q : Eq + Hash
    {
        let idx = self.key_to_id(key)?;
        self.remove(idx)
    }

    /// Return the entry if it have no keys remaning
    pub fn remove_entry_key<Q : ?Sized>(&mut self, key: &Q) -> Option<Entry<K,V,Gen,S>> where K : Borrow<Q> , Q : Eq + Hash
    {
        let id = self.key_to_id(key)?;
        let entry = self.get_entry_mut(id)?;
        if entry.keys.len() == 1
        {
            self.search.remove(key);
            let v = self.values.remove(id);
            debug_assert!(v.as_ref().unwrap().main_key().borrow() == key);
            v
        }else
        {
            // Can probably be more opti by removing only the first ID since it is present only once,

            // Doing a swap_remove is not ok since the order the the keys matter:
            // If you remove the main key, the next backward compatibility key become the main key
            entry.keys.retain(|e| (*e).borrow() == key);
            self.search.remove(key);
            None
        }
    }

    /// Return the value if it have no keys remaning
    pub fn remove_key<Q : ?Sized>(&mut self, key: &Q) -> Option<V> where K : Borrow<Q> , Q : Eq + Hash
    {
        self.remove_entry_key(key).map(|e| e.value)
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all [`Entry`] for which `f(&Entry)` returns `false`. The elements are visited in unsorted (and unspecified) order.
    pub fn retain<F>(&mut self, mut f: F) where F: FnMut(&Entry<K,V,Gen,S>) -> bool
    {
        let Self { values, search } = self;
        values.retain_mut(|_id, entry|
            {
                debug_assert_eq!(_id, entry.id());
                if !f(entry)
                {
                    for k in entry.keys()
                    {
                        search.remove(k);
                    }
                    true
                }else
                {
                    false
                }
            });
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all [`Entry`] for which `f(EntryID, &mut v)` returns `false`. The elements are visited in unsorted (and unspecified) order.
    pub fn retain_mut<F>(&mut self, mut f: F) where F: FnMut(EntryID<'_,K,V,Gen,S>, &mut V) -> bool
    {
        let Self { values, search } = self;
        values.retain_mut(|_id,entry|
            {
                let (id, val) = entry.value_mut_with_entry_id();
                debug_assert_eq!(_id, id.id);
                if !f(id, val)
                {
                    for k in entry.keys()
                    {
                        search.remove(k);
                    }
                    true
                }else
                {
                    false
                }
            });
    }
}
impl<K,V,Gen,S> MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher, K: Eq + Hash + Clone
{
    pub fn insert(&mut self, key: K, value: V) -> Option<MultiMapIDOf<K,V,Gen,S>>
    {
        self.insert_with_keys(vec![key], value)
    }
    /// Return [None] if main_key_follow_by_backward_keys is empty
    pub fn insert_with_keys<Keys>(&mut self, main_key_follow_by_backward_keys : Keys, value: V) -> Option<MultiMapIDOf<K,V,Gen,S>> where Keys: IntoIterator<Item = K>
    {
        let keys = main_key_follow_by_backward_keys.to_vec();
        if keys.is_empty() { return None; } // Missing main keys
        for key in keys.iter()
        {
            if self.contains_key(key) { return None; }
        }

        let id= self.values.insert(Entry::new(keys, value));
        let entry = &mut self.values[id];
        entry.id = id;
        for key in entry.keys()
        {
            let old_key = self.search.insert(key.clone(), id);
            assert!(old_key.is_none());
        }
        Some(id)
    }

    /// If any key is a duplicate / already used, return an error
    pub fn add_keys(&mut self, source_id: MultiMapIDOf<K,V,Gen,S>, keys: Vec<K>) -> Result<(), Vec<K>>
    {
        for key in keys.iter()
        {
            if self.contains_key(key) { return Err(keys); }
        }

        let Some(e) = self.get_entry_mut(source_id) else { return Err(keys); };

        e.keys.extend_from_slice(&keys);

        for key in keys.into_iter()
        {
            let old = self.search.insert(key, source_id);
            assert!(old.is_none());
        }
        Ok(())
    }
    /// If any key is a duplicate / already used, return an error
    pub fn add_keys_from_key<Q : ?Sized>(&mut self, source_key: &Q, keys: Vec<K>) -> Result<(), Vec<K>> where K : Borrow<Q> , Q : Eq + Hash
    {
        match self.key_to_id(source_key)
        {
            Some(id) => self.add_keys(id, keys),
            None => Err(keys),
        }
    }

    pub fn add_key(&mut self, source_id: MultiMapIDOf<K,V,Gen,S>, key: K) -> Result<(), K>
    {
        if self.contains_key(&key) { return Err(key); }

        let Some(e) = self.get_entry_mut(source_id) else { return Err(key); };
        e.keys.push(key.clone());
        let old = self.search.insert(key, source_id);
        assert!(old.is_none());

        Ok(())
    }
    pub fn add_key_from_key<Q : ?Sized>(&mut self, source_key: &Q, key: K) -> Result<(), K> where K : Borrow<Q> , Q : Eq + Hash
    {
        match self.key_to_id(source_key)
        {
            Some(id) => self.add_key(id, key),
            None => Err(key),
        }
    }
}

impl<K,V,Gen,S> Index<MultiMapIDOf<K,V,Gen,S>> for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Output=V;
    fn index(&self, id: MultiMapIDOf<K,V,Gen,S>) -> &Self::Output {
        self.get(id).unwrap()
    }
}
impl<K,V,Gen,S> IndexMut<MultiMapIDOf<K,V,Gen,S>> for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    fn index_mut(&mut self, id: MultiMapIDOf<K,V,Gen,S>) -> &mut Self::Output {
        self.get_mut(id).unwrap()
    }
}
impl<K,V,Gen,S> Get<MultiMapIDOf<K,V,Gen,S>> for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Output = V;
    fn get(&self, index : MultiMapIDOf<K,V,Gen,S>) -> Option<&Self::Output> {
        self.get(index)
    }
}
impl<K,V,Gen,S> GetMut<MultiMapIDOf<K,V,Gen,S>> for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    fn get_mut(&mut self, index: MultiMapIDOf<K,V,Gen,S>) -> Option<&mut Self::Output> {
        self.get_mut(index)
    }
}
impl<K,V,Gen,S> GetManyMut<MultiMapIDOf<K,V,Gen,S>> for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    fn get_many_mut<const N: usize>(&mut self, indices: [MultiMapIDOf<K,V,Gen,S>; N]) -> Option<[&mut Self::Output;N]>
    {
        self.values.get_many_mut(indices).map(|entries| entries.map(|e| &mut e.value))
    }

    fn try_get_many_mut<const N: usize>(&mut self, indices: [MultiMapIDOf<K,V,Gen,S>; N]) -> Result<[&mut Self::Output;N], ManyMutError>
    {
        self.values.try_get_many_mut(indices).map(|entries| entries.map(|e| &mut e.value))
    }
}

impl<K,V,Gen,S> Remove<MultiMapIDOf<K,V,Gen,S>> for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher, K: Eq + Hash
{
    type Output=V;
    fn remove(&mut self, id: MultiMapIDOf<K,V,Gen,S>) -> Option<Self::Output> {
        self.remove(id)
    }
}


impl<K,V,Gen,S> Length for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher { #[inline(always)] fn len(&self) -> usize { self.len() } }
impl<K,V,Gen,S> Clearable for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher { #[inline(always)] fn clear(&mut self) { self.clear() } }

impl<K,V,Gen,S> Capacity for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Param=S;

    #[inline(always)]
    fn capacity(&self) -> usize { self.values.capacity() }

    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, hasher : Self::Param) -> Self { let mut s = Self::with_hasher(hasher); s.values.reserve(capacity); s }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.values.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.values.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.values.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.values.try_reserve_exact(additional) }
}

impl<Keys,K,V,Gen,S> FromIterator<(Keys,V)> for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher + Default, Keys: IntoIterator<Item = K>, K: Eq + Hash + Clone
{
    fn from_iter<I: IntoIterator<Item = (Keys,V)>>(iter: I) -> Self
    {
        let it = iter.into_iter();
        let (min_size,max_size) = it.size_hint();
        let mut s = Self::with_capacity(max_size.unwrap_or(min_size));
        for (keys, v) in it
        {
            let _ = s.insert_with_keys(keys, v);
        }
        s
    }
}

impl<K,V,Gen,S> IntoIterator for MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Item=(Vec<K>, V);
    type IntoIter=IntoIter<K,V,Gen,S>;
    fn into_iter(self) -> Self::IntoIter { let len_remaining = self.values.len(); IntoIter{ iter: self.values.values.into_iter(), len_remaining } }
}

#[derive(Clone, Debug)]
pub struct IntoIter<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    iter: std::vec::IntoIter<crate::gen_vec::Entry<Entry<K,V,Gen,S>, Gen>>,
    len_remaining : usize,
}
impl<K,V,Gen,S> Iterator for IntoIter<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Item = (Vec<K>, V);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(slot) = self.iter.next()
        {
            if let EntryValue::Occupied(value) = slot.value
            {
                self.len_remaining -= 1;
                return Some(value.into_keys_and_value());
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<K,V,Gen,S> FusedIterator for IntoIter<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher {}
impl<K,V,Gen,S> ExactSizeIterator for IntoIter<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher { fn len(&self) -> usize { self.len_remaining } }

pub struct EntryID<'a,K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    pub key : &'a [K],
    pub id  : MultiMapIDOf<K,V,Gen,S>,
}



impl<'a,K,V,Gen,S> IntoIterator for &'a MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Item=(EntryID<'a,K,V,Gen,S>, &'a V);
    type IntoIter=Iter<'a,K,V,Gen,S>;
    fn into_iter(self) -> Self::IntoIter { Iter { iter: self.values.iter() } }
}

#[derive(Clone, Debug)]
pub struct Iter<'a,K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    iter: gen_vec::Iter<'a,Entry<K,V,Gen,S>,Gen>,
}
impl<'a,K,V,Gen,S> Iterator for Iter<'a,K, V, Gen, S> where Gen: IGeneration, S:BuildHasher
{
    type Item = (EntryID<'a,K,V,Gen,S>, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((id, entry)) = self.iter.next()
        {
            debug_assert_eq!(id, entry.id());
            return Some((entry.entry_id(), &entry.value));
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<'a,K,V,Gen,S> FusedIterator for Iter<'a,K,V,Gen,S> where Gen: IGeneration, S:BuildHasher {}
impl<'a,K,V,Gen,S> ExactSizeIterator for Iter<'a,K,V,Gen,S> where Gen: IGeneration, S:BuildHasher { fn len(&self) -> usize { self.iter.len() } }


impl<'a,K,V,Gen,S> IntoIterator for &'a mut MultiMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Item=(EntryID<'a,K,V,Gen,S>, &'a mut V);
    type IntoIter=IterMut<'a,K,V,Gen,S>;
    fn into_iter(self) -> Self::IntoIter { IterMut { iter: self.values.iter_mut() } }
}

#[derive(Debug)]
pub struct IterMut<'a,K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    iter: gen_vec::IterMut<'a,Entry<K,V,Gen,S>,Gen>,
}
impl<'a,K,V,Gen,S> Iterator for IterMut<'a,K, V, Gen, S> where Gen: IGeneration, S:BuildHasher
{
    type Item = (EntryID<'a,K,V,Gen,S>, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((id, entry)) = self.iter.next()
        {
            debug_assert_eq!(id, entry.id());
            return Some(entry.value_mut_with_entry_id());
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<'a,K,V,Gen,S> FusedIterator for IterMut<'a,K,V,Gen,S> where Gen: IGeneration, S:BuildHasher {}
impl<'a,K,V,Gen,S> ExactSizeIterator for IterMut<'a,K,V,Gen,S> where Gen: IGeneration, S:BuildHasher { fn len(&self) -> usize { self.iter.len() } }


pub trait CollectToMultiMap<Keys,K,V> : Sized + IntoIterator<Item = (Keys,V)> where Keys: IntoIterator<Item = K>, K: Eq + Hash + Clone
{
    fn to_multimap(self) -> MultiMapOf<K,V>
    {
        MultiMapOf::from_iter(self)
    }
}
impl<Keys,K,V,I> CollectToMultiMap<Keys,K,V> for I where I : IntoIterator<Item = (Keys,V)>, Keys: IntoIterator<Item = K>, K: Eq + Hash + Clone {}