use crate::*;


/// A mutable slice inside a [Grid]
#[derive(Debug, Hash)]
pub struct GridParamViewMut<'a, T, Param, Idx, const N : usize> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>

{
    view  : GridViewMut<'a,T,Idx,N>,
    param : &'a mut Param,
}

impl<'a, T, Param, Idx, const N : usize> GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
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

impl<'a, T, Param, Idx, const N : usize> IGridView<T,Idx,N> for GridParamViewMut<'a, T, Param, Idx,N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { self.view.get(pos) }
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { self.view.get_unchecked(pos) } }
    
    type ToGrid=GridParamBase<T,Param,Idx,N>;
    fn to_grid(self) -> Self::ToGrid where T : Clone { GridParamBase::from_grid_with_param(self.view.to_grid(), self.param.clone()) }
    
    type SubView<'b> = GridParamView<'b,T,Param,Idx,N> where Self: 'b;
    fn subview<'b>(&'b self, rect : Rectangle<Idx, N>) -> Self::SubView<'b> where T : Clone { Self::SubView::from_view(self.view.subview(rect), self.param) }
    fn subgrid(&self, rect : Rectangle<Idx, N>) -> Self::ToGrid where T : Clone, Self : Sized { self.subview(rect).to_grid() }
}

impl<'a, T, Param, Idx, const N : usize> IRectangle<Idx,N> for GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>  
{
    fn begin(&self) -> Vector<Idx,N> { self.view.begin() }
    fn size (&self) -> Vector<Idx,N> { self.view.size()  }
}

impl<'a, T, Param, Idx, const N : usize> IGridViewMut<T,Idx,N> for GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut T> { self.view.get_mut(pos) }
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut T { unsafe { self.view.get_unchecked_mut(pos) } }
    
    fn swap(&mut self, pos_a : Vector<Idx,N>, pos_b : Vector<Idx,N>) -> bool { self.view.swap(pos_a, pos_b) }
    
    type SubViewMut<'b> = GridParamViewMut<'b,T,Param,Idx,N> where Self: 'b;
    fn subview_mut<'b>(&'b mut self, rect : Rectangle<Idx, N>) -> Self::SubViewMut<'b> 
    { Self::SubViewMut::from_view(self.view.subview_mut(rect), self.param) }
}

impl<'a, T, Param, Idx, const N : usize> PartialEq for GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> ,
    T : PartialEq,
    Param : Clone
{
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() { return false; }
        self.size().iter_idx().all(|p| unsafe { self.get_unchecked(p) == other.get_unchecked(p) })
    }
}

impl<'a, T, Param, Idx, const N : usize> Eq for GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> , 
    T : Eq, 
    Param : Clone
{ }

impl<'a, T, Param, Idx, const N : usize> Index<Vector<Idx,N>> for GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get(index).unwrap() }
}

impl<'a, T, Param, Idx, const N : usize> IndexMut<Vector<Idx,N>> for GridParamViewMut<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    fn index_mut(&mut self, index: Vector<Idx,N>) -> &mut Self::Output { self.get_mut(index).unwrap() }
}
