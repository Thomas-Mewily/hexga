use crate::*;


/// A mutable slice inside a [GridParam]
#[derive(Debug, Hash)]
pub struct GridParamViewMut<'a, T, Param, Idx, const N : usize> where Idx : IntegerIndex
{
    view  : GridViewMut<'a,T,Idx,N>,
    param : &'a mut Param,
}

impl<'a, T, Param, Idx, const N : usize> GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex 
{
    pub fn from_grid(grid : &'a mut GridParamBase<T,Param,Idx,N>) -> Self 
    { 
        let GridParamBase { grid, param } = grid;
        Self { view : grid.view_mut(), param } 
    }
    pub fn new(grid : &'a mut GridParamBase<T,Param,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        let GridParamBase { grid, param } = grid;
        let view = grid.rect().intersect_or_empty(view);
        Self { view : GridViewMut::new(grid, view), param }
    }
    pub const fn from_view(view : GridViewMut<'a,T,Idx,N>, param : &'a mut Param) -> Self 
    {
        Self { view, param }
    }

    /// Can't access the outside rectangle
    pub fn crop_margin<'b>(&'b mut self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> GridParamViewMut<'b, T, Param, Idx,N> where 'b: 'a 
    {
        GridParamViewMut::from_view(self.view.crop_margin(margin_start, margin_end), self.param)
    }
}

impl<'a, T, Param, Idx, const N : usize> IGridView<T,Param,Idx,N> for GridParamViewMut<'a, T, Param, Idx,N> 
    where Idx : IntegerIndex,
{
    type Map<Dest>=GridParamBase<Dest,Param,Idx,N>;
    fn map<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, Param : Clone { GridParamBase::from_grid_with_param(self.view.map(f), self.param.clone()) }
    fn map_par<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, Param : Clone { GridParamBase::from_grid_with_param(self.view.map_par(f), self.param.clone()) }

    fn subgrid(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone, Param : Clone { self.subview(rect).to_grid() }
    fn subgrid_par(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone+ Send + Sync, Idx : Sync, Param : Clone { self.subview(rect).to_grid_par() }

    type SubView<'b> = GridParamView<'b,T,Param,Idx,N> where Self: 'b;
    fn subview<'b>(&'b self, rect : Rectangle<Idx, N>) -> Self::SubView<'b> where T : Clone { Self::SubView::from_view(self.view.subview(rect), self.param) }
}

impl<'a,T,Param,Idx,const N : usize> CollectionGet<Vector<Idx,N>> for GridParamViewMut<'a,T,Param,Idx,N>
    where Idx : IntegerIndex
{
    type Output = <Self as Index<Vector<Idx,N>>>::Output;
    fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { self.view.get(pos) }
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { self.view.get_unchecked(pos) } }
}

impl<'a,T,Param,Idx,const N : usize> CollectionGetMut<Vector<Idx,N>> for GridParamViewMut<'a,T,Param,Idx,N>
    where Idx : IntegerIndex
{
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut T> { self.view.get_mut(pos) }
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut T { unsafe { self.view.get_unchecked_mut(pos) } }
}

impl<'a, T, Param, Idx, const N : usize> IRectangle<Idx,N> for GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex  
{
    fn begin(&self) -> Vector<Idx,N> { self.view.begin() }
    fn size (&self) -> Vector<Idx,N> { self.view.size()  }
}

impl<'a, T, Param, Idx, const N : usize> IGridViewMut<T,Param,Idx,N> for GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex,
{
    fn swap(&mut self, pos_a : Vector<Idx,N>, pos_b : Vector<Idx,N>) -> bool { self.view.swap(pos_a, pos_b) }
    
    type SubViewMut<'b> = GridParamViewMut<'b,T,Param,Idx,N> where Self: 'b;
    fn subview_mut<'b>(&'b mut self, rect : Rectangle<Idx, N>) -> Self::SubViewMut<'b> 
    { Self::SubViewMut::from_view(self.view.subview_mut(rect), self.param) }
}

impl<'a, T, Param, Idx, const N : usize> PartialEq for GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex ,
    T : PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() { return false; }
        self.size().iter_idx().all(|p| unsafe { self.get_unchecked(p) == other.get_unchecked(p) })
    }
}

impl<'a, T, Param, Idx, const N : usize> Eq for GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex , 
    T : Eq, 
{ }

impl<'a, T, Param, Idx, const N : usize> Index<Vector<Idx,N>> for GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex,
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get(index).unwrap() }
}

impl<'a, T, Param, Idx, const N : usize> IndexMut<Vector<Idx,N>> for GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex,
{
    fn index_mut(&mut self, index: Vector<Idx,N>) -> &mut Self::Output { self.get_mut(index).unwrap() }
}
