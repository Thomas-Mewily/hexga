use super::*;


pub trait Insert<K,V=()> : Collection
{
    /// Insert the (key, value), and return the old value
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    // -> Option<V> + Self::Extra; // ex; passing an GenID ??
}
/*
pub trait TryInsert<K,V=()> : Insert<K,V>
{
    type Error;
    fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, Self::Error>;
}*/
#[cfg(feature = "std")]
impl<K,V,S> Insert<K,V> for HashMap<K,V,S>
    where
    K: Eq + Hash,
    S: BuildHasher
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }
}
#[cfg(feature = "std")]
impl<K,S> Insert<K,()> for HashSet<K,S>
    where
    K: Eq + Hash,
    S: BuildHasher
{
    fn insert(&mut self, key: K, _value: ()) -> Option<()> {
        (!self.insert(key)).then_some(())
    }
}

impl<K,V> Insert<K,V> for BTreeMap<K,V>
    where
    K: Ord,
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }
}
impl<K> Insert<K,()> for BTreeSet<K>
    where
    K: Ord,
{
    fn insert(&mut self, key: K, _value: ()) -> Option<()> {
        (!self.insert(key)).then_some(())
    }
}
