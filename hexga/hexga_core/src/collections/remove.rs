use super::*;

pub trait Remove<Idx: ?Sized>
{
    type Output;
    fn remove(&mut self, index: Idx) -> Option<Self::Output>;
}

impl<T> Remove<usize> for Vec<T>
{
    type Output = T;
    fn remove(&mut self, index: usize) -> Option<Self::Output>
    {
        if index < self.len()
        {
            Some(self.remove(index))
        }
        else
        {
            None
        }
    }
}

impl<T> Remove<usize> for VecDeque<T>
{
    type Output = T;
    fn remove(&mut self, index: usize) -> Option<Self::Output> { self.remove(index) }
}

impl<K, V, S, Q> Remove<&Q> for HashMap<K, V, S>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    K: Eq + Hash,
    S: BuildHasher,
{
    type Output = V;
    fn remove(&mut self, index: &Q) -> Option<Self::Output> { self.remove(index) }
}
impl<K, V, Q> Remove<&Q> for BTreeMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord,
    K: Ord,
{
    type Output = V;
    fn remove(&mut self, index: &Q) -> Option<Self::Output> { self.remove(index) }
}

impl<K, S, Q> Remove<&Q> for HashSet<K, S>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq + Clone,
    K: Eq + Hash,
    S: BuildHasher,
{
    type Output = ();
    fn remove(&mut self, index: &Q) -> Option<Self::Output> { self.remove(index).then_some(()) }
}
impl<K, Q> Remove<&Q> for BTreeSet<K>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord + Clone,
    K: Ord,
{
    type Output = ();
    fn remove(&mut self, index: &Q) -> Option<Self::Output> { self.remove(index).then_some(()) }
}
