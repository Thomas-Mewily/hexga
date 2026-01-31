pub use core::iter::{Product, Sum};
use super::*;

#[cfg(feature = "std")]
use std::error::Error;
#[cfg(feature = "std")]
use core::any::Any;

/// A trait to create a value from an iterator.
pub trait TryFromIterator<T> : Sized
{
    type Error;
    fn try_from_iter<It: IntoIterator<Item = T>>(iter: It) -> Result<Self, Self::Error>;
}
impl<T> TryFromIterator<T> for Vec<T> {
    type Error = Never;
    fn try_from_iter<It: IntoIterator<Item = T>>(iter: It) -> Result<Self, Self::Error> {
        Ok(Self::from_iter(iter))
    }
}

impl<T> TryFromIterator<T> for VecDeque<T> {
    type Error = Never;
    fn try_from_iter<It: IntoIterator<Item = T>>(iter: It) -> Result<Self, Self::Error> {
        Ok(Self::from_iter(iter))
    }
}

impl<T> TryFromIterator<T> for LinkedList<T> {
    type Error = Never;
    fn try_from_iter<It: IntoIterator<Item = T>>(iter: It) -> Result<Self, Self::Error> {
        Ok(Self::from_iter(iter))
    }
}

impl<T: Ord> TryFromIterator<T> for BinaryHeap<T> {
    type Error = Never;
    fn try_from_iter<It: IntoIterator<Item = T>>(iter: It) -> Result<Self, Self::Error> {
        Ok(Self::from_iter(iter))
    }
}

#[cfg(feature = "std")]
impl<T: Eq + Hash, S: BuildHasher + Default> TryFromIterator<T> for HashSet<T, S> {
    type Error = Never; // HashSet::from_iter never fails
    fn try_from_iter<It: IntoIterator<Item = T>>(iter: It) -> Result<Self, Self::Error> {
        Ok(Self::from_iter(iter))
    }
}

impl<T: Ord> TryFromIterator<T> for BTreeSet<T> {
    type Error = Never;
    fn try_from_iter<It: IntoIterator<Item = T>>(iter: It) -> Result<Self, Self::Error> {
        Ok(Self::from_iter(iter))
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V, S: BuildHasher + Default> TryFromIterator<(K, V)> for HashMap<K, V, S> {
    type Error = Never;
    fn try_from_iter<It: IntoIterator<Item = (K, V)>>(iter: It) -> Result<Self, Self::Error> {
        Ok(Self::from_iter(iter))
    }
}

impl<K: Ord, V> TryFromIterator<(K, V)> for BTreeMap<K, V> {
    type Error = Never;
    fn try_from_iter<It: IntoIterator<Item = (K, V)>>(iter: It) -> Result<Self, Self::Error> {
        Ok(Self::from_iter(iter))
    }
}

impl TryFromIterator<char> for String {
    type Error = Never;
    fn try_from_iter<It: IntoIterator<Item = char>>(iter: It) -> Result<Self, Self::Error> {
        Ok(Self::from_iter(iter))
    }
}

impl<'a> TryFromIterator<&'a str> for String {
    type Error = Never;
    fn try_from_iter<It: IntoIterator<Item = &'a str>>(iter: It) -> Result<Self, Self::Error> {
        Ok(Self::from_iter(iter))
    }
}




#[repr(transparent)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct CapacityFullError<T = ()>
{
    pub element: T,
}

impl<T> From<T> for CapacityFullError<T>
{
    fn from(value: T) -> Self { Self::new(value) }
}

impl<T> CapacityFullError<T>
{
    pub const fn new(element: T) -> CapacityFullError<T> { CapacityFullError { element: element } }
}

const CAPERROR: &'static str = "capacity full";

#[cfg(feature = "std")]
/// Requires `features="std"`.
impl<T: Any> Error for CapacityFullError<T> {}

impl<T> core::fmt::Display for CapacityFullError<T>
{
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result { write!(f, "{}", CAPERROR) }
}

impl<T> core::fmt::Debug for CapacityFullError<T>
{
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result
    {
        write!(f, "{}: {}", "CapacityError", CAPERROR)
    }
}


#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct WrongLenError<T> {
    pub remaining: T,
}
impl<T> From<T> for WrongLenError<T>
{
    fn from(value: T) -> Self { Self::new(value) }
}

impl<T> WrongLenError<T>
{
    pub const fn new(remaining: T) -> WrongLenError<T> { WrongLenError { remaining } }
}
impl<T> Debug for WrongLenError<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Wrong len")
    }
}

impl<T, const N: usize> TryFromIterator<T> for [T; N] {
    type Error = WrongLenError<Vec<T>>;
    fn try_from_iter<It: IntoIterator<Item = T>>(iter: It) -> Result<Self, Self::Error> {
        let vec: Vec<T> = iter.into_iter().collect();
        vec.try_into().map_err(|remaining| WrongLenError { remaining })
    }
}

pub trait IterExtension<'a, Item>
where
    Self: 'a,
    &'a Self: IntoIterator<Item = Item>,
{
    fn iter(&'a self) -> <&'a Self as IntoIterator>::IntoIter { self.into_iter() }
}
impl<'a, Item, T> IterExtension<'a, Item> for T where &'a T: IntoIterator<Item = Item> + 'a {}

pub trait IterPredicate<Item>: IntoIterator<Item = Item> + Sized
{
    #[inline(always)]
    fn any<P>(self, p: P) -> bool
    where
        P: FnMut(Item) -> bool,
    {
        self.into_iter().any(p)
    }
    #[inline(always)]
    fn all<P>(self, p: P) -> bool
    where
        P: FnMut(Item) -> bool,
    {
        self.into_iter().all(p)
    }
    //#[inline(always)]
    //fn for_each<F>(self, f: F) where F: FnMut(Item) { self.into_iter().for_each(f); }

    #[inline(always)]
    fn any_with<P, O>(self, other: O, mut p: P) -> bool
    where
        P: FnMut(Item, Item) -> bool,
        O: IntoIterator<Item = Item>,
    {
        let it_a = self.into_iter();
        let it_b = other.into_iter();
        it_a.zip(it_b).any(|v| p(v.0, v.1))
    }

    #[inline(always)]
    fn all_with<P, O>(self, other: O, mut p: P) -> bool
    where
        P: FnMut(Item, Item) -> bool,
        O: IntoIterator<Item = Item>,
    {
        let it_a = self.into_iter();
        let it_b = other.into_iter();
        it_a.zip(it_b).all(|v| p(v.0, v.1))
    }
}
impl<Item, T> IterPredicate<Item> for T where T: IntoIterator<Item = Item> + Sized {}

pub trait IterMutExtension<'a, Item>
where
    Self: 'a + IterExtension<'a, Item>,
    &'a Self: IntoIterator<Item = Item>,
    &'a mut Self: IntoIterator<Item = Item>,
{
    fn iter_mut(&'a mut self) -> <&'a mut Self as IntoIterator>::IntoIter { self.into_iter() }

    //fn for_each_mut<F>(&'a mut self, f: F) where F: FnMut(Item) { self.iter_mut().for_each(f); }
}
impl<'a, Item, T> IterMutExtension<'a, Item> for T
where
    &'a mut T: IntoIterator<Item = Item> + 'a,
    &'a Self: IntoIterator<Item = Item>,
{
}
