use super::*;

pub struct StableMapID<T>
{
    idx : usize,
    phantom: PhantomData<T>,
}

impl<T> Clone for StableMapID<T>
{
    fn clone(&self) -> Self {
        Self { idx: self.idx.clone(), phantom: self.phantom.clone() }
    }
}

impl<T> Copy for StableMapID<T> {}
impl<T> PartialEq for StableMapID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}
impl<T> Eq for StableMapID<T> {}
impl<T> Hash for StableMapID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}
impl<T> PartialOrd for StableMapID<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.idx.partial_cmp(&other.idx)
    }
}
impl<T> Ord for StableMapID<T> 
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.idx.cmp(&other.idx)
    }
}
impl<T> From<usize> for StableMapID<T>
{
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}
impl<T> From<StableMapID<T>> for usize
{
    fn from(value: StableMapID<T>) -> Self {
        value.index()
    }
}
impl<T> Debug for StableMapID<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StableMapID({:?})", self.index())
    }
}
impl<T> Display for StableMapID<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StableMapID({})", self.index())
    }
}
impl<T> StableMapID<T>
{
    const fn new(value: usize) -> Self {
        Self { idx: value, phantom: PhantomData }
    }

    const fn index(self) -> usize { self.idx }
}

// Todo : integrate them with hexga ?
// Wrap the StableID type
// + reserve
// + impl the hexga traits

/// A map of value, where each value have a stable idx.
#[derive(Clone)]
pub struct StableMap<K,V>
{
    values : Vec<(K,V)>,
    index : Vec<usize>,
}

impl<K,V> Eq for StableMap<K,V> where K: Eq, V: Eq {}
impl<K,V> PartialEq for StableMap<K,V> 
    where K: PartialEq, V: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() { return false; }
        self.iter().all(|(idx,k,v)| other.get(idx).map_or(false, |(k2,v2)| k == k2 && v == v2))
    }
}

impl<K,V> Hash for StableMap<K,V> 
    where K: Hash, V: Hash
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.values.hash(state);
    }
}

impl<K,V> PartialOrd for StableMap<K,V> where K: PartialOrd, V: PartialOrd 
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.values.partial_cmp(&other.values)
    }
}
impl<K,V> Ord for StableMap<K,V> where K: Ord, V: Ord 
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.values.cmp(&other.values)
    }
}

impl<K,V> Default for StableMap<K,V>
{
    fn default() -> Self {
        Self { values: Default::default(), index: Default::default() }
    }
}

impl<K,V> Debug for StableMap<K,V>
    where K: Debug, V: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(self.values.iter().enumerate())
            .finish()
    }
}



impl<K,V> FromIterator<(K,V)> for StableMap<K,V>
    where K: Ord
{
    fn from_iter<I: IntoIterator<Item = (K,V)>>(iter: I) -> Self 
    {
        let mut value: Vec<(K,V)> = iter.into_iter().collect();
        value.sort_by(|a,b| a.0.cmp(&b.0));
        let len = value.len();
        Self { values: value, index: (0..len).collect() }
    }
}


impl<K,V> Extend<(K,V)> for StableMap<K,V>
    where K: Ord
{
    fn extend<T: IntoIterator<Item = (K,V)>>(&mut self, iter: T) {
        for (k,v) in iter.into_iter()
        {
            self.insert(k, v);
        }
    }
}

impl<'a,K,V> Extend<(&'a K,&'a V)> for StableMap<K,V>
    where K: Ord + Copy, V: Copy
{
    fn extend<T: IntoIterator<Item = (&'a K,&'a V)>>(&mut self, iter: T) {
        for (k,v) in iter.into_iter()
        {
            self.insert(*k, *v);
        }
    }
}

impl<K,V> StableMap<K,V>
    where K: Ord
{
    /// Return the idx and the old value if any
    #[inline(always)]
    pub fn insert(&mut self, key: K, value: V) -> (StableMapID<V>, Option<V>) // Similar to HashMap. Could also return the old key
    {
        self.insert_cyclic(key, |_| value)
    }

    #[track_caller]
    pub fn insert_cyclic<F>(&mut self, key: K, init: F) -> (StableMapID<V>, Option<V>)
        where F: FnOnce(StableMapID<V>) -> V
    {
        match self.index.binary_search_by(|i| self.values[*i].0.cmp(&key)) 
        {
            Ok(i) => 
            {
                let idx = self.index[i];
                let id = idx.into();
                (id, Some(core::mem::replace(&mut self.values[idx].1, init(id))))
            },
            Err(v) => 
            {
                let idx = self.values.len();
                let id = idx.into();
                self.values.push((key,init(id)));
                self.index.insert(v, idx);
                (id, None)
            },
        }
    }

    pub fn id(&self, key: &K) -> Option<StableMapID<V>> 
    {
        self.index.binary_search_by(|i| self.values[*i].0.cmp(&key)).ok().map(|v| v.into())
    }
    pub fn contains(&self, key: &K) -> bool { let Some(id) = self.id(key) else { return false; }; self.get(id).is_some() }

    pub fn lookup(&self, key: &K) -> Option<(StableMapID<V>, &V)> { let id = self.id(key)?; self.values.get(id.index()).map(|(_,v)| (id, v)) }
    pub fn lookup_mut(&mut self, key: &K) -> Option<(StableMapID<V>, &mut V)> { let id = self.id(key)?; self.values.get_mut(id.index()).map(|(_,v)| (id, v)) }
}

impl<K,V> StableMap<K,V>
{
    pub fn new() -> Self 
    {
        Self::default()
    }

    pub fn get(&self, index: StableMapID<V>) -> Option<(&K, &V)> { self.values.get(index.index()).map(|(k,v)| (k,v)) }
    pub fn get_mut(&mut self, index: StableMapID<V>) -> Option<(&K, &mut V)> { self.values.get_mut(index.index()).map(|(k,v)| (&*k,v)) }

    pub fn clear(&mut self) { self.index.clear(); self.values.clear(); }

    pub fn len(&self) -> usize { self.values.len() }
    pub fn capacity(&self) -> usize { self.values.capacity() }

    pub fn iter<'a>(&'a self) -> Iter<'a,K,V> { self.into_iter() }
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a,K,V> { self.into_iter() }

    pub fn keys(&self) -> impl Iterator<Item = &K> + DoubleEndedIterator + ExactSizeIterator { self.values.iter().map(|(k,_)| k) }
    pub fn values(&self) -> impl Iterator<Item = &V> + DoubleEndedIterator + ExactSizeIterator { self.values.iter().map(|(_,v)| v) }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> + DoubleEndedIterator + ExactSizeIterator { self.values.iter_mut().map(|(_,v)| v) }
}

impl<K,V> Index<StableMapID<V>> for StableMap<K,V>
{
    type Output=V;
    fn index(&self, index: StableMapID<V>) -> &Self::Output { &self.values[index.index()].1 }
}
impl<K,V> IndexMut<StableMapID<V>> for StableMap<K,V>
{
    fn index_mut(&mut self, index: StableMapID<V>) -> &mut Self::Output { &mut self.values[index.index()].1 }
}


pub struct IntoIter<K, V> {
    inner: std::iter::Enumerate<std::vec::IntoIter<(K, V)>>,
}
impl<'a, K, V> ExactSizeIterator for IntoIter<K, V> {}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (usize, K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(id, (k, v))| (id, k, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(id, (k, v))| (id, k, v))
    }
}


impl<K, V> IntoIterator for StableMap<K, V> {
    type Item = (usize, K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.values.into_iter().enumerate(),
        }
    }
}

pub struct Iter<'a, K, V> {
    inner: std::iter::Enumerate<std::slice::Iter<'a, (K, V)>>,
}
impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {}
impl<'a, K, V> Clone for Iter<'a, K, V>
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (StableMapID<V>, &'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(id, (k, v))| (id.into(), k, v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V> 
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(id, (k, v))| (id.into(), k, v))
    }
}

impl<'a, K, V> IntoIterator for &'a StableMap<K, V> {
    type Item = (StableMapID<V>, &'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.values.iter().enumerate(),
        }
    }
}

pub struct IterMut<'a, K, V> {
    inner: std::iter::Enumerate<std::slice::IterMut<'a, (K, V)>>,
}
impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (StableMapID<V>, &'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(id, (k, v))| (id.into(), &*k, v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
impl<'a, K, V> DoubleEndedIterator for IterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(id, (k, v))| (id.into(), &*k, v))
    }
}

impl<'a, K, V> IntoIterator for &'a mut StableMap<K, V> {
    type Item = (StableMapID<V>, &'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            inner: self.values.iter_mut().enumerate(),
        }
    }
}