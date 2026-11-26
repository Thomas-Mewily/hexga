use core::cell::Cell;
use core::iter::FusedIterator;
use core::mem::size_of;
use core::ops::{Index, Range};
use std::ops::{Add, Sub};
use std::slice::GetDisjointMutError;
use hexga_core::prelude::*;
use hexga_number::{BitArithmetic, BitManip, One, Zero};

pub mod prelude
{
    pub use super::{
        BitSet, Bit, BitIndex, 
        //BitMut, BitRef, BitValueAndIndex,
        //IterOnes, IterZeros,
    };
}

pub mod traits
{
    pub use super::BitSetType;
}

trait_marker!(BitSetType : One + Zero + BitArithmetic + BitManip + PartialEq + Add<Output=Self> + Sub<Output=Self>);

pub type BitIndex = u8;
pub type Bit = bool;
pub type BitRef = Identity<Bit>;

/// A mutable bit reference that writes the edited value back when dropped.
pub struct BitMut<'a, T: BitSetType>
{
    value: Bit,
    mask: T,
    storage: &'a Cell<T>,
    dirty: bool,
}

impl<'a, T: BitSetType> BitMut<'a, T>
{
    fn new(storage: &'a Cell<T>, index: BitIndex) -> Self
    {
        let mask = T::ONE << index;
        let value = (storage.get() & mask) != T::ZERO;
        Self { value, mask, storage, dirty: false }
    }
}

impl<T: BitSetType> Deref for BitMut<'_, T>
{
    type Target = Bit;

    fn deref(&self) -> &Bit { &self.value }
}

impl<T: BitSetType> DerefMut for BitMut<'_, T>
{
    fn deref_mut(&mut self) -> &mut Bit
    {
        self.dirty = true;
        &mut self.value
    }
}

impl<T: BitSetType> Drop for BitMut<'_, T>
{
    fn drop(&mut self)
    {
        if !self.dirty { return; }
        let current = self.storage.get();
        if self.value
        {
            self.storage.set(current | self.mask);
        }
        else
        {
            self.storage.set(current & !self.mask);
        }
    }
}

/// A fixed-width collection of bits backed by an integer.
///
/// Index zero refers to the least significant bit. The length counts the number of bits with value 1 ([`BitManip::count_ones`]),
/// and the capacity is the storage width in bits.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BitSet<T>(pub T);

impl<T: Zero> Zero for BitSet<T>
{
    const ZERO: Self = Self(T::ZERO);
}

impl<T: One> One for BitSet<T>
{
    const ONE: Self = Self(T::ONE);
}

impl<T: Zero> Default for BitSet<T>
{
    fn default() -> Self { Self::ZERO }
}

impl<T> From<T> for BitSet<T>
{
    fn from(value: T) -> Self { Self(value) }
}

impl<T: BitSetType> Length for BitSet<T>
{
    fn len(&self) -> usize
    {
        self.0.count_ones() as usize
    }
}

impl<T: BitSetType> Capacity<Bit> for BitSet<T>
{
    fn capacity(&self) -> usize { Self::BITS }
}

impl<T: Zero> Clear for BitSet<T>
{
    fn clear(&mut self) { *self = Self::ZERO; }
}

impl<T: BitSetType> CollectionBijective for BitSet<T> {}

impl<T: BitSetType> Collection<Bit> for BitSet<T>
{
    type Ref<'a> = BitRef where Self: 'a, Bit: 'a;
}
impl<T: BitSetType> CollectionMut<Bit> for BitSet<T>
{
    type RefMut<'a> = BitMut<'a, T> where Self: 'a, Bit: 'a;
}

impl<T: BitSetType> CollectionIndex<BitIndex> for BitSet<T>
{
    type Output = Bit;
    type Ref<'a> = BitRef where Self: 'a;
}

impl<T: BitSetType> CollectionIndexMut<BitIndex> for BitSet<T>
{
    type RefMut<'a> = BitMut<'a, T> where Self: 'a;
}

impl<T: BitSetType> BitSet<T>
{
    pub const BITS : usize = size_of::<T>() * u8::BITS as usize;

    pub fn index_range() -> Range<usize>
    {
        const { assert!(size_of::<T>() * u8::BITS as usize <= usize::MAX) };
        0..size_of::<T>() * u8::BITS as usize
    }

    pub fn bit_index_range() -> Range<BitIndex>
    {
        const {
            assert!(size_of::<T>() * u8::BITS as usize <= u8::MAX as usize);
        }
        0..(size_of::<T>() * u8::BITS as usize) as BitIndex
    }
}

impl<T: BitSetType> BitSet<T>
{
    pub fn bit_at(&self, index: BitIndex) -> Bit
    {
        ((self.0 >> index) & T::ONE) != T::ZERO
    }

    /// Iterates over all bits, starting at the least significant bit.
    pub fn iter(&self) -> Iter<T> { (*self).into_iter() }

    /// Iterates over mutable bit references, starting at the least significant bit.
    ///
    /// Each reference writes its bit back when dropped.
    pub fn iter_mut(&mut self) -> IterMut<'_, T>
    {
        let indices = Self::index_range();
        IterMut { storage: Cell::from_mut(&mut self.0), indices }
    }

    /// Iterates over the indices of bits with value 1, in ascending order.
    pub fn iter_ones(self) -> IterOnes<T>
    {
        IterOnes::new(self)
    }

    /// Iterates over the indices of bits with value 0, in ascending order.
    pub fn iter_zeros(self) -> IterZeros<T>
    {
        IterZeros::new(self)
    }
}

impl<T: BitSetType> Get<BitIndex> for BitSet<T>
{
    fn get<'a>(&'a self, index: BitIndex) -> Option<Self::Ref<'a>> where Self::Output: 'a
    {
        Self::index_range().contains(&usize::from(index)).then(|| Identity(self.bit_at(index)))
    }
}

impl<T: BitSetType> GetMut<BitIndex> for BitSet<T>
{
    fn get_mut<'a>(&'a mut self, index: BitIndex) -> Option<Self::RefMut<'a>> where Self::Output: 'a
    {
        Self::index_range()
            .contains(&usize::from(index))
            .then(|| BitMut::new(Cell::from_mut(&mut self.0), index))
    }
}

impl<T: BitSetType> TryGet<BitIndex> for BitSet<T>
{
    type Error = IndexOutOfBounds<BitIndex, Range<usize>>;

    fn try_get<'a>(&'a self, index: BitIndex) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a
    {
        self.get(index).ok_or_else(|| IndexOutOfBounds::new(index, Self::index_range()))
    }
}

impl<T: BitSetType> TryGetMut<BitIndex> for BitSet<T>
{
    fn try_get_mut<'a>(&'a mut self, index: BitIndex) -> Result<Self::RefMut<'a>, Self::Error> where Self::Output: 'a
    {
        let range = Self::index_range();
        self.get_mut(index).ok_or_else(|| IndexOutOfBounds::new(index, range))
    }
}

impl<T: BitSetType> GetManyMut<BitIndex> for BitSet<T>
{
    fn try_get_many_mut<'a, const N: usize>(&'a mut self, indices: [BitIndex; N]) -> Result<[Self::RefMut<'a>; N], ManyMutError>
    where Self::Output: 'a {
        let range = Self::index_range();

        for &i in &indices
        {
            if !range.contains(&usize::from(i))
            {
                return Err(GetDisjointMutError::IndexOutOfBounds);
            }
        }

        for a in 0..N
        {
            for b in (a + 1)..N
            {
                if indices[a] == indices[b]
                {
                    return Err(GetDisjointMutError::OverlappingIndices);
                }
            }
        }

        let cell: &'a Cell<T> = Cell::from_mut(&mut self.0);
        Ok(core::array::from_fn(|k| BitMut::new(cell, indices[k])))
    }
}

impl<T: BitSetType> First<BitIndex> for BitSet<T>
{
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>> where Self::Output: 'a { self.get(0) }
}

impl<T: BitSetType> Last<BitIndex> for BitSet<T>
{
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>> where Self::Output: 'a
    {
        self.get(Self::index_range().next_back()? as BitIndex)
    }
}

impl<T: BitSetType> FirstMut<BitIndex> for BitSet<T>
{
    fn first_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>> where Self::Output: 'a { self.get_mut(0) }
}

impl<T: BitSetType> LastMut<BitIndex> for BitSet<T>
{
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>> where Self::Output: 'a
    {
        self.get_mut(Self::index_range().next_back()? as BitIndex)
    }
}

impl<T: BitSetType> Index<BitIndex> for BitSet<T>
{
    type Output = Bit;

    #[track_caller]
    fn index(&self, index: BitIndex) -> &Self::Output
    {
        match *self.get_or_panic(index)
        {
            true => &true,
            false => &false,
        }
    }
}
/// An iterator over all bits of a `BitSet`, in ascending order.
#[derive(Clone, Debug)]
pub struct Iter<T: BitSetType>
{
    bits: BitSet<T>,   // the value being read
    mask: T,           // one-hot cursor: 1, 2, 4, ... ; 0 when exhausted
}

impl<T: BitSetType> Iter<T>
{
    fn new(bits: BitSet<T>) -> Self
    {
        Self { bits, mask: T::ONE }
    }
}

impl<T: BitSetType> Iterator for Iter<T>
{
    type Item = Bit;

    fn next(&mut self) -> Option<Bit>
    {
        if self.mask == T::ZERO { return None; }
        let bit = (self.bits.0 & self.mask) != T::ZERO;
        self.mask = self.mask << 1u8;
        Some(bit)
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let n = if self.mask == T::ZERO
        {
            0
        }
        else
        {
            self.bits.capacity() - self.mask.trailing_zeros() as usize
        };
        (n, Some(n))
    }

    fn count(self) -> usize
    {
        if self.mask == T::ZERO
        {
            0
        }
        else
        {
            self.bits.capacity() - self.mask.trailing_zeros() as usize
        }
    }
}

impl<T: BitSetType> ExactSizeIterator for Iter<T> {}
impl<T: BitSetType> FusedIterator for Iter<T> {}

impl<T: BitSetType> IntoIterator for BitSet<T>
{
    type Item = Bit;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        Iter::new(self)
    }
}

impl<T: BitSetType> IntoIterator for &BitSet<T>
{
    type Item = Bit;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

/// An iterator over the mutable bits of an integer.
pub struct IterMut<'a, T: BitSetType>
{
    storage: &'a Cell<T>,
    indices: Range<usize>,
}

impl<'a, T: BitSetType> Iterator for IterMut<'a, T>
{
    type Item = BitMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.indices.next().map(|index| BitMut::new(self.storage, index as BitIndex))
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.indices.size_hint() }

    fn nth(&mut self, n: usize) -> Option<Self::Item>
    {
        self.indices.nth(n).map(|index| BitMut::new(self.storage, index as BitIndex))
    }
}

impl<T: BitSetType> DoubleEndedIterator for IterMut<'_, T>
{
    fn next_back(&mut self) -> Option<Self::Item>
    {
        self.indices.next_back().map(|index| BitMut::new(self.storage, index as BitIndex))
    }
}

impl<T: BitSetType> ExactSizeIterator for IterMut<'_, T> {}
impl<T: BitSetType> FusedIterator for IterMut<'_, T> {}

impl<'a, T: BitSetType> IntoIterator for &'a mut BitSet<T>
{
    type Item = BitMut<'a, T>;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

/// Iterator over the indices of set bits, in ascending order.
#[derive(Clone, Debug)]
pub struct IterOnes<T: BitSetType>
{
    remaining: T,
}

impl<T: BitSetType> IterOnes<T>
{
    fn new(bits: BitSet<T>) -> Self
    {
        Self { remaining: bits.0 }
    }
}

impl<T: BitSetType> Iterator for IterOnes<T>
{
    type Item = BitIndex;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.remaining == T::ZERO { return None; }
        let idx = self.remaining.trailing_zeros() as BitIndex;
        self.remaining = self.remaining & (self.remaining - T::ONE);
        Some(idx)
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        // Lower bound is 0 until we know the popcount; upper is popcount.
        let n = self.remaining.count_ones() as usize;
        (n, Some(n))
    }
}

impl<T: BitSetType> ExactSizeIterator for IterOnes<T> {}
impl<T: BitSetType> FusedIterator for IterOnes<T> {}


/// Iterator over the indices of cleared bits, in ascending order.
#[derive(Clone, Debug)]
pub struct IterZeros<T: BitSetType>
{
    remaining: T,
}

impl<T: BitSetType> IterZeros<T>
{
    fn new(bits: BitSet<T>) -> Self
    {
        Self { remaining: !bits.0, }
    }
}

impl<T: BitSetType> Iterator for IterZeros<T>
{
    type Item = BitIndex;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.remaining == T::ZERO { return None; }
        let idx = self.remaining.trailing_zeros() as BitIndex;
        self.remaining = self.remaining & (self.remaining - T::ONE);
        Some(idx)
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let n = self.remaining.count_ones() as usize;
        (n, Some(n))
    }
}

impl<T: BitSetType> ExactSizeIterator for IterZeros<T> {}
impl<T: BitSetType> FusedIterator for IterZeros<T> {}