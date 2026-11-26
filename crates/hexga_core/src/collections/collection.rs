use super::*;

/// The shared reference type for a collection output.
pub trait Collection<Output: ?Sized> {
    type Ref<'a>: Deref<Target = Output>
    where
        Self: 'a,
        Output: 'a;
}
/// The mutable reference type for a collection output.
pub trait CollectionMut<Output: ?Sized>: Collection<Output> {
    type RefMut<'a>: DerefMut<Target = Output>
    where
        Self: 'a,
        Output: 'a;
}

/// Access to a collection's output through an index of type `Idx`.
///
/// Indexed access can distinguish outputs by index even when both outputs have
/// the same type, as with `Result<T, T>`. Generic code that also needs the
/// output-based reference type can add a `Collection<Self::Output>` bound.
pub trait CollectionIndex<Idx> {
    type Output: ?Sized;
    type Ref<'a>: Deref<Target = Self::Output>
    where
        Self: 'a,
        Self::Output: 'a;
}

pub trait CollectionIndexMut<Idx>: CollectionIndex<Idx> {
    type RefMut<'a>: DerefMut<Target = Self::Output>
    where
        Self: 'a,
        Self::Output: 'a;
}

impl<Output: ?Sized, S: ?Sized> Collection<Output> for &S
where
    S: Collection<Output>,
{
    type Ref<'a>
        = <S as Collection<Output>>::Ref<'a>
    where
        Self: 'a,
        Output: 'a;
}

impl<Idx, S: ?Sized> CollectionIndex<Idx> for &S
where
    S: CollectionIndex<Idx>,
{
    type Output = S::Output;
    type Ref<'a>
        = <S as CollectionIndex<Idx>>::Ref<'a>
    where
        Self: 'a,
        Self::Output: 'a;
}

impl<Output: ?Sized, S: ?Sized> Collection<Output> for &mut S
where
    S: Collection<Output>,
{
    type Ref<'a>
        = <S as Collection<Output>>::Ref<'a>
    where
        Self: 'a,
        Output: 'a;
}

impl<Output: ?Sized, S: ?Sized> CollectionMut<Output> for &mut S
where
    S: CollectionMut<Output>,
{
    type RefMut<'a>
        = <S as CollectionMut<Output>>::RefMut<'a>
    where
        Self: 'a,
        Output: 'a;
}

impl<Idx, S: ?Sized> CollectionIndex<Idx> for &mut S
where
    S: CollectionIndex<Idx>,
{
    type Output = S::Output;
    type Ref<'a>
        = <S as CollectionIndex<Idx>>::Ref<'a>
    where
        Self: 'a,
        Self::Output: 'a;
}

impl<Idx, S: ?Sized> CollectionIndexMut<Idx> for &mut S
where
    S: CollectionIndexMut<Idx>,
{
    type RefMut<'a>
        = <S as CollectionIndexMut<Idx>>::RefMut<'a>
    where
        Self: 'a,
        Self::Output: 'a;
}

impl<T> Collection<T> for [T] {
    type Ref<'a>
        = &'a T
    where
        Self: 'a,
        T: 'a;
}
impl<T> CollectionMut<T> for [T] {
    type RefMut<'a>
        = &'a mut T
    where
        Self: 'a,
        T: 'a;
}

impl<T> Collection<[T]> for [T] {
    type Ref<'a>
        = &'a [T]
    where
        Self: 'a,
        T: 'a;
}
impl<T> CollectionMut<[T]> for [T] {
    type RefMut<'a>
        = &'a mut [T]
    where
        Self: 'a,
        T: 'a;
}

impl<T, Idx> CollectionIndex<Idx> for [T]
where
    Idx: SliceIndex<[T]>,
    [T]: Collection<Idx::Output>,
{
    type Output = Idx::Output;
    type Ref<'a>
        = &'a Idx::Output
    where
        Self: 'a,
        Idx::Output: 'a;
}
impl<T, Idx> CollectionIndexMut<Idx> for [T]
where
    Idx: SliceIndex<[T]>,
    [T]: CollectionMut<Idx::Output>,
{
    type RefMut<'a>
        = &'a mut Idx::Output
    where
        Self: 'a,
        Idx::Output: 'a;
}

impl<T> Collection<T> for Option<T> {
    type Ref<'a>
        = &'a T
    where
        Self: 'a,
        T: 'a;
}
impl<T> CollectionMut<T> for Option<T> {
    type RefMut<'a>
        = &'a mut T
    where
        Self: 'a,
        T: 'a;
}

impl<T> CollectionIndex<()> for Option<T> {
    type Output = T;
    type Ref<'a>
        = <Self as Collection<T>>::Ref<'a>
    where
        Self: 'a,
        T: 'a;
}
impl<T> CollectionIndexMut<()> for Option<T> {
    type RefMut<'a>
        = <Self as CollectionMut<T>>::RefMut<'a>
    where
        Self: 'a,
        T: 'a;
}

impl<T> Collection<T> for LinkedList<T> {
    type Ref<'a>
        = &'a T
    where
        Self: 'a,
        T: 'a;
}
impl<T> CollectionMut<T> for LinkedList<T> {
    type RefMut<'a>
        = &'a mut T
    where
        Self: 'a,
        T: 'a;
}

impl<T> CollectionIndex<usize> for LinkedList<T> {
    type Output = T;
    type Ref<'a>
        = <Self as Collection<T>>::Ref<'a>
    where
        Self: 'a,
        T: 'a;
}
impl<T> CollectionIndexMut<usize> for LinkedList<T> {
    type RefMut<'a>
        = <Self as CollectionMut<T>>::RefMut<'a>
    where
        Self: 'a,
        T: 'a;
}

impl<T: Ord> Collection<T> for BinaryHeap<T> {
    type Ref<'a>
        = &'a T
    where
        Self: 'a,
        T: 'a;
}
impl<T: Ord> CollectionMut<T> for BinaryHeap<T> {
    type RefMut<'a>
        = collections::binary_heap::PeekMut<'a, T>
    where
        Self: 'a,
        T: 'a;
}

impl<T: Ord> CollectionIndex<()> for BinaryHeap<T> {
    type Output = T;
    type Ref<'a>
        = <Self as Collection<T>>::Ref<'a>
    where
        Self: 'a,
        T: 'a;
}
impl<T: Ord> CollectionIndexMut<()> for BinaryHeap<T> {
    type RefMut<'a>
        = <Self as CollectionMut<T>>::RefMut<'a>
    where
        Self: 'a,
        T: 'a;
}

#[cfg(feature = "std")]
mod platform_strings {
    use super::*;
    use std::path::Path;

    hexga_map_on::map_on!(
        (OsStr, OsString, Path, PathBuf),
        ($ty:ty) => {
            impl Collection<char> for $ty
            {
                type Ref<'a> = Identity<char> where Self: 'a;
            }
            impl CollectionIndex<usize> for $ty
            {
                type Output = char;
                type Ref<'a> = <Self as Collection<char>>::Ref<'a> where Self: 'a;
            }
        }
    );
}

/// Adding elements at the end don't invalidate other existing entries Key.
/// Examples: `Vec`, `VecDeque`, `GenVec`, `HashMap`, `BTreeMap`.
pub trait CollectionPushStableKey {}

// Adding elements at the front don't invalidate other existing entries Key.
// pub trait CollectionPushFrontStableKey: Collection {}

/// Removing elements from the end don't invalidate other existing entries Key.
/// Examples: `Vec`, `VecDeque`, `GenVec`.
pub trait CollectionPopStableKey {}

// Removing elements from the front don't invalidate other existing entries Key.
// pub trait CollectionPopFrontStableKey: Collection {}

/// Inserting elements at an arbitrary position don't invalidate other existing entries Key.
/// Examples: `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `GenVec`.
pub trait CollectionInsertStableKey {}

/// Removing elements at an arbitrary position or by key don't invalidate other existing entries Key.
/// Examples: `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `GenVec`.
pub trait CollectionRemoveStableKey {}

/// Editing elements don't invalidate other existing entries Key.
/// Examples: `HashMap`, `BTreeMap`, `Vec`, `GenVec`.
pub trait CollectionGetMutStableKey {}

pub trait CollectionStableKey:
    CollectionPushStableKey + CollectionPopStableKey + CollectionInsertStableKey + CollectionRemoveStableKey + CollectionGetMutStableKey
{
}
impl<S> CollectionStableKey for S where
    S: CollectionPushStableKey + CollectionPopStableKey + CollectionInsertStableKey + CollectionRemoveStableKey + CollectionGetMutStableKey
{
}

/*
Returning a mutable reference suck because it's break the getter/setter api
Returning any kind of reference suck because it don't work with singleton
*/

/*
// Note: work for getter/setter (no &mut exposed) and singleton (no reference returned)
pub trait PushPop<T> :
    //Collection<T>
    Push<T>
    + Pop<T>
    //+ Dup<T> // Dup depend on Last that return a reference, so it don't work for singleton
{}
impl<S: ?Sized,T> PushPop<T> for S where
    S:
    //Collection<T>
    Push<T>
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
*/

#[cfg(feature = "std")]
mod std_impl_v2 {
    use super::*;
    impl<K, V, S> CollectionPushStableKey for HashMap<K, V, S> {}
    impl<K, V, S> CollectionPopStableKey for HashMap<K, V, S> {}
    impl<K, V, S> CollectionInsertStableKey for HashMap<K, V, S> {}
    impl<K, V, S> CollectionRemoveStableKey for HashMap<K, V, S> {}
    impl<K, V, S> CollectionGetMutStableKey for HashMap<K, V, S> {}

    impl<K, S> CollectionPushStableKey for HashSet<K, S> {}
    impl<K, S> CollectionPopStableKey for HashSet<K, S> {}
    impl<K, S> CollectionInsertStableKey for HashSet<K, S> {}
    impl<K, S> CollectionRemoveStableKey for HashSet<K, S> {}
    impl<K, S> CollectionGetMutStableKey for HashSet<K, S> {}

    impl<T, A: Allocator> CollectionPushStableKey for Vec<T, A> {}
    impl<T, A: Allocator> CollectionPopStableKey for Vec<T, A> {}
    impl<T, A: Allocator> CollectionGetMutStableKey for Vec<T, A> {}
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
pub trait CollectionBijective {}

impl<T, const N: usize> CollectionBijective for [T; N] {}
impl<T, A: Allocator> CollectionBijective for Vec<T, A> {}
impl<T> CollectionBijective for VecDeque<T> {}
impl<T> CollectionBijective for [T] {}

impl<S> CollectionBijective for &S where S: CollectionBijective {}
impl<S> CollectionBijective for &mut S where S: CollectionBijective {}

#[cfg(feature = "std")]
impl<K, V, S> CollectionBijective for HashMap<K, V, S> {}
impl<K, V> CollectionBijective for BTreeMap<K, V> {}
#[cfg(feature = "std")]
impl<K, S> CollectionBijective for HashSet<K, S> {}
impl<K> CollectionBijective for BTreeSet<K> {}

impl CollectionBijective for str {}
impl CollectionBijective for String {}
#[cfg(feature = "std")]
impl CollectionBijective for OsStr {}
/*
impl CollectionBijective<u8> for str {}
impl CollectionBijective<u8> for String {}
#[cfg(feature = "std")]
impl CollectionBijective<u8> for OsStr {}

impl CollectionBijective<char> for str {}
impl CollectionBijective<char> for String {}
#[cfg(feature = "std")]
impl CollectionBijective<char> for OsStr {}
*/

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
