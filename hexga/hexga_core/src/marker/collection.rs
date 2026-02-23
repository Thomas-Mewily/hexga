#[cfg(feature = "std")]
use std::path::Path;

use super::*;

/// Contains element
pub trait Collection
{
    /*
    const SEQUENCE: bool;
    const ASSOCIATIF: bool;
     */
}

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

/// Adding elements at the end don't invalidate other existing entries Key.
/// Examples: `Vec`, `VecDeque`, `GenVec`, `HashMap`, `BTreeMap`.
pub trait CollectionPushStableKey: Collection {}

// Adding elements at the front don't invalidate other existing entries Key.
// pub trait CollectionPushFrontStableKey: Collection {}

/// Removing elements from the end don't invalidate other existing entries Key.
/// Examples: `Vec`, `VecDeque`, `GenVec`.
pub trait CollectionPopStableKey: Collection {}

// Removing elements from the front don't invalidate other existing entries Key.
// pub trait CollectionPopFrontStableKey: Collection {}

/// Inserting elements at an arbitrary position don't invalidate other existing entries Key.
/// Examples: `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `GenVec`.
pub trait CollectionInsertStableKey: Collection {}

/// Removing elements at an arbitrary position or by key don't invalidate other existing entries Key.
/// Examples: `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `GenVec`.
pub trait CollectionRemoveStableKey: Collection {}

/// Editing elements don't invalidate other existing entries Key.
/// Examples: `HashMap`, `BTreeMap`, `Vec`, `GenVec`.
pub trait CollectionGetMutStableKey: Collection {}

pub trait CollectionStableKey: CollectionPushStableKey + CollectionPopStableKey + CollectionInsertStableKey + CollectionRemoveStableKey + CollectionGetMutStableKey {}
impl<C> CollectionStableKey for C where C: CollectionPushStableKey + CollectionPopStableKey + CollectionInsertStableKey + CollectionRemoveStableKey + CollectionGetMutStableKey {}

#[cfg(feature = "std")]
mod std_impl
{
    use super::*;
    impl<K, V, S> CollectionPushStableKey for HashMap<K, V, S> {}
    impl<K, V, S> CollectionPopStableKey for HashMap<K, V, S> {}
    impl<K, V, S> CollectionInsertStableKey for HashMap<K, V, S> {}
    impl<K, V, S> CollectionRemoveStableKey for HashMap<K, V, S> {}
    impl<K, V, S> CollectionGetMutStableKey for HashMap<K, V, S> {}

    impl<K, V> CollectionPushStableKey for HashSet<K, V> {}
    impl<K, V> CollectionPopStableKey for HashSet<K, V> {}
    impl<K, V> CollectionInsertStableKey for HashSet<K, V> {}
    impl<K, V> CollectionRemoveStableKey for HashSet<K, V> {}
    impl<K, V> CollectionGetMutStableKey for HashSet<K, V> {}

    impl<T> CollectionPushStableKey for Vec<T> {}
    impl<T> CollectionPopStableKey for Vec<T> {}
    impl<T> CollectionGetMutStableKey for Vec<T> {}
}

impl<K, V> CollectionPushStableKey for BTreeMap<K, V> {}
impl<K, V> CollectionPopStableKey for BTreeMap<K, V> {}
impl<K, V> CollectionInsertStableKey for BTreeMap<K, V> {}
impl<K, V> CollectionRemoveStableKey for BTreeMap<K, V> {}
impl<K, V> CollectionGetMutStableKey for BTreeMap<K, V> {}

impl<K> CollectionPushStableKey for BTreeSet<K> {}
impl<K> CollectionPopStableKey for BTreeSet<K> {}
impl<K> CollectionInsertStableKey for BTreeSet<K> {}
impl<K> CollectionRemoveStableKey for BTreeSet<K> {}
impl<K> CollectionGetMutStableKey for BTreeSet<K> {}

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

/*
/// An unique key/index that can be used to identify a single element
pub trait CollectionKey: Collection + Get<Self::Indice>
{
    type Indice;
}
impl<T, const N: usize> CollectionKey for [T; N] { type Indice = usize; }
impl<T> CollectionKey for Vec<T> { type Indice = usize; }
impl<T> CollectionKey for VecDeque<T> { type Indice = usize; }
impl<T> CollectionKey for [T] { type Indice = usize; }
impl<T> CollectionKey for &[T] { type Indice = usize; }
impl<T> CollectionKey for &mut [T] { type Indice = usize; }
*/