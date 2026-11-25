use super::*;

/// Clear the collection
pub trait Clear
{
    fn clear(&mut self);
}

impl<T, A: Allocator> Clear for Vec<T, A>
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
mod std_impl
{
    pub use super::*;

    impl<K, S> Clear for HashSet<K, S>
    {
        #[inline(always)]
        fn clear(&mut self) { self.clear(); }
    }

    impl<K, V, S> Clear for HashMap<K, V, S>
    {
        #[inline(always)]
        fn clear(&mut self) { self.clear(); }
    }
    impl Clear for OsString
    {
        #[inline(always)]
        fn clear(&mut self) { self.clear(); }
    }
    impl Clear for PathBuf
    {
        #[inline(always)]
        fn clear(&mut self) { self.clear(); }
    }
}

impl<T> Clear for BinaryHeap<T> where T: Ord
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

impl<T> Clear for Option<T>
{
    #[inline(always)]
    fn clear(&mut self) { *self = None; }
}
impl<T, E> Clear for Result<T, E>
    where E: Default
{
    #[inline(always)]
    fn clear(&mut self) { *self = Err(___()); }
}
