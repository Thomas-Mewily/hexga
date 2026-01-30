#[cfg(feature = "std")]
use std::path::Path;

use super::*;

/// Contains element
pub trait Collection {}

impl<T> Collection for &T where T: Collection {}
impl<T> Collection for &mut T where T: Collection {}

impl<T, const N: usize> Collection for [T; N] {}
impl<T> Collection for Vec<T> {}
impl<T> Collection for VecDeque<T> {}
impl<T> Collection for [T] {}
impl<T> Collection for &[T] {}
impl<T> Collection for &mut [T] {}

impl Collection for str {}
impl Collection for &str {}
impl Collection for &mut str {}
impl Collection for String {}
#[cfg(feature = "std")]
impl Collection for OsStr {}
#[cfg(feature = "std")]
impl Collection for OsString {}
#[cfg(feature = "std")]
impl Collection for Path {}
#[cfg(feature = "std")]
impl Collection for PathBuf {}

#[cfg(feature = "std")]
impl<K, V, S> Collection for HashMap<K, V, S> {}
impl<K, V> Collection for BTreeMap<K, V> {}
#[cfg(feature = "std")]
impl<K, V> Collection for HashSet<K, V> {}
impl<K> Collection for BTreeSet<K> {}

impl<T> Collection for LinkedList<T> {}
impl<T> Collection for BinaryHeap<T> {}

/// Doing operation on an entry (remove them, edit them) will not invalidate the others entries.
///
/// Implementation include `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `GenVec`...
pub trait CollectionStableKey: Collection {}

#[cfg(feature = "std")]
impl<K, V, S> CollectionStableKey for HashMap<K, V, S> {}
impl<K, V> CollectionStableKey for BTreeMap<K, V> {}
#[cfg(feature = "std")]
impl<K, S> CollectionStableKey for HashSet<K, S> {}
impl<K> CollectionStableKey for BTreeSet<K> {}

// todo: impl for grid, image
/// The mapping of `key <=> value` is a bijection: each key point to a unique value, each value is uniquely pointed by a key.
pub trait CollectionBijective: Collection {}

impl<T, const N: usize> CollectionBijective for [T; N] {}
impl<T> CollectionBijective for Vec<T> {}
impl<T> CollectionBijective for VecDeque<T> {}
impl<T> CollectionBijective for [T] {}
impl<T> CollectionBijective for &[T] {}
impl<T> CollectionBijective for &mut [T] {}

#[cfg(feature = "std")]
impl<K, V, S> CollectionBijective for HashMap<K, V, S> {}
impl<K, V> CollectionBijective for BTreeMap<K, V> {}
#[cfg(feature = "std")]
impl<K, S> CollectionBijective for HashSet<K, S> {}
impl<K> CollectionBijective for BTreeSet<K> {}

impl CollectionBijective for str {}
impl CollectionBijective for &str {}
impl CollectionBijective for &mut str {}
impl CollectionBijective for String {}
#[cfg(feature = "std")]
impl CollectionBijective for OsStr {}
