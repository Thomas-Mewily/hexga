use crate::*;


pub trait IterIndex<T, const N : usize>
{
    type IterIndex : Iterator<Item=Vector<T,N>>;

    /// Iter over all point in self.
    /// 
    /// The last value is always excluded.
    /// 
    /// Work on Point, Rect and Grid ([Point1], [Point2], [Point3], [Point4], [Rect1], [Rect2], [Rect3], [Rect4]) for ex.
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

/// Iter over all idx from Vec::ZERO to the current vector
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VectorIter<T, const N : usize> where T : Integer
{
    cur : Vector<T,N>,
    end : Vector<T,N>,
}

impl<T, const N : usize> VectorIter<T, N> where T : Integer
{
    pub fn from_cur_to_end(cur : Vector<T,N>, end : Vector<T,N>) -> Self
    {
        Self { cur, end : if end.have_area_usize() { end } else { Vector::ZERO } }
    }

    pub fn new(end : Vector<T,N>) -> Self
    {
        Self::from_cur_to_end(Vector::ZERO, end)
    }
}

impl<T, const N : usize> Iterator for VectorIter<T, N> where T : Integer
{
    type Item=Vector<T,N>;

    fn next(&mut self) -> Option<Self::Item> 
    {
        let v = self.cur;
        let mut i = 0;
        
        while i < N
        {
            self.cur[i].increase();
            if self.cur[i] < self.end[i]
            {
                return Some(v);
            }
            self.cur[i] = T::ZERO;
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

    /* 
    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!()
        //let total = self.end.area_usize();
    }
    */
}
impl<T, const N : usize> std::iter::FusedIterator for VectorIter<T, N> where T : Integer {}
//impl<T, const N : usize> std::iter::ExactSizeIterator for VectorIter<T, N> where T : Integer  {}


impl<T,const N : usize> IterIndex<T,N> for Vector<T,N> where T : Integer
{
    type IterIndex = VectorIter<T,N>;
    
    fn iter_index(&self) -> Self::IterIndex {
        Self::IterIndex::new(*self)
    }
}