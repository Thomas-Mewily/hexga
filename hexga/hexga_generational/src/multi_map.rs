use hexga_core::iter::IterExtension;

use super::*;
use crate::gen_vec::*;

pub mod prelude
{
    pub use super::{MultiHashMap,MultiHashMapID,CollectToMultiHashMap};
}


pub type MultiHashMap<K,V> = MultiHashMapOf<K,V,Generation>;
pub type MultiHashMapID = MultiHashMapIDOf<Generation>;


#[derive(Clone)]
pub struct Entry<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    keys: Vec<K>,
    id: MultiHashMapIDOf<Gen>,
    value: V,
    phantom: PhantomData<S>,
}

impl<K, V, Gen, S> Debug for Entry<K,V,Gen,S> where Gen: IGeneration + Debug, K: Debug, V: Debug, S:BuildHasher
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry").field("keys", &self.keys).field("id", &self.id).field("value", &self.value).finish()
    }
}

impl<K, V, Gen, S> PartialEq for Entry<K,V,Gen,S> where Gen: IGeneration, K: PartialEq, V: PartialEq, S: BuildHasher
{
    fn eq(&self, other: &Self) -> bool {
        self.keys == other.keys && self.id == other.id && self.value == other.value
    }
}
impl<K, V, Gen, S> Eq for Entry<K,V,Gen,S> where Gen: IGeneration, K: Eq, V: Eq, S: BuildHasher {}
impl<K, V, Gen, S> Hash for Entry<K,V,Gen,S> where Gen: IGeneration, K: Hash, V: Hash, S: BuildHasher
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.keys.hash(state);
        self.id.hash(state);
        self.value.hash(state);
    }
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
            id: MultiHashMapIDOf::NULL,
            phantom: PhantomData,
        })
    }
}


impl<K,V,Gen,S> Entry<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    pub(crate) const fn new(keys: Vec<K>, value: V) -> Self
    {
        assert!(keys.len() >= 1);
        Self { value, keys, phantom: PhantomData, id: MultiHashMapIDOf::NULL }
    }

    pub fn value(&self) -> &V { &self.value }
    pub fn value_mut(&mut self) -> &mut V { &mut self.value }

    pub fn entry_id(&self) -> EntryID<'_,K,Gen> { EntryID { key: &self.keys, id: self.id } }
    pub fn value_mut_with_entry_id(&mut self) -> (EntryID<'_,K,Gen>, &mut V) { (EntryID{ key: &self.keys, id: self.id }, &mut self.value) }

    /// `[main_key, backward_compatibility_keys...]`
    pub fn keys(&self) -> &[K] { self.keys.as_slice() }
    pub const fn nb_keys(&self) -> usize { self.keys.len() }

    pub const fn id(&self) -> MultiHashMapIDOf<Gen> { self.id }

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

pub type MultiHashMapIDOf<Gen> = GenIDOf<Gen>;

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
pub struct MultiHashMapOf<K,V,Gen=Generation,S=std::hash::RandomState> where Gen: IGeneration, S:BuildHasher
{
    values: GenVecOf<Entry<K,V,Gen,S>,Gen>,
    search: HashMap<K,MultiHashMapIDOf<Gen>,S>,
}

impl<K, V, Gen, S> Eq for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher, Entry<K,V,Gen,S>: Eq {}
impl<K, V, Gen, S> PartialEq for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher, Entry<K,V,Gen,S>: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

#[cfg(feature = "serde")]
impl<K, V, Gen, St> Serialize for MultiHashMapOf<K, V, Gen, St>
where
    K: Serialize,
    V: Serialize,
    Gen: IGeneration + Serialize,
    St: BuildHasher,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.values.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, K, V, Gen, S> Deserialize<'de> for MultiHashMapOf<K, V, Gen, S>
where
    K: Deserialize<'de> + Eq + Hash + Clone,
    V: Deserialize<'de>,
    Gen: IGeneration + Deserialize<'de>,
    S: BuildHasher + Default,
    GenVecOf::<Entry<K, V, Gen, S>, Gen>: Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut values = GenVecOf::<Entry<K, V, Gen, S>, Gen>::deserialize(deserializer)?;
        let mut search = HashMap::<K, GenIDOf<Gen>, S>::with_capacity(values.len());

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

        Ok(MultiHashMapOf { values, search })
    }
}

impl<K,V,Gen,S> Default for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S: BuildHasher + Default
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K,V,Gen> MultiHashMapOf<K,V,Gen,RandomState> where Gen: IGeneration
{
    /// Constructs a new, empty [`MultiHashMap`] with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self { Self { values: GenVecOf::with_capacity(capacity), search: HashMap::with_capacity(capacity) } }
}
impl<K,V,Gen,S> MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    /// Creates a new empty [`MultiHashMap`] with the default hasher.
    pub fn new() -> Self where S: Default
    {
        Self::with_hasher(___())
    }

    /// Creates a new empty [`MultiHashMap`] with a custom hasher.
    pub fn with_hasher(hasher: S) -> Self
    {
        Self { values: GenVecOf::new(), search: HashMap::with_hasher(hasher) }
    }

    /// Creates a new `MultiHashMapOf` with at least the specified capacity and a custom hasher.
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self
    {
        Self { values: GenVecOf::with_capacity(capacity), search: HashMap::with_hasher(hasher) }
    }

    /// Returns a reference to the entry associated with the given ID, or `None` if it does not exist.
    pub fn get_entry(&self, id: MultiHashMapIDOf<Gen>) -> Option<&Entry<K,V,Gen,S>> { self.values.get(id) }
    /// Returns a reference to the value associated with the given ID, or `None` if it does not exist.
    pub fn get(&self, id: MultiHashMapIDOf<Gen>) -> Option<&V> { self.get_entry(id).map(|e| &e.value) }

    /// Returns `true` if the [`MultiHashMap`] contains an entry for the specified ID.
    pub fn contains(&self, id: MultiHashMapIDOf<Gen>) -> bool { self.values.get(id).is_some() }

    /// Returns an iterator over all entries in the [`MultiHashMap`].
    pub fn entries(&self) -> impl Iterator<Item = &Entry<K,V,Gen,S>> { self.values.iter().map(|(_idx,val)| val) }
    /// Returns an iterator over all entry IDs in the [`MultiHashMap`].
    pub fn ids(&self) -> impl Iterator<Item = MultiHashMapIDOf<Gen>> { self.values.ids() }

    /// Returns an iterator over references to all values in the [`MultiHashMap`].
    pub fn values(&self) -> impl Iterator<Item = &V> { self.values.values().map(|e| e.value()) }
    /// Returns an iterator over mutable references to all values in the [`MultiHashMap`].
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> { self.values.values_mut().map(|e| e.value_mut()) }

    /// Consumes the [`MultiHashMap`] and returns an iterator over its entries.
    pub fn into_entries(self) -> impl Iterator<Item = Entry<K,V,Gen,S>> { self.values.into_values() }
    /// Consumes the [`MultiHashMap`] and returns an iterator over its values.
    pub fn into_values(self) -> impl Iterator<Item = V> { self.into_entries().map(|e| e.value) }

    /// Returns the number of entries in the [`MultiHashMap`].
    pub const fn len(&self) -> usize { self.values.len() }


    /// Removes all entries from the [`MultiHashMap`] and resetting all [`GenID`]. After calling this method, previously obtained [`GenID`] **must** not be used, as doing so may lead to undefined behavior.
    pub fn clear(&mut self)
    {
        self.values.clear();
        self.search.clear();
    }

    /// Removes all entries and invalidates all [`GenID`]..
    pub fn remove_all(&mut self)
    {
        self.values.remove_all();
        self.search.clear();
    }

    /// Returns an iterator over all entries.
    pub fn iter(&self) -> Iter<'_,K,V,Gen,S> { self.into_iter() }
    /// Returns a mutable iterator over all entries.
    pub fn iter_mut(&mut self) -> IterMut<'_,K,V,Gen,S> { self.into_iter() }

    /// Returns a mutable reference to the entry corresponding to the given [`GenID`], or `None` if it does not exist.
    pub(crate) fn get_entry_mut(&mut self, id: MultiHashMapIDOf<Gen>) -> Option<&mut Entry<K,V,Gen,S>> { self.values.get_mut(id) }

    /// Returns a mutable reference to the value corresponding to the given [`GenID`], or `None` if it does not exist.
    pub fn get_mut(&mut self, id: MultiHashMapIDOf<Gen>) -> Option<&mut V> { self.get_entry_mut(id).map(|e| &mut e.value) }

    /// Returns a reference to the inner `GenVec` storing the entries.
    pub fn inner_genvec(&self) -> &GenVecOf<Entry<K,V,Gen,S>,Gen>
    {
        &self.values
    }
}

impl<K,V,Gen,S> MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher, K: Eq + Hash
{
    /// Returns the [`GenID`] associated with the given key, or `None` if the key is not present.
    pub fn key_to_id<Q: ?Sized>(&self, key: &Q) -> Option<MultiHashMapIDOf<Gen>> where K: Borrow<Q>, Q: Eq + Hash { self.search.get(key).copied() }

    /// Returns `true` if any of the specified keys exist in the [`MultiHashMap`].
    pub fn contains_any_keys(&self, keys: &[K]) -> bool
    {
        keys.iter().any(|k| self.contains_key(k))
    }
    /// Returns `true` if the [`MultiHashMap`] contains an entry for the given key.
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool where K: Borrow<Q>, Q: Eq + Hash { self.get_entry_from_key(key).is_some() }

    /// Returns a reference to the entry associated with the given key, or `None` if it does not exist.
    pub fn get_entry_from_key<Q: ?Sized>(&self, key: &Q) -> Option<&Entry<K,V,Gen,S>> where K: Borrow<Q>, Q: Eq + Hash
    {
        let idx = self.key_to_id(key)?;
        self.get_entry(idx)
    }
    /// Returns a reference to the value associated with the given key, or `None` if it does not exist.
    pub fn get_from_key<Q: ?Sized>(&self, key: &Q) -> Option<&V> where K: Borrow<Q>, Q: Eq + Hash
    {
        self.get_entry_from_key(key).map(|e| &e.value)
    }

    /// Returns a mutable reference to the entry associated with the given key, or `None` if it does not exist.
    pub(crate) fn get_entry_mut_from_key<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut Entry<K,V,Gen,S>> where K: Borrow<Q>, Q: Eq + Hash
    {
        let idx = self.key_to_id(key)?;
        self.get_entry_mut(idx)
    }

    /// Returns a mutable reference to the value associated with the given key, or `None` if it does not exist.
    pub fn get_mut_from_key<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V> where K: Borrow<Q>, Q: Eq + Hash
    {
        self.get_entry_mut_from_key(key).map(|e| &mut e.value)
    }

    /// Removes the entry associated with the given [`GenID`] and returns it, or `None` if it does not exist.
    pub fn remove_entry(&mut self, id: MultiHashMapIDOf<Gen>) -> Option<Entry<K,V,Gen,S>>
    {
        let v = self.values.remove(id)?;
        for id in v.keys.iter()
        {
            self.search.remove(&id).unwrap();
        }
        Some(v)
    }

    /// Removes the entry associated with the given key and returns it, or `None` if it does not exist.
    pub fn remove_entry_from_key<Q: ?Sized>(&mut self, key: &Q) -> Option<Entry<K,V,Gen,S>> where K: Borrow<Q> , Q: Eq + Hash
    {
        let idx = self.key_to_id(key)?;
        self.remove_entry(idx)
    }

    /// Removes the value associated with the given [`GenID`] and returns it, or `None` if it does not exist.
    pub fn remove(&mut self, id: MultiHashMapIDOf<Gen>) -> Option<V>
    {
        self.remove_entry(id).map(|e| e.value)
    }

    /// Removes the value associated with the given key and returns it, or `None` if it does not exist.
    pub fn remove_from_key<Q: ?Sized>(&mut self, key: &Q) -> Option<V> where K: Borrow<Q> , Q: Eq + Hash
    {
        let idx = self.key_to_id(key)?;
        self.remove(idx)
    }

    /// Removes the key from the entry, and remove the entry if it has no remaining keys, or `None` otherwise.
    pub fn remove_entry_key<Q: ?Sized>(&mut self, key: &Q) -> Option<Entry<K,V,Gen,S>> where K: Borrow<Q> , Q: Eq + Hash
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

    /// Removes the value
    pub fn remove_key<Q: ?Sized>(&mut self, key: &Q) -> Option<V> where K: Borrow<Q> , Q: Eq + Hash
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
    pub fn retain_mut<F>(&mut self, mut f: F) where F: FnMut(EntryID<'_,K,Gen>, &mut V) -> bool
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
impl<K,V,Gen,S> MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher, K: Eq + Hash + Clone
{
    /// Inserts a value with the specified key. Returns the entry [`GenID`] if the insertion succeeds.
    pub fn insert(&mut self, key: K, value: V) -> Option<MultiHashMapIDOf<Gen>>
    {
        self.insert_with_keys(vec![key], value)
    }

    /// Inserts a value with the specified keys.
    /// Returns the entry [`GenID`] if the insertion succeeds.
    ///
    /// Duplicate keys within the input Keys iterator are ignored.
    /// Returns `None` if any key already exists in the [`MultiHashMap`] or if the list of keys is empty.
    pub fn insert_with_keys<Keys>(&mut self, main_key_follow_by_backward_keys: Keys, value: V) -> Option<MultiHashMapIDOf<Gen>> where Keys: IntoIterator<Item = K>
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

        entry.keys.retain(|key| {
            let old = self.search.insert(key.clone(), id);
            old.is_none() // Remove duplicate key inside the input main_key_follow_by_backward_keys
        });
        Some(id)
    }

    /// Adds multiple keys to an existing entry.
    ///
    /// Returns an error if any key already exists for a different entry.
    /// Duplicate keys within the input Keys iterator are ignored.
    pub fn add_keys(&mut self, source_id: MultiHashMapIDOf<Gen>, keys: Vec<K>) -> Result<(), Vec<K>>
    {
        if !source_id.is_valid(self) { return Err(keys); };

        for key in keys.iter()
        {
            let target_id = self.key_to_id(key);
            if target_id.is_some() && target_id != Some(source_id) { return Err(keys); }
        }

        for key in keys.into_iter()
        {
            let old = self.search.insert(key.clone(), source_id);
            if old.is_none()
            {
                let entry = self.get_entry_mut(source_id).unwrap();
                entry.keys.push(key);
            }else
            {
                debug_assert!(self.get_entry(source_id).unwrap().keys().contains(&key))
            }
        }
        Ok(())
    }

    /// Adds multiple keys to an entry identified by a key. Returns an error if any key already exists.
    pub fn add_keys_from_key<Q: ?Sized>(&mut self, source_key: &Q, keys: Vec<K>) -> Result<(), Vec<K>> where K: Borrow<Q> , Q: Eq + Hash
    {
        match self.key_to_id(source_key)
        {
            Some(id) => self.add_keys(id, keys),
            None => Err(keys),
        }
    }

    /// Adds a single key to an existing entry. Returns an error if the key already exists.
    pub fn add_key(&mut self, source_id: MultiHashMapIDOf<Gen>, key: K) -> Result<(), K>
    {
        if self.contains_key(&key) { return Err(key); }

        let Some(e) = self.get_entry_mut(source_id) else { return Err(key); };
        e.keys.push(key.clone());
        let old = self.search.insert(key, source_id);
        assert!(old.is_none());

        Ok(())
    }

    /// Adds a single key to an entry identified by a key. Returns an error if the key already exists.
    pub fn add_key_from_key<Q: ?Sized>(&mut self, source_key: &Q, key: K) -> Result<(), K> where K: Borrow<Q> , Q: Eq + Hash
    {
        match self.key_to_id(source_key)
        {
            Some(id) => self.add_key(id, key),
            None => Err(key),
        }
    }
}

impl<K,V,Gen,S> Index<MultiHashMapIDOf<Gen>> for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Output=V;
    fn index(&self, id: MultiHashMapIDOf<Gen>) -> &Self::Output {
        self.get(id).unwrap()
    }
}
impl<K,V,Gen,S> IndexMut<MultiHashMapIDOf<Gen>> for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    fn index_mut(&mut self, id: MultiHashMapIDOf<Gen>) -> &mut Self::Output {
        self.get_mut(id).unwrap()
    }
}
impl<K,V,Gen,S> Collection for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher {}
impl<K,V,Gen,S> Get<MultiHashMapIDOf<Gen>> for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Output = V;
    fn get(&self, index: MultiHashMapIDOf<Gen>) -> Option<&Self::Output> {
        self.get(index)
    }
}
// harcoded for string to avoid conflicting implementations of trait
impl<V,Gen,S,Q> Get<&Q> for MultiHashMapOf<String,V,Gen,S> where Gen: IGeneration, S:BuildHasher, String:Borrow<Q>, Q: Eq + Hash
{
    type Output = V;
    fn get(&self, key: &Q) -> Option<&Self::Output> {
        self.get_from_key(key)
    }
}
impl<K,V,Gen,S> GetMut<MultiHashMapIDOf<Gen>> for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    fn get_mut(&mut self, index: MultiHashMapIDOf<Gen>) -> Option<&mut Self::Output> {
        self.get_mut(index)
    }
}
// harcoded for string to avoid conflicting implementations of trait
impl<V,Gen,S,Q> GetMut<&Q> for MultiHashMapOf<String,V,Gen,S> where Gen: IGeneration, S:BuildHasher, String:Borrow<Q>, Q: Eq + Hash
{
    fn get_mut(&mut self, key: &Q) -> Option<&mut Self::Output> {
        self.get_mut_from_key(key)
    }
}
impl<K,V,Gen,S> GetManyMut<MultiHashMapIDOf<Gen>> for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    fn get_many_mut<const N: usize>(&mut self, indices: [MultiHashMapIDOf<Gen>; N]) -> Option<[&mut Self::Output;N]>
    {
        self.values.get_many_mut(indices).map(|entries| entries.map(|e| &mut e.value))
    }
    fn try_get_many_mut<const N: usize>(&mut self, indices: [MultiHashMapIDOf<Gen>; N]) -> Result<[&mut Self::Output;N], ManyMutError>
    {
        self.values.try_get_many_mut(indices).map(|entries| entries.map(|e| &mut e.value))
    }
}
// harcoded for string to avoid conflicting implementations of trait
impl<V,Gen,S,Q> GetManyMut<&Q> for MultiHashMapOf<String,V,Gen,S> where Gen: IGeneration, S:BuildHasher, String:Borrow<Q>, Q: Eq + Hash
{
    fn get_many_mut<const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut Self::Output;N]>
    {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        let keys = keys.map(|key| self.key_to_id(key));
        if keys.any(|k| k.is_none()) { return None; }
        let indices = keys.map(|k| k.unwrap());
        self.get_many_mut(indices)
    }
    fn try_get_many_mut<const N: usize>(&mut self, keys: [&Q; N]) -> Result<[&mut Self::Output;N], ManyMutError>
    {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        let keys = keys.map(|key| self.key_to_id(key));
        if keys.any(|k| k.is_none()) { return Err(ManyMutError::IndexOutOfBounds); }
        let indices = keys.map(|k| k.unwrap());
        self.try_get_many_mut(indices)
    }
}
impl<K,V,Gen,S> Remove<MultiHashMapIDOf<Gen>> for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher, K: Eq + Hash
{
    type Output=V;
    fn remove(&mut self, id: MultiHashMapIDOf<Gen>) -> Option<Self::Output> {
        self.remove(id)
    }
}
// harcoded for string to avoid conflicting implementations of trait
impl<V,Gen,S,Q> Remove<&Q> for MultiHashMapOf<String,V,Gen,S> where Gen: IGeneration, S:BuildHasher, String:Borrow<Q>, Q: Eq + Hash
{
    type Output=V;
    fn remove(&mut self, key: &Q) -> Option<Self::Output> {
        self.remove_from_key(key)
    }
}


impl<K,V,Gen,S> Length for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher { #[inline(always)] fn len(&self) -> usize { self.len() } }
impl<K,V,Gen,S> Clear for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher { #[inline(always)] fn clear(&mut self) { self.clear() } }

impl<K,V,Gen,S> Capacity for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Param=S;

    #[inline(always)]
    fn capacity(&self) -> usize { self.values.capacity() }

    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, hasher: Self::Param) -> Self { Self::with_capacity_and_hasher(capacity, hasher) }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.values.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.values.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.values.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.values.try_reserve_exact(additional) }
}

impl<Keys,K,V,Gen,S> FromIterator<(Keys,V)> for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher + Default, Keys: IntoIterator<Item = K>, K: Eq + Hash + Clone
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

impl<K,V,Gen,S> IntoIterator for MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Item=(Vec<K>, V);
    type IntoIter=IntoIter<K,V,Gen,S>;
    fn into_iter(self) -> Self::IntoIter { let len_remaining = self.values.len(); IntoIter{ iter: self.values.values.into_iter(), len_remaining } }
}

#[derive(Clone, Debug)]
pub struct IntoIter<K,V,Gen=Generation,S=std::hash::RandomState> where Gen: IGeneration, S:BuildHasher
{
    iter: std::vec::IntoIter<crate::gen_vec::Entry<Entry<K,V,Gen,S>, Gen>>,
    len_remaining: usize,
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

pub struct EntryID<'a,K,Gen> where Gen: IGeneration
{
    pub key: &'a [K],
    pub id : MultiHashMapIDOf<Gen>,
}



impl<'a,K,V,Gen,S> IntoIterator for &'a MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Item=(EntryID<'a,K,Gen>, &'a V);
    type IntoIter=Iter<'a,K,V,Gen,S>;
    fn into_iter(self) -> Self::IntoIter { Iter { iter: self.values.iter() } }
}

#[derive(Debug)]
pub struct Iter<'a,K,V,Gen=Generation,S=std::hash::RandomState> where Gen: IGeneration, S:BuildHasher
{
    iter: gen_vec::Iter<'a,Entry<K,V,Gen,S>,Gen>,
}
impl<'a,K,V,Gen,S> Clone for Iter<'a,K, V, Gen, S> where Gen: IGeneration + Clone, S:BuildHasher + Clone
{
    fn clone(&self) -> Self {
        Self { iter: self.iter.clone() }
    }
}

impl<'a,K,V,Gen,S> Iterator for Iter<'a,K, V, Gen, S> where Gen: IGeneration, S:BuildHasher
{
    type Item = (EntryID<'a,K,Gen>, &'a V);
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


impl<'a,K,V,Gen,S> IntoIterator for &'a mut MultiHashMapOf<K,V,Gen,S> where Gen: IGeneration, S:BuildHasher
{
    type Item=(EntryID<'a,K,Gen>, &'a mut V);
    type IntoIter=IterMut<'a,K,V,Gen,S>;
    fn into_iter(self) -> Self::IntoIter { IterMut { iter: self.values.iter_mut() } }
}

#[derive(Debug)]
pub struct IterMut<'a,K,V,Gen=Generation,S=std::hash::RandomState> where Gen: IGeneration, S:BuildHasher
{
    iter: gen_vec::IterMut<'a,Entry<K,V,Gen,S>,Gen>,
}
impl<'a,K,V,Gen,S> Iterator for IterMut<'a,K, V, Gen, S> where Gen: IGeneration, S:BuildHasher
{
    type Item = (EntryID<'a,K,Gen>, &'a mut V);
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


pub trait CollectToMultiHashMap<Keys,K,V>: Sized + IntoIterator<Item = (Keys,V)> where Keys: IntoIterator<Item = K>, K: Eq + Hash + Clone
{
    fn to_multihashmap(self) -> MultiHashMapOf<K,V>
    {
        MultiHashMapOf::from_iter(self)
    }
}
impl<Keys,K,V,I> CollectToMultiHashMap<Keys,K,V> for I where I: IntoIterator<Item = (Keys,V)>, Keys: IntoIterator<Item = K>, K: Eq + Hash + Clone {}