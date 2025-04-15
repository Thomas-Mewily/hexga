use crate::*;


/// A slice inside a [GridParam]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct GridParamView<'a, T, Param, Idx, const N : usize> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    view  : GridView<'a,T,Idx,N>,
    param : &'a Param,
}

impl<'a, T, Param, Idx, const N : usize> GridParamView<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
{
    pub fn from_grid(grid : &'a GridParamBase<T,Param,Idx,N>) -> Self 
    { 
        let GridParamBase { grid, param } = grid;
        Self { view : grid.view(), param } 
    }
    pub fn new(grid : &'a GridParamBase<T,Param,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        let GridParamBase { grid, param } = grid;
        let view = grid.rect().intersect_or_empty(view);
        Self { view : GridView::new(grid, view), param }
    }
    pub const fn from_view(view : GridView<'a,T,Idx,N>, param : &'a Param) -> Self 
    {
        Self { view, param }
    }

    /// Can't access the outside rectangle
    pub fn crop_margin(&self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> Self 
    {
        Self::from_view(self.view.crop_margin(margin_start, margin_end), self.param)
    }
}


impl<'a, T, Param, Idx, const N : usize> IGridView<T,Param,Idx,N> for GridParamView<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
{
    fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { self.view.get(pos) }
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { self.view.get_unchecked(pos) } }
    
    type ToGrid=GridParamBase<T,Param,Idx,N>;
    fn to_grid(self) -> Self::ToGrid where T : Clone, Param : Clone { Self::ToGrid::from_grid_with_param(self.view.to_grid(), self.param.clone()) }
    
    type SubView<'b> = GridParamView<'b, T, Param, Idx, N> where Self: 'b;
    fn subview<'b>(&'b self, rect : Rectangle<Idx, N>) -> Self::SubView<'b> where T : Clone
    { Self::SubView::from_view(self.view.subview(rect), self.param) }
        
    fn subgrid(&self, rect : Rectangle<Idx, N>) -> Self::ToGrid where T : Clone, Param : Clone, Self : Sized 
    { self.subview(rect).to_grid() }
}

impl<'a, T, Param, Idx, const N : usize> IRectangle<Idx,N> for GridParamView<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
{
    fn begin(&self) -> Vector<Idx,N> { self.view.begin() }
    fn size (&self) -> Vector<Idx,N> { self.view.size()  }
}

impl<'a, T, Param, Idx, const N : usize> Index<Vector<Idx,N>> for GridParamView<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.view.index(index) }
}