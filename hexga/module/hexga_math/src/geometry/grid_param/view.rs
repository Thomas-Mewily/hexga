use crate::*;


/// A slice inside a [GridParam]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct GridParamView<'a, T, Param, Idx, const N : usize> where Idx : Integer
{
    view  : GridView<'a,T,Idx,N>,
    param : &'a Param,
}

impl<'a, T, Param, Idx, const N : usize> Crop<Idx,N> for GridParamView<'a, Param, T, Idx, N> where Idx : Integer
{
    fn crop(self, subrect : Rectangle<Idx, N>) -> Option<Self> 
    {
        self.view.crop(subrect).map(|r| Self::from_view(r, self.param))
    }
    unsafe fn crop_unchecked(mut self, subrect : Rectangle<Idx, N>) -> Self 
    {
        self.view = unsafe { self.view.crop_unchecked(subrect) };
        self
    }
}

impl<'a, T, Param, Idx, const N : usize> GridParamView<'a, T, Param, Idx, N> 
    where Idx : Integer 
{
    pub fn from_grid(grid : &'a GridParamBase<T,Param,Idx,N>) -> Self 
    { 
        let GridParamBase { grid, param } = grid;
        Self { view : grid.view(), param } 
    }
    pub fn new_intersect(grid : &'a GridParamBase<T,Param,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        Self { view : GridView::new_intersect(&grid.grid, view), param : &grid.param }
    }
    pub const unsafe fn new_unchecked(grid : &'a GridParamBase<T,Param,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        Self { view: unsafe { GridView::new_unchecked(&grid.grid, view) }, param: &grid.param }
    }
    pub const fn from_view(view : GridView<'a,T,Idx,N>, param : &'a Param) -> Self 
    {
        Self { view, param }
    }
}


impl<'a, T, Param, Idx, const N : usize> IGridView<T,Param,Idx,N> for GridParamView<'a, T, Param, Idx, N> 
    where Idx : Integer,
{
    type Map<Dest>=GridParamBase<Dest,Param,Idx,N>;
    fn map<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, Param : Clone { GridParamBase::from_grid_with_param(self.view.map(f), self.param.clone()) }
    fn map_par<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, Param : Clone { GridParamBase::from_grid_with_param(self.view.map_par(f), self.param.clone()) }
}

impl<'a,T,Param,Idx,const N : usize> Get<Vector<Idx,N>> for GridParamView<'a,T,Param,Idx,N>
    where Idx : Integer
{
    type Output = <Self as Index<Vector<Idx,N>>>::Output;
    #[inline(always)]
    fn try_get(&self, pos : Vector<Idx,N>) -> Result<&Self::Output, ()> { self.view.try_get(pos) }
    #[inline(always)]
    fn get(&self, pos : Vector<Idx,N>) -> Option<&Self::Output> { self.view.get(pos) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &Self::Output { unsafe { self.view.get_unchecked(pos) } }
}

impl<'a, T, Param, Idx, const N : usize> IRectangle<Idx,N> for GridParamView<'a, T, Param, Idx, N> 
    where Idx : Integer 
{
    #[inline(always)]
    fn begin(&self) -> Vector<Idx,N> { self.view.begin() }
    #[inline(always)]
    fn size (&self) -> Vector<Idx,N> { self.view.size()  }
}

impl<'a, T, Param, Idx, const N : usize> Index<Vector<Idx,N>> for GridParamView<'a, T, Param, Idx, N> 
    where Idx : Integer 
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.view.index(index) }
}