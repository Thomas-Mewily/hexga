use super::*;

pub trait CollectTo<T>: Sized + IntoIterator<Item = T>
{
    fn to_vec(self) -> Vec<T> { self.into_iter().collect() }
    fn to_linkedlist(self) -> std::collections::LinkedList<T> { self.into_iter().collect() }
    fn to_vecdeque(self) -> VecDeque<T> { self.into_iter().collect() }
    fn to_hashset(self) -> HashSet<T>
    where
        T: Eq + Hash,
    {
        self.into_iter().collect()
    }
    fn to_btreeset(self) -> BTreeSet<T>
    where
        T: Ord,
    {
        self.into_iter().collect()
    }
}
impl<I, T> CollectTo<T> for I where I: IntoIterator<Item = T> {}

pub trait CollectToMap<T1, T2>: Sized + IntoIterator<Item = (T1, T2)>
{
    fn to_hashmap(self) -> HashMap<T1, T2>
    where
        T1: Eq + Hash,
    {
        self.into_iter().collect()
    }
    fn to_btreemap(self) -> BTreeMap<T1, T2>
    where
        T1: Ord,
    {
        self.into_iter().collect()
    }
}
impl<I, T1, T2> CollectToMap<T1, T2> for I where I: IntoIterator<Item = (T1, T2)> {}
