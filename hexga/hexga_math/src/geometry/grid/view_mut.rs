use crate::*;

pub trait IGridViewMut<T, const N : usize, I> : IGridView<T,N,I> + IndexMut<Vector<I,N>,Output=T>
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    fn get_mut(&mut self, pos : Vector<I,N>) -> Option<&mut T>;
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<I,N>) -> &mut T { &mut self[pos] }

    fn subview_mut<'a>(&'a mut self, rect : Rectangle<I, N>) -> GridViewMut<'a,T,N,I>;

    fn swap(&mut self, pos_a : Vector<I,N>, pos_b : Vector<I,N>) -> bool;

    fn replace(&mut self, val : T, pos : Vector<I,N>) ->  Option<T> { self.get_mut(pos).map(|v| std::mem::replace(v, val)) }
    /// Do nothings if the index is outside the range
    fn set(&mut self, val : T, pos : Vector<I,N>) -> &mut Self { self.get_mut(pos).map(|v| *v = val); self }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item=(Vector<I,N>, &'a mut T)> where T: 'a
    {
        let r = self.rect();

        // SAFELY get a raw pointer to the underlying storage
        let ptr = self as *mut Self;
    
        r.iter_idx().map(move |p| {
            // SAFETY : each position is unique, so the same tiles can't be returned twice or more,
            // so each element only have one mutable borrow at a time
            let t = unsafe { (*ptr).get_unchecked_mut(p) };
            (p, t)
        })
    }
}


/// A mutable slice inside a [Grid]
#[derive(Debug)]
pub struct GridViewMut<'a, T, const N : usize, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    grid : &'a mut GridBase<T,N,I>,
    view : Rectangle<I,N>,
}

impl<'a, T, const N : usize, I> GridViewMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    pub fn from_grid(grid : &'a mut GridBase<T,N,I>) -> Self 
    {
        let view = grid.rect();
        Self { grid, view }
    }
    pub fn new(grid : &'a mut GridBase<T,N,I>, view : Rectangle<I,N>) -> Self 
    {
        let view = grid.rect().intersect_or_empty(view);
        Self { grid, view }
    }
    pub unsafe fn new_unchecked(grid : &'a mut GridBase<T,N,I>, view : Rectangle<I,N>) -> Self 
    {
        Self { grid, view }
    }
    pub fn crop_margin<'b>(&'b mut self, margin_start : Vector<I,N>, margin_end : Vector<I,N>) -> GridViewMut<'b, T, N, I> where 'b: 'a 
    {
        unsafe { Self::new_unchecked(self.grid, self.view.crop_margin(margin_start, margin_end)) }
    }
}

impl<'a, T, const N : usize, I> IGridView<T,N,I> for GridViewMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn get(&self, pos : Vector<I,N>) -> Option<&T> { self.grid.get(self.view.pos + pos) }
    unsafe fn get_unchecked(&self, pos : Vector<I,N>) -> &T { unsafe { self.grid.get_unchecked(pos) } }
    
    fn subview<'b>(&'b self, rect : Rectangle<I, N>) -> GridView<'b,T,N,I> { GridView::new(self.grid, self.view.intersect_or_empty(rect.moved_by(self.position()))) }
}

impl<'a, T, const N : usize, I> IRectangle<I,N> for GridViewMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn begin(&self) -> Vector<I,N> { self.view.begin() }
    fn size (&self) -> Vector<I,N> { self.view.size()  }
}

impl<'a, T, const N : usize, I> IGridViewMut<T,N,I> for GridViewMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn get_mut(&mut self, pos : Vector<I,N>) -> Option<&mut T> { self.grid.get_mut(self.view.pos + pos) }
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<I,N>) -> &mut T { unsafe { self.grid.get_unchecked_mut(self.view.pos + pos) } }
    fn subview_mut<'b>(&'b mut self, rect : Rectangle<I, N>) -> GridViewMut<'b,T,N,I> { GridViewMut::new(self.grid, self.view.intersect_or_empty(rect.moved_by(self.position()))) }
    
    fn swap(&mut self, pos_a : Vector<I,N>, pos_b : Vector<I,N>) -> bool 
    {
        let offset = self.position();
        self.grid.swap(pos_a + offset, pos_b + offset)
    }
}

impl<'a, T, const N : usize, I> PartialEq for GridViewMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>,
    T : PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() { return false; }
        self.size().iter_idx().all(|p| unsafe { self.get_unchecked(p) == other.get_unchecked(p) })
    }
}

impl<'a, T, const N : usize, I> Eq for GridViewMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>, 
    T : Eq { }

impl<'a, T, const N : usize, I> Index<Vector<I,N>> for GridViewMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    type Output=T;
    fn index(&self, index: Vector<I,N>) -> &Self::Output { self.get(index).unwrap() }
}

impl<'a, T, const N : usize, I> IndexMut<Vector<I,N>> for GridViewMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn index_mut(&mut self, index: Vector<I,N>) -> &mut Self::Output { self.get_mut(index).unwrap() }
}
