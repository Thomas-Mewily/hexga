use core::cell::Cell;
use core::iter::FusedIterator;
use core::mem::size_of;
use core::ops::{Index, Range, Shl, Shr};
use hexga_core::prelude::*;
use hexga_number::{Integer, One, Zero};

pub mod prelude
{
    pub use super::{Bit, BitIndex, BitMut, BitRef, BitSet, BitValueAndIndex};
}

pub mod traits
{
}

pub type BitIndex = u8;
pub type Bit = bool;
pub type BitRef = Identity<Bit>;
pub type BitValueAndIndex = (Bit, BitIndex);

/// A mutable bit reference that writes the edited value back when dropped.
pub struct BitMut<'a, T: Integer>
{
    value: Bit,
    mask: T,
    storage: &'a Cell<T>,
}

impl<'a, T: Integer + Shl<BitIndex, Output = T>> BitMut<'a, T>
{
    fn new(storage: &'a Cell<T>, index: BitIndex) -> Self
    {
        let mask = T::ONE << index;
        let value = (storage.get() & mask) != T::ZERO;
        Self { value, mask, storage }
    }
}

impl<T: Integer> Deref for BitMut<'_, T>
{
    type Target = Bit;

    fn deref(&self) -> &Bit { &self.value }
}

impl<T: Integer> DerefMut for BitMut<'_, T>
{
    fn deref_mut(&mut self) -> &mut Bit { &mut self.value }
}

impl<T: Integer> Drop for BitMut<'_, T>
{
    fn drop(&mut self)
    {
        // Another guard may have updated a different bit since this was created.
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
/// Index zero refers to the least significant bit. The length counts set bits,
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

impl<T: Integer> Length for BitSet<T>
{
    fn len(&self) -> usize
    {
        self.0.count_bit_ones() as usize
    }
}

impl<T: Integer> Capacity<Bit> for BitSet<T>
{
    fn capacity(&self) -> usize { size_of::<T>() * u8::BITS as usize }
}

impl<T: Zero> Clear for BitSet<T>
{
    fn clear(&mut self) { *self = Self::ZERO; }
}

impl<T: Integer> CollectionBijective for BitSet<T> {}

impl<T: Integer> Collection<BitIndex> for BitSet<T>
{
    type Output = Bit;
    type Ref<'a> = BitRef where Self: 'a;
}

impl<T: Integer> CollectionMut<BitIndex> for BitSet<T>
{
    type RefMut<'a> = BitMut<'a, T> where Self: 'a;
}

impl<T: Integer> BitSet<T>
{
    fn index_range(&self) -> Range<usize>
    {
        0..self.capacity().min(usize::from(BitIndex::MAX) + 1)
    }
}

impl<T: Integer + Shr<BitIndex, Output = T>> BitSet<T>
{
    fn bit_at(&self, index: BitIndex) -> Bit
    {
        ((self.0 >> index) & T::ONE) != T::ZERO
    }

    /// Iterates over all bits, starting at the least significant bit.
    pub fn iter(&self) -> Iter<T> { (*self).into_iter() }

    /// Iterates over each bit and its index, starting at index zero.
    pub fn iter_indexed(&self) -> impl DoubleEndedIterator<Item = BitValueAndIndex> + ExactSizeIterator + FusedIterator + use<T>
    {
        self.iter().enumerate().map(|(index, value)| (value, index as BitIndex))
    }
}

impl<T: Integer + Shl<BitIndex, Output = T>> BitSet<T>
{
    /// Iterates over mutable bit references, starting at the least significant bit.
    ///
    /// Each reference writes its bit back when dropped.
    pub fn iter_mut(&mut self) -> IterMut<'_, T>
    {
        let indices = self.index_range();
        IterMut { storage: Cell::from_mut(&mut self.0), indices }
    }
}

impl<T: Integer + Shr<BitIndex, Output = T>> Get<BitIndex> for BitSet<T>
{
    fn get<'a>(&'a self, index: BitIndex) -> Option<Self::Ref<'a>> where Self::Output: 'a
    {
        self.index_range().contains(&usize::from(index)).then(|| Identity(self.bit_at(index)))
    }
}

impl<T: Integer + Shr<BitIndex, Output = T> + Shl<BitIndex, Output = T>> GetMut<BitIndex> for BitSet<T>
{
    fn get_mut<'a>(&'a mut self, index: BitIndex) -> Option<Self::RefMut<'a>> where Self::Output: 'a
    {
        self.index_range()
            .contains(&usize::from(index))
            .then(|| BitMut::new(Cell::from_mut(&mut self.0), index))
    }
}

impl<T: Integer + Shr<BitIndex, Output = T>> TryGet<BitIndex> for BitSet<T>
{
    type Error = IndexOutOfBounds<BitIndex, Range<usize>>;

    fn try_get<'a>(&'a self, index: BitIndex) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a
    {
        self.get(index).ok_or_else(|| IndexOutOfBounds::new(index, self.index_range()))
    }
}

impl<T: Integer + Shr<BitIndex, Output = T> + Shl<BitIndex, Output = T>> TryGetMut<BitIndex> for BitSet<T>
{
    fn try_get_mut<'a>(&'a mut self, index: BitIndex) -> Result<Self::RefMut<'a>, Self::Error> where Self::Output: 'a
    {
        let range = self.index_range();
        self.get_mut(index).ok_or_else(|| IndexOutOfBounds::new(index, range))
    }
}

impl<T: Integer + Shr<BitIndex, Output = T>> First<BitIndex> for BitSet<T>
{
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>> where Self::Output: 'a { self.get(0) }
}

impl<T: Integer + Shr<BitIndex, Output = T>> Last<BitIndex> for BitSet<T>
{
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>> where Self::Output: 'a
    {
        self.get(self.index_range().next_back()? as BitIndex)
    }
}

impl<T: Integer + Shr<BitIndex, Output = T> + Shl<BitIndex, Output = T>> FirstMut<BitIndex> for BitSet<T>
{
    fn first_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>> where Self::Output: 'a { self.get_mut(0) }
}

impl<T: Integer + Shr<BitIndex, Output = T> + Shl<BitIndex, Output = T>> LastMut<BitIndex> for BitSet<T>
{
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>> where Self::Output: 'a
    {
        self.get_mut(self.index_range().next_back()? as BitIndex)
    }
}

impl<T: Integer + Shr<BitIndex, Output = T>> Index<BitIndex> for BitSet<T>
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

/// An iterator over the bits of an integer.
#[derive(Clone, Debug)]
pub struct Iter<T>
{
    bits: BitSet<T>,
    indices: Range<usize>,
}

impl<T: Integer + Shr<BitIndex, Output = T>> Iterator for Iter<T>
{
    type Item = Bit;

    fn next(&mut self) -> Option<Bit>
    {
        self.indices.next().map(|index| self.bits.bit_at(index as BitIndex))
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.indices.size_hint() }

    fn nth(&mut self, n: usize) -> Option<Bit>
    {
        self.indices.nth(n).map(|index| self.bits.bit_at(index as BitIndex))
    }
}

impl<T: Integer + Shr<BitIndex, Output = T>> DoubleEndedIterator for Iter<T>
{
    fn next_back(&mut self) -> Option<Bit>
    {
        self.indices.next_back().map(|index| self.bits.bit_at(index as BitIndex))
    }
}

impl<T: Integer + Shr<BitIndex, Output = T>> ExactSizeIterator for Iter<T> {}
impl<T: Integer + Shr<BitIndex, Output = T>> FusedIterator for Iter<T> {}

impl<T: Integer + Shr<BitIndex, Output = T>> IntoIterator for BitSet<T>
{
    type Item = Bit;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        Iter { bits: self, indices: self.index_range() }
    }
}

impl<T: Integer + Shr<BitIndex, Output = T>> IntoIterator for &BitSet<T>
{
    type Item = Bit;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

/// An iterator over the mutable bits of an integer.
pub struct IterMut<'a, T: Integer>
{
    storage: &'a Cell<T>,
    indices: Range<usize>,
}

impl<'a, T: Integer + Shl<BitIndex, Output = T>> Iterator for IterMut<'a, T>
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

impl<T: Integer + Shl<BitIndex, Output = T>> DoubleEndedIterator for IterMut<'_, T>
{
    fn next_back(&mut self) -> Option<Self::Item>
    {
        self.indices.next_back().map(|index| BitMut::new(self.storage, index as BitIndex))
    }
}

impl<T: Integer + Shl<BitIndex, Output = T>> ExactSizeIterator for IterMut<'_, T> {}
impl<T: Integer + Shl<BitIndex, Output = T>> FusedIterator for IterMut<'_, T> {}

impl<'a, T: Integer + Shl<BitIndex, Output = T>> IntoIterator for &'a mut BitSet<T>
{
    type Item = BitMut<'a, T>;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn zero_and_one_use_the_backing_integer_constants()
    {
        assert_eq!(BitSet::<u16>::default(), BitSet::<u16>::ZERO);
        assert_eq!(BitSet::<u16>::ZERO, BitSet(0));
        assert_eq!(BitSet::<u16>::ONE, BitSet(1));
        assert_eq!(BitSet::<u16>::ZERO.capacity(), 16);
        assert_eq!(BitSet::<u16>::ONE.len(), 1);
    }

    #[test]
    fn mutable_bit_references_write_back_and_preserve_other_bits()
    {
        let mut bits = BitSet(0b0101u8);
        *bits.get_mut(0).unwrap() = false;
        *bits.get_mut(1).unwrap() = true;
        assert_eq!(bits.0, 0b0110);

        assert_eq!(bits.replace(2, false), Some(true));
        assert!(!bits[2]);
        assert_eq!(bits.0, 0b0010);
    }

    #[test]
    fn mutable_iteration_preserves_edits_from_multiple_guards()
    {
        let mut bits = BitSet(0b0000_0010u8);
        {
            let mut iter = bits.iter_mut();
            let mut first = iter.next().unwrap();
            let mut last = iter.next_back().unwrap();
            assert_eq!(iter.len(), 6);
            *first = true;
            *last = true;
            drop(last);
            drop(first);
        }
        assert_eq!(bits.0, 0b1000_0011);

        for mut bit in &mut bits
        {
            *bit = !*bit;
        }
        assert_eq!(bits.0, 0b0111_1100);
    }

    #[test]
    fn signed_bits_and_bounds_use_the_storage_width()
    {
        let mut bits = BitSet::<i8>::default();
        assert_eq!(bits.0, 0);
        assert_eq!(bits.capacity(), 8);
        *bits.last_mut().unwrap() = true;
        assert_eq!(bits.0, i8::MIN);
        assert_eq!(bits.len(), 1);
        assert!(bits[7]);
        assert!(!bits[6]);
        assert_eq!(bits.try_get(8), Err(IndexOutOfBounds::new(8, 0..8)));
        assert!(matches!(bits.try_get_mut(8), Err(error) if error == IndexOutOfBounds::new(8, 0..8)));

        bits.clear();
        assert_eq!(bits, BitSet(0));
        assert_eq!(bits.len(), 0);
        assert_eq!(bits.capacity(), 8);
    }

    #[test]
    fn iteration_visits_true_and_false_bits_in_index_order()
    {
        let bits = BitSet(0b1000_0010u8);
        let mut iter = (&bits).into_iter();
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next_back(), Some(true));
        assert_eq!(iter.len(), 6);
        assert_eq!(iter.next(), Some(true));
        assert!(iter.all(|bit| !bit));

        let set_bits: Vec<_> = bits.iter_indexed().filter(|&(bit, _)| bit).collect();
        assert_eq!(set_bits, [(true, 1), (true, 7)]);
        assert_eq!(BitSet(1u64 << 63).into_iter().last(), Some(true));
    }
}
