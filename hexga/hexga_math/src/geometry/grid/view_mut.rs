use crate::*;

pub trait IGridViewMut<T, Param, Idx, const N : usize> : IGridView<T,Param,Idx,N> + GetIndexMut<Vector<Idx,N>,Output=T>
    where Idx : IntegerIndex
{
    type SubViewMut<'b> where Self: 'b;
    fn subview_mut<'a>(&'a mut self, rect : Rectangle<Idx, N>) -> Self::SubViewMut<'a>;

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
#[derive(Debug, Hash)]
pub struct GridViewMut<'a, T, Idx,const N : usize> where Idx : IntegerIndex
{
    grid : &'a mut GridBase<T,Idx,N>,
    view : Rectangle<Idx,N>,
}

impl<'a, T, Idx, const N : usize> GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex 
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
    pub const unsafe fn new_unchecked(grid : &'a mut GridBase<T,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        Self { grid, view }
    }
    pub fn crop_margin<'b>(&'b mut self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> GridViewMut<'b, T, Idx,N> where 'b: 'a 
    {
        unsafe { Self::new_unchecked(self.grid, self.view.crop_margin(margin_start, margin_end)) }
    }
}

impl<'a, T, Idx, const N : usize> IGridView<T,(),Idx,N> for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex 
{
    type Map<Dest>=GridBase<Dest,Idx,N>;
    fn map<Dest, F>(&self, mut f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, () : Clone { GridBase::from_fn(self.size(), |p| f(&self[p])) }
    fn map_par<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, () : Clone  { GridBase::from_fn_par(self.size(), |p| f(&self[p])) }

    fn subgrid(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone { self.subview(rect).to_grid() }
    fn subgrid_par(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone+ Send + Sync, Idx : Sync { self.subview(rect).to_grid_par() }
    
    type SubView<'b> = GridView<'b,T,Idx,N> where Self: 'b;
    fn subview<'b>(&'b self, rect : Rectangle<Idx, N>) -> Self::SubView<'b> where T : Clone { GridView::new(self.grid, self.view.intersect_or_empty(rect.moved_by(self.position()))) }
}

impl<'a, T, Idx, const N : usize> GetIndex<Vector<Idx,N>> for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex 
{
    fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { self.grid.get(self.view.pos + pos) }
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { self.grid.get_unchecked(pos) } }
}
impl<'a, T, Idx, const N : usize> GetIndexMut<Vector<Idx,N>> for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex 
{
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut T> { self.grid.get_mut(self.view.pos + pos) }
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut T { unsafe { self.grid.get_unchecked_mut(self.view.pos + pos) } }
}

impl<'a, T, Idx, const N : usize> IRectangle<Idx,N> for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex 
{
    fn begin(&self) -> Vector<Idx,N> { self.view.begin() }
    fn size (&self) -> Vector<Idx,N> { self.view.size()  }
}

impl<'a, T, Idx, const N : usize> IGridViewMut<T,(),Idx,N> for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex 
{
    fn swap(&mut self, pos_a : Vector<Idx,N>, pos_b : Vector<Idx,N>) -> bool 
    {
        let offset = self.position();
        self.grid.swap(pos_a + offset, pos_b + offset)
    }
    
    type SubViewMut<'b> = GridViewMut<'b,T,Idx,N> where Self: 'b;
    fn subview_mut<'b>(&'b mut self, rect : Rectangle<Idx, N>) -> GridViewMut<'b,T,Idx,N> { GridViewMut::new(self.grid, self.view.intersect_or_empty(rect.moved_by(self.position()))) }
}

impl<'a, T, Idx, const N : usize> PartialEq for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex,
    T : PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() { return false; }
        self.size().iter_idx().all(|p| unsafe { self.get_unchecked(p) == other.get_unchecked(p) })
    }
}

impl<'a, T, Idx, const N : usize> Eq for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex, 
    T : Eq { }

impl<'a, T, Idx, const N : usize> Index<Vector<Idx,N>> for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex 
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get(index).unwrap() }
}

impl<'a, T, Idx, const N : usize> IndexMut<Vector<Idx,N>> for GridViewMut<'a, T, Idx,N> 
    where Idx : IntegerIndex 
{
    fn index_mut(&mut self, index: Vector<Idx,N>) -> &mut Self::Output { self.get_mut(index).unwrap() }
}
