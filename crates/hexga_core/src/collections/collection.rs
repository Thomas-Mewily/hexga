#[cfg(feature = "std")]
use std::path::Path;
use super::*;

/*
/// Contains element. Trait marker
pub trait Collection
{
    /*
    const SEQUENCE: bool;
    const ASSOCIATIF: bool;
     */
}
*/

pub trait Collection<T: ?Sized> //: Collection
{
    type Guard<'a> : Deref<Target = T> where Self: 'a;
}
pub trait CollectionMut<T: ?Sized> : Collection<T>
{
    type GuardMut<'a> : DerefMut<Target = T> where Self: 'a;
}

impl<T: ?Sized, S: ?Sized> Collection<T> for &S where S: Collection<T> { type Guard<'a> = <S as Collection<T>>::Guard<'a> where Self: 'a; }

impl<T: ?Sized, S: ?Sized> Collection<T> for &mut S where S: Collection<T> { type Guard<'a> = <S as Collection<T>>::Guard<'a> where Self: 'a; }
impl<T: ?Sized, S: ?Sized> CollectionMut<T> for &mut S where S: CollectionMut<T> { type GuardMut<'a> = <S as CollectionMut<T>>::GuardMut<'a> where Self: 'a; }


impl<T> Collection<T> for Option<T> { type Guard<'a> = &'a T where Self: 'a; }
impl<T> CollectionMut<T> for Option<T> { type GuardMut<'a> = &'a mut T where Self: 'a; }

impl<T, E> Collection<T> for Result<T, E> { type Guard<'a> = &'a T where Self: 'a; }
impl<T, E> CollectionMut<T> for Result<T, E> { type GuardMut<'a> = &'a mut T where Self: 'a; }


impl<T> Collection<T> for [T] { type Guard<'a> = &'a T where Self: 'a; }
impl<T> CollectionMut<T> for [T] { type GuardMut<'a> = &'a mut T where Self: 'a; }
impl<T> Collection<[T]> for [T] { type Guard<'a> = &'a [T] where Self: 'a; }
impl<T> CollectionMut<[T]> for [T] { type GuardMut<'a> = &'a mut [T] where Self: 'a; }

impl<T, const N: usize> Collection<T> for [T; N] { type Guard<'a> = &'a T where Self: 'a; }
impl<T, const N: usize> CollectionMut<T> for [T; N] { type GuardMut<'a> = &'a mut T where Self: 'a; }
impl<T, const N: usize> Collection<[T]> for [T; N] { type Guard<'a> = &'a [T] where Self: 'a; }
impl<T, const N: usize> CollectionMut<[T]> for [T; N] { type GuardMut<'a> = &'a mut [T] where Self: 'a; }

impl<T> Collection<T> for Vec<T> { type Guard<'a> = &'a T where Self: 'a; }
impl<T> CollectionMut<T> for Vec<T> { type GuardMut<'a> = &'a mut T where Self: 'a; }
impl<T> Collection<[T]> for Vec<T> { type Guard<'a> = &'a [T] where Self: 'a; }
impl<T> CollectionMut<[T]> for Vec<T> { type GuardMut<'a> = &'a mut [T] where Self: 'a; }

impl<T> Collection<T> for VecDeque<T> { type Guard<'a> = &'a T where Self: 'a; }
impl<T> CollectionMut<T> for VecDeque<T> { type GuardMut<'a> = &'a mut T where Self: 'a; }


impl Collection<u8> for str { type Guard<'a> = &'a u8 where Self: 'a; }
impl Collection<u8> for String { type Guard<'a> = &'a u8 where Self: 'a; }
impl CollectionMut<u8> for String { type GuardMut<'a> = &'a mut u8 where Self: 'a; }

impl Collection<char> for str { type Guard<'a> = Identity<char> where Self: 'a; }
impl Collection<char> for String { type Guard<'a> = Identity<char> where Self: 'a; }

#[cfg(feature = "std")]
mod std_impl
{
    use super::*;
    impl Collection<u8> for OsStr { type Guard<'a> = &'a u8 where Self: 'a; }
    impl Collection<u8> for OsString { type Guard<'a> = &'a u8 where Self: 'a; }
    impl CollectionMut<u8> for OsString { type GuardMut<'a> = &'a mut u8 where Self: 'a; }
    impl Collection<u8> for Path { type Guard<'a> = &'a u8 where Self: 'a; }
    impl Collection<u8> for PathBuf { type Guard<'a> = &'a u8 where Self: 'a; }
    impl CollectionMut<u8> for PathBuf { type GuardMut<'a> = &'a mut u8 where Self: 'a; }

    impl Collection<char> for OsStr { type Guard<'a> = &'a char where Self: 'a; }
    impl Collection<char> for OsString { type Guard<'a> = &'a char where Self: 'a; }
    impl CollectionMut<char> for OsString { type GuardMut<'a> = &'a mut char where Self: 'a; }
    impl Collection<char> for Path { type Guard<'a> = &'a char where Self: 'a; }
    impl Collection<char> for PathBuf { type Guard<'a> = &'a char where Self: 'a; }
    impl CollectionMut<char> for PathBuf { type GuardMut<'a> = &'a mut char where Self: 'a; }


    impl<K, V, S> Collection<V> for HashMap<K, V, S> { type Guard<'a> = &'a V where Self: 'a; }
    impl<K, V, S> CollectionMut<V> for HashMap<K, V, S> { type GuardMut<'a> = &'a mut V where Self: 'a; }

    impl<K, S> Collection<K> for HashSet<K, S> { type Guard<'a> = &'a K where Self: 'a; }
}

impl<K, V> Collection<V> for BTreeMap<K, V> { type Guard<'a> = &'a V where Self: 'a; }
impl<K, V> CollectionMut<V> for BTreeMap<K, V> { type GuardMut<'a> = &'a mut V where Self: 'a; }
impl<K> Collection<K> for BTreeSet<K> { type Guard<'a> = &'a K where Self: 'a; }
impl<T> Collection<T> for LinkedList<T> { type Guard<'a> = &'a T where Self: 'a; }
impl<T> CollectionMut<T> for LinkedList<T> { type GuardMut<'a> = &'a mut T where Self: 'a; }

impl<T> Collection<T> for BinaryHeap<T> where T: Ord { type Guard<'a> = &'a T where Self: 'a; }
impl<T> CollectionMut<T> for BinaryHeap<T> where T: Ord { type GuardMut<'a> = collections::binary_heap::PeekMut<'a,T> where Self: 'a; }

// TODO: impl Collection<BitRef> for iX/uX... 

/// Adding elements at the end don't invalidate other existing entries Key.
/// Examples: `Vec`, `VecDeque`, `GenVec`, `HashMap`, `BTreeMap`.
pub trait CollectionPushStableKey<T> : Collection<T> {}

// Adding elements at the front don't invalidate other existing entries Key.
// pub trait CollectionPushFrontStableKey: Collection {}

/// Removing elements from the end don't invalidate other existing entries Key.
/// Examples: `Vec`, `VecDeque`, `GenVec`.
pub trait CollectionPopStableKey<T> : Collection<T> {}

// Removing elements from the front don't invalidate other existing entries Key.
// pub trait CollectionPopFrontStableKey: Collection {}

/// Inserting elements at an arbitrary position don't invalidate other existing entries Key.
/// Examples: `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `GenVec`.
pub trait CollectionInsertStableKey<T> : Collection<T> {}

/// Removing elements at an arbitrary position or by key don't invalidate other existing entries Key.
/// Examples: `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `GenVec`.
pub trait CollectionRemoveStableKey<T> : Collection<T> {}

/// Editing elements don't invalidate other existing entries Key.
/// Examples: `HashMap`, `BTreeMap`, `Vec`, `GenVec`.
pub trait CollectionGetMutStableKey<T> : Collection<T> {}

pub trait CollectionStableKey<T>:
    CollectionPushStableKey<T> + CollectionPopStableKey<T> + CollectionInsertStableKey<T> + CollectionRemoveStableKey<T> + CollectionGetMutStableKey<T>
{
}
impl<S,T> CollectionStableKey<T> for S where
    S: CollectionPushStableKey<T> + CollectionPopStableKey<T> + CollectionInsertStableKey<T> + CollectionRemoveStableKey<T> + CollectionGetMutStableKey<T>
{
}


/*
Returning a mutable reference suck because it's break the getter/setter api
Returning any kind of reference suck because it don't work with singleton
*/

// Note: work for getter/setter (no &mut exposed) and singleton (no reference returned)
pub trait PushPop<T> : 
    Collection<T>
    + Push<T>
    + Pop<T>
    //+ Dup<T> // Dup depend on Last that return a reference, so it don't work for singleton
{}
impl<S: ?Sized,T> PushPop<T> for S where
    S: Collection<T>
    + Push<T>
    + Pop<T>
    //+ Dup<T>
{}

pub trait CollectionStackRef<T> : 
    Collection<T>
    // Not PeekLastMut<T> for respecting getter/setter separation, 
    // otherwise we need to set some dirty flag on the collection when mutating the last element, 
    // and I don't want to force it 
    + Last<T> 
    + PushPop<T>
{}
impl<S: ?Sized,T> CollectionStackRef<T> for S where
    S: Collection<T>
    + Last<T>
    + Push<T>
    + PushPop<T>
{}

pub trait CollectionStack<T> : 
    CollectionStackRef<T>
    + LastMut<T> 
{}
impl<S: ?Sized,T> CollectionStack<T> for S where
    S: CollectionStackRef<T>
    + LastMut<T> 
{}

#[cfg(feature = "std")]
mod std_impl_v2
{
    use super::*;
    impl<K, V, S> CollectionPushStableKey<V> for HashMap<K, V, S> {}
    impl<K, V, S> CollectionPopStableKey<V> for HashMap<K, V, S> {}
    impl<K, V, S> CollectionInsertStableKey<V> for HashMap<K, V, S> {}
    impl<K, V, S> CollectionRemoveStableKey<V> for HashMap<K, V, S> {}
    impl<K, V, S> CollectionGetMutStableKey<V> for HashMap<K, V, S> {}

    impl<K, S> CollectionPushStableKey<K> for HashSet<K, S> {}
    impl<K, S> CollectionPopStableKey<K> for HashSet<K, S> {}
    impl<K, S> CollectionInsertStableKey<K> for HashSet<K, S> {}
    impl<K, S> CollectionRemoveStableKey<K> for HashSet<K, S> {}
    impl<K, S> CollectionGetMutStableKey<K> for HashSet<K, S> {}

    impl<T> CollectionPushStableKey<T> for Vec<T> {}
    impl<T> CollectionPopStableKey<T> for Vec<T> {}
    impl<T> CollectionGetMutStableKey<T> for Vec<T> {}
}

impl<K, V> CollectionPushStableKey<V> for BTreeMap<K, V> {}
impl<K, V> CollectionPopStableKey<V> for BTreeMap<K, V> {}
impl<K, V> CollectionInsertStableKey<V> for BTreeMap<K, V> {}
impl<K, V> CollectionRemoveStableKey<V> for BTreeMap<K, V> {}
impl<K, V> CollectionGetMutStableKey<V> for BTreeMap<K, V> {}

impl<K> CollectionPushStableKey<K> for BTreeSet<K> {}
impl<K> CollectionPopStableKey<K> for BTreeSet<K> {}
impl<K> CollectionInsertStableKey<K> for BTreeSet<K> {}
impl<K> CollectionRemoveStableKey<K> for BTreeSet<K> {}
impl<K> CollectionGetMutStableKey<K> for BTreeSet<K> {}

// todo: impl for grid, image
/// The mapping of `key <=> value` is a bijection: each key point to a unique value, each value is uniquely pointed by a key.
pub trait CollectionBijective<T> : Collection<T> {}

impl<T, const N: usize> CollectionBijective<T> for [T; N] {}
impl<T> CollectionBijective<T> for Vec<T> {}
impl<T> CollectionBijective<T> for VecDeque<T> {}
impl<T> CollectionBijective<T> for [T] {}

impl<S,T> CollectionBijective<T> for &S where S: CollectionBijective<T> {}
impl<S,T> CollectionBijective<T> for &mut S where S: CollectionBijective<T> {}

#[cfg(feature = "std")]
impl<K, V, S> CollectionBijective<V> for HashMap<K, V, S> {}
impl<K, V> CollectionBijective<V> for BTreeMap<K, V> {}
#[cfg(feature = "std")]
impl<K, S> CollectionBijective<K> for HashSet<K, S> {}
impl<K> CollectionBijective<K> for BTreeSet<K> {}

impl CollectionBijective<u8> for str {}
impl CollectionBijective<u8> for String {}
#[cfg(feature = "std")]
impl CollectionBijective<u8> for OsStr {}

impl CollectionBijective<char> for str {}
impl CollectionBijective<char> for String {}
#[cfg(feature = "std")]
impl CollectionBijective<char> for OsStr {}

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
