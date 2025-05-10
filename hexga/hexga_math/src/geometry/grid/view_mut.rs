use crate::*;

pub trait IGridViewMut<T, Param, Idx, const N : usize> : IGridView<T,Param,Idx,N> + GetMut<Vector<Idx,N>,Output=T>
    where Idx : IntegerIndex
{
    type SubViewMut<'b> where Self: 'b;
    fn subview_mut<'a>(&'a mut self, rect : Rectangle<Idx, N>) -> Self::SubViewMut<'a>;

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

impl<'a, T, Idx, const N : usize> GridViewMut<'a, T, Idx,N> where Idx : IntegerIndex 
{
    pub fn from_grid(grid : &'a mut GridBase<T,Idx,N>) -> Self 
    {
        let view = grid.rect();
        Self { grid, view }
    }
    pub fn new_intersect(grid : &'a mut GridBase<T,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        let view = grid.rect().intersect_or_empty(view);
        Self { grid, view }
    }
    pub const unsafe fn new_unchecked(grid : &'a mut GridBase<T,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        Self { grid, view }
    }
}

impl<'a, T, Idx, const N : usize> Crop<Idx,N> for GridViewMut<'a, T, Idx, N> where Idx : IntegerIndex
{
    fn crop(self, subrect : Rectangle<Idx, N>) -> Option<Self> 
    {
        self.view.crop(subrect).map(|r| unsafe { Self::new_unchecked(self.grid, r) })
    }
    unsafe fn crop_unchecked(mut self, subrect : Rectangle<Idx, N>) -> Self 
    {
        self.view = unsafe { self.view.crop_unchecked(subrect) };
        self
    }
}

impl<'a, T, Idx, const N : usize> IGridView<T,(),Idx,N> for GridViewMut<'a, T, Idx,N> where Idx : IntegerIndex 
{
    type Map<Dest>=GridBase<Dest,Idx,N>;
    fn map<Dest, F>(&self, mut f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, () : Clone { GridBase::from_fn(self.size(), |p| f(&self[p])) }
    fn map_par<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, () : Clone  { GridBase::from_fn_par(self.size(), |p| f(&self[p])) }

    /*
    fn subgrid(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone { self.crop_intersect(rect).to_grid() }
    fn subgrid_par(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone+ Send + Sync, Idx : Sync { self.crop_intersect(rect).to_grid_par() }
    
    type SubView<'b> = GridView<'b,T,Idx,N> where Self: 'b;
    fn crop_intersect<'b>(&'b self, rect : Rectangle<Idx, N>) -> Self::SubView<'b> where T : Clone { GridView::new(self.grid, self.view.intersect_or_empty(rect.moved_by(self.position()))) }
    */
}

impl<'a, T, Idx, const N : usize> Get<Vector<Idx,N>> for GridViewMut<'a, T, Idx,N> where Idx : IntegerIndex 
{
    type Output = <Self as Index<Vector<Idx,N>>>::Output;
    #[inline(always)]
    fn try_get(&self, pos : Vector<Idx,N>) -> Result<&Self::Output, ()> { self.get(pos).ok_or_void() }

    #[inline(always)]
    fn get(&self, pos : Vector<Idx,N>) -> Option<&Self::Output> 
    { 
        let idx = self.view.pos + pos;
        if self.rect().is_inside(idx) { self.grid.get(idx) } else { None } 
    }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &Self::Output { unsafe { self.grid.get_unchecked(self.view.pos + pos) } }
}

impl<'a, T, Idx, const N : usize> GetMut<Vector<Idx,N>> for GridViewMut<'a, T, Idx,N> where Idx : IntegerIndex 
{
    #[inline(always)]
    fn try_get_mut(&mut self, pos : Vector<Idx,N>) -> Result<&mut Self::Output, ()> { self.get_mut(pos).ok_or_void() }

    #[inline(always)]
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut Self::Output> 
    { 
        let idx = self.view.pos + pos;
        if self.rect().is_inside(idx) { self.grid.get_mut(idx) } else { None } 
    }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut Self::Output { unsafe { self.grid.get_unchecked_mut(self.view.pos + pos) } }
}

impl<'a, T, Idx, const N : usize> GetManyMut<Vector<Idx,N>> for GridViewMut<'a, T, Idx,N> where Idx : IntegerIndex 
{
    #[inline(always)]
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [Vector<Idx,N>; N2]) -> Result<[&mut Self::Output;N2], ()> 
    {
        let r = self.rect();
        if indices.any(|i| r.is_outside(*i)) { return Err(()); }
        self.grid.try_get_many_mut(indices.map(|pos| self.view.pos + pos))
    }
}

impl<'a, T, Idx, const N : usize> IRectangle<Idx,N> for GridViewMut<'a, T, Idx,N> where Idx : IntegerIndex 
{
    #[inline(always)]
    fn begin(&self) -> Vector<Idx,N> { self.view.begin() }
    #[inline(always)]
    fn size (&self) -> Vector<Idx,N> { self.view.size()  }
}

impl<'a, T, Idx, const N : usize> IGridViewMut<T,(),Idx,N> for GridViewMut<'a, T, Idx,N> where Idx : IntegerIndex 
{
    type SubViewMut<'b> = GridViewMut<'b,T,Idx,N> where Self: 'b;
    fn subview_mut<'b>(&'b mut self, rect : Rectangle<Idx, N>) -> GridViewMut<'b,T,Idx,N> { GridViewMut::new_intersect(self.grid, self.view.intersect_or_empty(rect.moved_by(self.position()))) }
}

impl<'a, T, Idx, const N : usize> PartialEq for GridViewMut<'a, T, Idx,N> where Idx : IntegerIndex,
    T : PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() { return false; }
        self.size().iter_idx().all(|p| unsafe { self.get_unchecked(p) == other.get_unchecked(p) })
    }
}

impl<'a, T, Idx, const N : usize> Eq for GridViewMut<'a, T, Idx,N> where Idx : IntegerIndex, T : Eq { }

impl<'a, T, Idx, const N : usize> Index<Vector<Idx,N>> for GridViewMut<'a, T, Idx,N> where Idx : IntegerIndex 
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get_or_panic(index) }
}

impl<'a, T, Idx, const N : usize> IndexMut<Vector<Idx,N>> for GridViewMut<'a, T, Idx,N> where Idx : IntegerIndex 
{
    fn index_mut(&mut self, index: Vector<Idx,N>) -> &mut Self::Output { self.get_mut_or_panic(index) }
}
