use super::*;


pub mod prelude
{
    pub use super::RectangleIter;
}

/// Iter over all idx inside a rectangle
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
pub struct RectangleIter<Idx, const N : usize> where Idx : Integer
{
    offset : Vector<Idx,N>,
    iter   : VectorIter<Idx, N>,
}
impl<Idx, const N : usize> RectangleIter<Idx,N> where Idx : Integer
{
    pub fn new(rect : Rectangle<Idx,N>) -> Self { Self::from_vec_iter(rect.pos, rect.size.iter_index()) }
    pub const fn from_vec_iter(offset : Vector<Idx,N>, iter : VectorIter<Idx, N>) -> Self { Self { offset, iter } }
}

impl<Idx, const N : usize> Iterator for RectangleIter<Idx,N> where Idx : Integer
{
    type Item = <VectorIter<Idx, N> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| v + self.offset)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<Idx, const N : usize> std::iter::FusedIterator for RectangleIter<Idx,N> where Idx : Integer, VectorIter<Idx,N> : std::iter::FusedIterator {}
impl<Idx, const N : usize> std::iter::ExactSizeIterator for RectangleIter<Idx,N> where Idx : Integer, VectorIter<Idx,N> : std::iter::ExactSizeIterator {}