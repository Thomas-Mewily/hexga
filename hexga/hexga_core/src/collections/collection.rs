use super::*;

/// Contains element
pub trait Collection {}



impl<T> Collection for &T where T: Collection {}
impl<T> Collection for &mut T where T: Collection {}

impl<T,const N: usize> Collection for [T;N] {}
impl<T> Collection for Vec<T> {}
impl<T> Collection for VecDeque<T> {}
impl<T> Collection for [T] {}
impl<T> Collection for &[T] {}
impl<T> Collection for &mut [T] {}

impl Collection for str {}
impl Collection for &str {}
impl Collection for &mut str {}
impl Collection for String {}
impl Collection for OsStr {}

impl<K,V,S> Collection for HashMap<K,V,S> {}
impl<K,V> Collection for BTreeMap<K,V> {}
impl<K,V> Collection for HashSet<K,V> {}
impl<K> Collection for BTreeSet<K> {}

impl<T> Collection for LinkedList<T> {}
impl<T> Collection for BinaryHeap<T> {}




/// Doing operation on an entry (remove them, edit them) will not invalidate the others entries.
///
/// Implementation include `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `GenVec`...
pub trait CollectionStableKey : Collection {}

impl<K,V,S> CollectionStableKey for HashMap<K,V,S> {}
impl<K,V> CollectionStableKey for BTreeMap<K,V> {}
impl<K,S> CollectionStableKey for HashSet<K,S> {}
impl<K> CollectionStableKey for BTreeSet<K> {}
