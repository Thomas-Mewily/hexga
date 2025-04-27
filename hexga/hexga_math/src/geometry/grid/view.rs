use std::iter::Map;

use crate::*;

/// A slice view to a [Grid]
/// 
/// [Param] is a silent parameter, generally void, that is here to facilitate the API for [GridParam] because some function depend 
/// if the param is clonable or not. 
pub trait IGridView<T, Param, Idx, const N : usize> : CollectionGet<Vector<Idx,N>,Output = T> + IRectangle<Idx, N>
    where Idx : IntegerIndex
{
    /* 
    // Can't be easily optimized (they will call `self.get(pos)`), so it is better to omit them
    fn get_index(&self, index : usize) -> Option<&T>;
    fn is_index_inside(&self, index : usize) -> bool { index < self.area().to_usize()  }
    fn is_index_outside(&self, index : usize) -> bool { !self.is_index_inside(index) }
    */

    type Map<Dest>;
    fn map<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, Param : Clone;
    fn map_par<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, Param : Clone;

    fn to_grid(&self) -> Self::Map<T> where T : Clone, Param : Clone { self.map(|v| v.clone() )}
    fn to_grid_par(&self) -> Self::Map<T> where T : Clone + Send + Sync, Idx : Sync, Param : Clone { self.map_par(|v| v.clone() )}

    fn subgrid(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone, Param : Clone;
    fn subgrid_par(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone + Send + Sync, Idx : Sync, Param : Clone;

    type SubView<'b> : IGridView<T,Param,Idx,N> where Self: 'b;
    fn subview<'b>(&'b self, rect : Rectangle<Idx, N>) -> Self::SubView<'b> where T : Clone;

    fn iter<'a>(&'a self) -> impl Iterator<Item=(Vector<Idx,N>, &'a T)> where T: 'a
    {
        let r = self.rect(); 
        r.iter_idx().map(|p| (p, unsafe { self.get_unchecked(p) }))
    }
}


impl<'a, T, Idx, const N : usize> CollectionGet<Vector<Idx,N>> for GridView<'a, T, Idx,N> 
    where Idx : IntegerIndex 
{
    type Output = <Self as Index<Vector<Idx,N>>>::Output;
    fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { self.grid.get(self.view.pos + pos) }
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { self.grid.get_unchecked(self.view.pos + pos) } }
}

/// A slice inside a [Grid]
#[derive(Clone, Debug, Copy, Hash)]
pub struct GridView<'a, T, Idx, const N : usize> where Idx : IntegerIndex
{
    grid : &'a GridBase<T,Idx,N>,
    view : Rectangle<Idx,N>,
}


impl<'a, T, Idx, const N : usize> PartialEq for GridView<'a, T, Idx, N> 
    where Idx : IntegerIndex,
    T : PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() { return false; }
        self.size().iter_idx().all(|p| unsafe { self.get_unchecked(p) == other.get_unchecked(p) })
    }
}

impl<'a, T, Idx, const N : usize> Eq for GridView<'a, T, Idx, N> 
    where Idx : IntegerIndex, 
    T : Eq { }

impl<'a, T, Idx, const N : usize> GridView<'a, T, Idx, N> 
    where Idx : IntegerIndex 
{
    pub fn from_grid(grid : &'a GridBase<T,Idx,N>) -> Self 
    {
        Self { grid, view: grid.rect() }
    }
    pub fn new(grid : &'a GridBase<T,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        let view = grid.rect().intersect_or_empty(view);
        Self { grid, view }
    }
    pub const unsafe fn new_unchecked(grid : &'a GridBase<T,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        Self { grid, view }
    }

    /// Can't access the outside rectangle
    pub fn crop_margin(&self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> Self 
    {
        unsafe { Self::new_unchecked(self.grid, self.view.crop_margin(margin_start, margin_end)) }
    }

    pub fn format(self) -> GridViewFormat<'a,T,Idx,N> { GridViewFormat::new(self) }
}


impl<'a, T, Idx, const N : usize> IGridView<T,(),Idx,N> for GridView<'a, T, Idx, N> 
    where Idx : IntegerIndex 
{
    type Map<Dest>=GridBase<Dest,Idx,N>;
    fn map<Dest, F>(&self, mut f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, () : Clone { GridBase::from_fn(self.size(), |p| f(&self[p])) }
    fn map_par<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, () : Clone  { GridBase::from_fn_par(self.size(), |p| f(&self[p])) }

    fn subgrid(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone { self.subview(rect).to_grid() }
    fn subgrid_par(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone + Send + Sync, Idx : Sync { self.subview(rect).to_grid_par() }

    type SubView<'b> = GridView<'b,T,Idx,N> where Self: 'b;
    fn subview<'b>(&'b self, rect : Rectangle<Idx, N>) -> Self::SubView<'b> where T : Clone { GridView::new(self.grid, self.view.intersect_or_empty(rect.moved_by(self.position()))) }
}

impl<'a, T, Idx, const N : usize> IRectangle<Idx,N> for GridView<'a, T, Idx, N> 
    where Idx : IntegerIndex 
{
    fn begin(&self) -> Vector<Idx,N> { self.view.begin() }
    fn size (&self) -> Vector<Idx,N> { self.view.size()  }
}

impl<'a, T, Idx, const N : usize> Index<Vector<Idx,N>> for GridView<'a, T, Idx, N> 
    where Idx : IntegerIndex 
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get(index).unwrap() }
}


