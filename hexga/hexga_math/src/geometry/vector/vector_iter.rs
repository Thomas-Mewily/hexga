use crate::*;


pub trait IterIdx<T, const N : usize>
{
    type IterIdx : Iterator<Item=Vector<T,N>>;

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
    /// for (i, expected) in point2(2, 3).iter_idx().zip(expected_points.iter())
    /// {
    ///     assert_eq!(&i, expected);
    /// }
    /// ```
    fn iter_idx(&self) -> Self::IterIdx;
}

/// Iter over all idx from Vec::ZERO to the current vector
pub struct VectorIter<T, const N : usize> where T : Number
{
    pub cur : Vector<T,N>,
    pub end : Vector<T,N>,
}

impl<T, const N : usize> VectorIter<T, N> where T : Number
{
    pub fn from_cur_to_end(cur : Vector<T,N>, end : Vector<T,N>) -> Self
    {
        Self { cur, end : if end.have_area() { end } else { Vector::ZERO } }
    }

    pub fn new(end : Vector<T,N>) -> Self
    {
        Self::from_cur_to_end(Vector::ZERO, end)
    }
}

impl<T, const N : usize> Iterator for VectorIter<T, N> where T : Number
{
    type Item=Vector<T,N>;

    fn next(&mut self) -> Option<Self::Item> 
    {
        if self.cur.array() == self.end.array() { return None; }

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
        Some(v)
    }
}

impl<T,const N : usize> IterIdx<T,N> for Vector<T,N> where T : Integer
{
    type IterIdx = VectorIter<T,N>;
    
    fn iter_idx(&self) -> Self::IterIdx {
        Self::IterIdx::new(*self)
    }
}