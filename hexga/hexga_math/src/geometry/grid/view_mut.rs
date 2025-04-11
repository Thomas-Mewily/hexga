use crate::*;

pub trait IGridViewMut<T, Idx, const N : usize> : IGridView<T,Idx,N> + IndexMut<Vector<Idx,N>,Output=T>
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut T>;
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut T { &mut self[pos] }

    fn subview_mut<'a>(&'a mut self, rect : Rectangle<Idx, N>) -> GridViewMut<'a,T,Idx,N>;

    fn swap(&mut self, pos_a : Vector<Idx,N>, pos_b : Vector<Idx,N>) -> bool;

    fn replace(&mut self, val : T, pos : Vector<Idx,N>) ->  Option<T> { self.get_mut(pos).map(|v| std::mem::replace(v, val)) }
    /// Do nothings if the index is outside the range
    fn set(&mut self, val : T, pos : Vector<Idx,N>) -> &mut Self { self.get_mut(pos).map(|v| *v = val); self }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item=(Vector<Idx,N>, &'a mut T)> where T: 'a
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
pub struct GridViewMut<'a, T, Idx,const N : usize> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    grid : &'a mut GridBase<T,Idx,N>,
    view : Rectangle<Idx,N>,
}

impl<'a, T, Idx, const N : usize> GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
{
    pub fn from_grid(grid : &'a mut GridBase<T,Idx,N>) -> Self 
    {
        let view = grid.rect();
        Self { grid, view }
    }
    pub fn new(grid : &'a mut GridBase<T,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        let view = grid.rect().intersect_or_empty(view);
        Self { grid, view }
    }
    pub unsafe fn new_unchecked(grid : &'a mut GridBase<T,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        Self { grid, view }
    }
    pub fn crop_margin<'b>(&'b mut self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> GridViewMut<'b, T, Idx,N> where 'b: 'a 
    {
        unsafe { Self::new_unchecked(self.grid, self.view.crop_margin(margin_start, margin_end)) }
    }
}

impl<'a, T, Idx, const N : usize> IGridView<T,Idx,N> for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
{
    fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { self.grid.get(self.view.pos + pos) }
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { self.grid.get_unchecked(pos) } }
    
    fn subview<'b>(&'b self, rect : Rectangle<Idx, N>) -> GridView<'b,T,Idx,N> { GridView::new(self.grid, self.view.intersect_or_empty(rect.moved_by(self.position()))) }
}

impl<'a, T, Idx, const N : usize> IRectangle<Idx,N> for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
{
    fn begin(&self) -> Vector<Idx,N> { self.view.begin() }
    fn size (&self) -> Vector<Idx,N> { self.view.size()  }
}

impl<'a, T, Idx, const N : usize> IGridViewMut<T,Idx,N> for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
{
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut T> { self.grid.get_mut(self.view.pos + pos) }
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut T { unsafe { self.grid.get_unchecked_mut(self.view.pos + pos) } }
    fn subview_mut<'b>(&'b mut self, rect : Rectangle<Idx, N>) -> GridViewMut<'b,T,Idx,N> { GridViewMut::new(self.grid, self.view.intersect_or_empty(rect.moved_by(self.position()))) }
    
    fn swap(&mut self, pos_a : Vector<Idx,N>, pos_b : Vector<Idx,N>) -> bool 
    {
        let offset = self.position();
        self.grid.swap(pos_a + offset, pos_b + offset)
    }
}

impl<'a, T, Idx, const N : usize> PartialEq for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    T : PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() { return false; }
        self.size().iter_idx().all(|p| unsafe { self.get_unchecked(p) == other.get_unchecked(p) })
    }
}

impl<'a, T, Idx, const N : usize> Eq for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, 
    T : Eq { }

impl<'a, T, Idx, const N : usize> Index<Vector<Idx,N>> for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get(index).unwrap() }
}

impl<'a, T, Idx, const N : usize> IndexMut<Vector<Idx,N>> for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
{
    fn index_mut(&mut self, index: Vector<Idx,N>) -> &mut Self::Output { self.get_mut(index).unwrap() }
}
