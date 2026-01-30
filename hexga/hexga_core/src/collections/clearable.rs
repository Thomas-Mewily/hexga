use super::*;

/// Clear the collection
pub trait Clear
{
    fn clear(&mut self);
}

impl<T> Clear for Vec<T>
{
    #[inline(always)]
    fn clear(&mut self) { self.clear(); }
}
impl<T> Clear for VecDeque<T>
{
    #[inline(always)]
    fn clear(&mut self) { self.clear(); }
}
#[cfg(feature = "std")]
impl<T, S> Clear for HashSet<T, S>
{
    #[inline(always)]
    fn clear(&mut self) { self.clear(); }
}
impl<T> Clear for BinaryHeap<T>
{
    #[inline(always)]
    fn clear(&mut self) { self.clear(); }
}
impl<T> Clear for BTreeSet<T>
{
    #[inline(always)]
    fn clear(&mut self) { self.clear(); }
}
impl<T> Clear for LinkedList<T>
{
    #[inline(always)]
    fn clear(&mut self) { self.clear(); }
}
#[cfg(feature = "std")]
impl<K, V, S> Clear for HashMap<K, V, S>
{
    #[inline(always)]
    fn clear(&mut self) { self.clear(); }
}
impl<K, V> Clear for BTreeMap<K, V>
{
    #[inline(always)]
    fn clear(&mut self) { self.clear(); }
}
impl Clear for String
{
    #[inline(always)]
    fn clear(&mut self) { self.clear(); }
}
#[cfg(feature = "std")]
impl Clear for OsString
{
    #[inline(always)]
    fn clear(&mut self) { self.clear(); }
}
#[cfg(feature = "std")]
impl Clear for PathBuf
{
    #[inline(always)]
    fn clear(&mut self) { self.clear(); }
}
impl<T> Clear for Option<T>
{
    #[inline(always)]
    fn clear(&mut self) { *self = None; }
}
