use super::*;


pub struct GridViewIter<'a, G, T, Idx, const N : usize> where G : IGrid<T,Idx,N>, Idx : Integer, T:'a
{
    grid : &'a G,
    rect : RectangleIter<Idx,N>,
    phantom : std::marker::PhantomData<T>,
}

impl<'a, G, T, Idx, const N : usize> GridViewIter<'a, G, T, Idx, N> where G : IGrid<T,Idx,N>, Idx : Integer, T:'a
{
    pub fn new(grid : &'a G) -> Self {
        Self {
            grid,
            rect : grid.rect().iter_index(),
            phantom : std::marker::PhantomData,
        }
    }

    pub fn from_rect(grid : &'a G, rect : Rectangle<Idx,N>) -> Option<Self>
    {
        if !grid.rect().is_rect_inside(rect) {
            return None;
        }else
        {
            Some(unsafe { Self::from_rect_unchecked(grid, rect) })
        }
    }

    pub unsafe fn from_rect_unchecked(grid : &'a G, rect : Rectangle<Idx,N>) -> Self 
    {
        Self {
            grid,
            rect : rect.iter_index(),
            phantom : std::marker::PhantomData,
        }
    }

    pub fn from_rect_intersect(grid : &'a G, rect : Rectangle<Idx,N>) -> Self 
    {
        Self {
            grid,
            rect : grid.rect().intersect_or_empty(rect).iter_index(),
            phantom : std::marker::PhantomData,
        }
    }
}

impl<'a, G, T, Idx, const N : usize> Clone for GridViewIter<'a, G, T, Idx, N> where G : IGrid<T, Idx, N>, Idx : Integer
{
    fn clone(&self) -> Self {
        Self {
            grid: self.grid,
            rect: self.rect.clone(),
            phantom: std::marker::PhantomData,
        }
    }
}
impl<'a, G, T, Idx, const N : usize> Iterator for GridViewIter<'a, G, T, Idx, N> where G : IGrid<T, Idx, N>, Idx : Integer
{
    type Item = (Vector<Idx,N>, &'a T);

    fn next(&mut self) -> Option<Self::Item> 
    {
        let idx = self.rect.next()?;
        Some((idx, unsafe { self.grid.get_unchecked(idx) }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.rect.size_hint()
    }
}
impl<'a, G, T, Idx, const N : usize> std::iter::FusedIterator for GridViewIter<'a, G, T, Idx, N> where G : IGrid<T, Idx, N>, Idx : Integer, RectangleIter<Idx,N> : std::iter::FusedIterator {}
impl<'a, G, T, Idx, const N : usize> std::iter::ExactSizeIterator for GridViewIter<'a, G, T, Idx, N> where G : IGrid<T, Idx, N>, Idx : Integer, RectangleIter<Idx,N> : std::iter::ExactSizeIterator {}