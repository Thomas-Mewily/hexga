use super::*;


pub trait IterIndex<T, const N : usize>
{
    type IterIndex : Iterator<Item=Vector<T,N>>;

    /// Iter over all `Point` in self.
    ///
    /// The last value is always excluded.
    ///
    /// Work on `Point`, Rect and Grid ([`Point1`], [`Point2`], [`Point3`], [`Point4`], [`Rect1`], [`Rect2`], [`Rect3`], [`Rect4`]) for ex.
    ///
    /// ```rust
    /// use hexga_math::prelude::*;
    ///
    /// let expected_points = [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2)].map(|(x,y)| point2(x, y));
    ///
    /// for (i, expected) in point2(2, 3).iter_index().zip(expected_points.iter())
    /// {
    ///     assert_eq!(&i, expected);
    /// }
    /// ```
    fn iter_index(&self) -> Self::IterIndex;
}

/// Iterates over all N-dimensional indices in a rectangular region.
///
/// Yields every `Vector<Idx, N>` from `Vector::ZERO` up to `end`
/// (exclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
pub struct VectorIter<Idx, const N : usize> where Idx : Integer
{
    cur : Vector<Idx,N>,
    end : Vector<Idx,N>,
}

impl<Idx, const N : usize> VectorIter<Idx, N> where Idx : Integer
{
    pub fn from_cur_to_end(cur : Vector<Idx,N>, end : Vector<Idx,N>) -> Self
    {
        Self { cur, end : if end.have_area_usize() { end } else { Vector::ZERO } }
    }

    pub fn new(end : Vector<Idx,N>) -> Self
    {
        Self::from_cur_to_end(Vector::ZERO, end)
    }
}

impl<Idx, const N : usize> Iterator for VectorIter<Idx, N> where Idx : Integer
{
    type Item=Vector<Idx,N>;

    fn next(&mut self) -> Option<Self::Item>
    {
        let v = self.cur;
        let mut i = 0;

        while i < N
        {
            self.cur[i].increment();
            if self.cur[i] < self.end[i]
            {
                return Some(v);
            }
            self.cur[i] = Idx::ZERO;
            i += 1;
        }

        self.cur = self.end;

        if self.end.all_zero()
        {
            None
        }else
        {
            self.end = zero();
            Some(v)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let len = self.len();
        (len, Some(len))
    }
}
impl<Idx, const N : usize> std::iter::FusedIterator for VectorIter<Idx, N> where Idx : Integer {}
impl<Idx, const N : usize> std::iter::ExactSizeIterator for VectorIter<Idx, N> where Idx : Integer
{
    fn len(&self) -> usize {
        self.end.area_usize() - unsafe { Vector::<Idx,N>::to_index_unchecked(self.cur, self.end) }
    }
}


impl<Idx,const N : usize> IterIndex<Idx,N> for Vector<Idx,N> where Idx : Integer
{
    type IterIndex = VectorIter<Idx,N>;

    fn iter_index(&self) -> Self::IterIndex {
        Self::IterIndex::new(*self)
    }
}