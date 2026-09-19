use super::*;

pub struct StableSetID<T>
{
    idx : usize,
    phantom: PhantomData<T>,
}

impl<T> Clone for StableSetID<T>
{
    fn clone(&self) -> Self {
        Self { idx: self.idx.clone(), phantom: self.phantom.clone() }
    }
}
impl<T> Copy for StableSetID<T> {}
impl<T> PartialEq for StableSetID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}
impl<T> Eq for StableSetID<T> {}
impl<T> Hash for StableSetID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}
impl<T> PartialOrd for StableSetID<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.idx.partial_cmp(&other.idx)
    }
}
impl<T> Ord for StableSetID<T> 
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.idx.cmp(&other.idx)
    }
}
impl<T> From<usize> for StableSetID<T>
{
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}
impl<T> From<StableSetID<T>> for usize
{
    fn from(value: StableSetID<T>) -> Self {
        value.index()
    }
}
impl<T> Debug for StableSetID<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StableSetID({:?})", self.index())
    }
}
impl<T> Display for StableSetID<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StableSetID({})", self.index())
    }
}
impl<T> StableSetID<T>
{
    const fn new(value: usize) -> Self {
        Self { idx: value, phantom: PhantomData }
    }

    const fn index(self) -> usize { self.idx }
}

/// A set of value, where each value have a stable idx.
#[derive(Clone)]
pub struct StableSet<T>
{
    // values: Vec<Option<T>>, or GenVec<T>. Should also support removing element in that case
    values : Vec<T>,
    idx  : Vec<usize>,
}

impl<T> Eq for StableSet<T> where T: Eq {}
impl<T> PartialEq for StableSet<T> 
    where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() { return false; }
        self.iter().all(|(idx, v)| other.get(idx).map_or(false, |v2| v == v2))
    }
}

impl<T> Hash for StableSet<T> 
    where T: Hash
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.values.hash(state);
    }
}

impl<T> PartialOrd for StableSet<T> where T: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.values.partial_cmp(&other.values)
    }
}
impl<T> Ord for StableSet<T> where T: Ord
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.values.cmp(&other.values)
    }
}


impl<T> Default for StableSet<T>
{
    fn default() -> Self {
        Self { values: Default::default(), idx: Default::default() }
    }
}

impl<T> Debug for StableSet<T>
    where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(self.values.iter().enumerate())
            .finish()
    }
}

impl<T> FromIterator<T> for StableSet<T>
    where T: Ord
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self 
    {
        let mut value: Vec<T> = iter.into_iter().collect();
        value.sort();
        let len = value.len();
        Self { values: value, idx: (0..len).collect() }
    }
}

impl<T> Extend<T> for StableSet<T>
    where T: Ord
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for v in iter.into_iter()
        {
            self.insert(v);
        }
    }
}


impl<'a,T> Extend<&'a T> for StableSet<T>
    where T: Ord + Copy
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        for v in iter.into_iter()
        {
            self.insert(*v);
        }
    }
}


impl<T> StableSet<T>
    where T: Ord
{
    /// Return the idx and the old value if any
    pub fn insert(&mut self, value: T) -> (StableSetID<T>, Option<T>)
    {
        match self.idx.binary_search_by(|i| self.values[*i].cmp(&value)) 
        {
            Ok(i) => 
            {
                let idx = self.idx[i];
                (idx.into(), Some(core::mem::replace(&mut self.values[idx], value)))
            },
            Err(v) => 
            {
                let idx = self.values.len();
                self.values.push(value);
                self.idx.insert(v, idx);
                (idx.into(), None)
            },
        }
    }

    pub fn id(&self, value: &T) -> Option<StableSetID<T>> 
    {
        self.idx.binary_search_by(|i| self.values[*i].cmp(&value)).ok().map(|v| v.into())
    }
    pub fn contains(&self, value: &T) -> bool { let Some(id) = self.id(value) else { return false; }; self.get(id).is_some() }

    pub fn lookup(&self, key: &T) -> Option<(StableSetID<T>, &T)> { let id = self.id(key)?; self.values.get(id.index()).map(|v| (id, v)) }
}

impl<T> StableSet<T>
{
    pub fn new() -> Self 
    {
        Self::default()
    }

    pub fn get(&self, index: StableSetID<T>) -> Option<&T> { self.values.get(index.index()) }
    pub fn get_mut(&mut self, index: StableSetID<T>) -> Option<&mut T> { self.values.get_mut(index.index()) }

    pub fn clear(&mut self) { self.idx.clear(); self.values.clear(); }

    pub fn len(&self) -> usize { self.values.len() }
    pub fn capacity(&self) -> usize { self.values.capacity() }

    pub fn iter<'a>(&'a self) -> Iter<'a,T> { self.into_iter() }
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a,T> { self.into_iter() }
}

impl<T> Index<StableSetID<T>> for StableSet<T>
{
    type Output=T;
    fn index(&self, index: StableSetID<T>) -> &Self::Output { &self.values[index.index()] }
}
impl<T> IndexMut<StableSetID<T>> for StableSet<T>
{
    fn index_mut(&mut self, index: StableSetID<T>) -> &mut Self::Output { &mut self.values[index.index()] }
}


pub struct IntoIter<T> {
    inner: std::iter::Enumerate<std::vec::IntoIter<T>>,
}
impl<'a, T> ExactSizeIterator for IntoIter<T> {}

impl<T> Iterator for IntoIter<T> {
    type Item = (StableSetID<T>, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(idx, v)| (idx.into(), v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(idx, v)| (idx.into(), v))
    }
}


impl<T> IntoIterator for StableSet<T> {
    type Item = (StableSetID<T>, T);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.values.into_iter().enumerate(),
        }
    }
}

pub struct Iter<'a, T> {
    inner: std::iter::Enumerate<std::slice::Iter<'a, T>>,
}
impl<'a, T> ExactSizeIterator for Iter<'a, T> {}
impl<'a, T> Clone for Iter<'a, T>
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (StableSetID<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(idx, v)| (idx.into(), v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> 
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(idx, v)| (idx.into(), v))
    }
}

impl<'a, T> IntoIterator for &'a StableSet<T> {
    type Item = (StableSetID<T>, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.values.iter().enumerate(),
        }
    }
}

pub struct IterMut<'a, T> {
    inner: std::iter::Enumerate<std::slice::IterMut<'a, T>>,
}
impl<'a, T> ExactSizeIterator for IterMut<'a, T> {}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (StableSetID<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(idx, v)| (idx.into(), v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(idx, v)| (idx.into(), v))
    }
}

impl<'a, T> IntoIterator for &'a mut StableSet<T> {
    type Item = (StableSetID<T>, &'a mut T);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            inner: self.values.iter_mut().enumerate(),
        }
    }
}