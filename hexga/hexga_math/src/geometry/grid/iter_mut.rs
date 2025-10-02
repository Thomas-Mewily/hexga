use super::*;

pub struct GridViewIterMut<'a, G, T, Idx, const N : usize> where G : IGrid<T,Idx,N>, Idx : Integer, T:'a
{
    grid : &'a mut G,
    rect : RectangleIter<Idx,N>,
    phantom : std::marker::PhantomData<T>,
}

impl<'a, G, T, Idx, const N : usize> GridViewIterMut<'a, G, T, Idx, N> where G : IGrid<T,Idx,N>, Idx : Integer, T:'a
{
    pub fn new(grid : &'a mut G) -> Self
    {
        let rect = grid.rect().iter_index();
        Self {
            grid,
            rect : rect,
            phantom : std::marker::PhantomData,
        }
    }

    pub fn from_rect(grid : &'a mut G, rect : Rectangle<Idx,N>) -> Option<Self>
    {
        if !grid.rect().is_rect_inside(rect) {
            return None;
        }else
        {
            Some(unsafe { Self::from_rect_unchecked(grid, rect) })
        }
    }

    pub unsafe fn from_rect_unchecked(grid : &'a mut G, rect : Rectangle<Idx,N>) -> Self
    {
        Self {
            grid,
            rect : rect.iter_index(),
            phantom : std::marker::PhantomData,
        }
    }

    pub fn from_rect_intersect(grid : &'a mut G, rect : Rectangle<Idx,N>) -> Self
    {
        let rect = grid.rect().intersect_or_empty(rect).iter_index();
        Self {
            grid,
            rect,
            phantom : std::marker::PhantomData,
        }
    }
}


impl<'a, G, T, Idx, const N : usize> Iterator for GridViewIterMut<'a, G, T, Idx, N> where G : IGrid<T, Idx, N>, Idx : Integer
{
    type Item = (Vector<Idx,N>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item>
    {
        let idx = self.rect.next()?;
        // # Safety
        // Each index are different
        Some((idx, unsafe { (&mut *(&mut *self.grid as *mut G)).get_unchecked_mut(idx) }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.rect.size_hint()
    }
}
impl<'a, G, T, Idx, const N : usize> std::iter::FusedIterator for GridViewIterMut<'a, G, T, Idx, N> where G : IGrid<T, Idx, N>, Idx : Integer, RectangleIter<Idx,N> : std::iter::FusedIterator {}
impl<'a, G, T, Idx, const N : usize> std::iter::ExactSizeIterator for GridViewIterMut<'a, G, T, Idx, N> where G : IGrid<T, Idx, N>, Idx : Integer, RectangleIter<Idx,N> : std::iter::ExactSizeIterator {}

impl<'a, G, T, Idx, const N : usize> Map for GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    T: Clone,
{
    type Item=T;

    fn map_intern<F>(mut self, mut f: F) -> Self where F: FnMut(Self::Item) -> Self::Item
    {
        self.iter_mut().for_each(|(_,v)|
        {
            unsafe
            {
                // Temporary moving the reference
                let old = std::ptr::read(v);
                let new = f(old);
                std::ptr::write(v, new);
            }
        });
        self
    }

    fn map_with_intern<F>(mut self, mut other: Self, mut f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item {
        assert_eq!(self.size(), other.size(), "size mismatch");
        self.iter_mut().zip(other.iter_mut()).for_each(|((_, a),(_, b))|
        {
            unsafe
            {
                // Temporary moving the reference
                let old = std::ptr::read(a);
                let new = f(old, b.clone());
                std::ptr::write(a, new);
            }
        });
        self
    }
}