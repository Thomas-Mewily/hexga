use crate::*;


/// A slice inside a [GridParam]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct GridParamView<'a, T, Param, Idx, const N : usize> where Idx : IntegerIndex
{
    view  : GridView<'a,T,Idx,N>,
    param : &'a Param,
}

impl<'a, T, Param, Idx, const N : usize> GridParamView<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex 
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
    where Idx : IntegerIndex,
{
    type Map<Dest>=GridParamBase<Dest,Param,Idx,N>;
    fn map<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, Param : Clone { GridParamBase::from_grid_with_param(self.view.map(f), self.param.clone()) }
    fn map_par<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, Param : Clone { GridParamBase::from_grid_with_param(self.view.map_par(f), self.param.clone()) }

    fn subgrid(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone, Param : Clone { self.subview(rect).to_grid() }
    fn subgrid_par(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone + Send + Sync, Idx : Sync, Param : Clone  { self.subview(rect).to_grid_par() }

    type SubView<'b> = GridParamView<'b, T, Param, Idx, N> where Self: 'b;
    fn subview<'b>(&'b self, rect : Rectangle<Idx, N>) -> Self::SubView<'b> where T : Clone
    { Self::SubView::from_view(self.view.subview(rect), self.param) }
}

impl<'a,T,Param,Idx,const N : usize> LookUp<Vector<Idx,N>> for GridParamView<'a,T,Param,Idx,N>
    where Idx : IntegerIndex
{
    type LookUpOutput = <Self as Index<Vector<Idx,N>>>::Output;
    fn lookup(&self, k: Vector<Idx,N>) -> Option<&Self::LookUpOutput> { self.get(k) }
}
impl<'a,T,Param,Idx,const N : usize> GetIndex<Vector<Idx,N>> for GridParamView<'a,T,Param,Idx,N>
    where Idx : IntegerIndex
{
    fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { self.view.get(pos) }
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { self.view.get_unchecked(pos) } }
}

impl<'a, T, Param, Idx, const N : usize> IRectangle<Idx,N> for GridParamView<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex 
{
    fn begin(&self) -> Vector<Idx,N> { self.view.begin() }
    fn size (&self) -> Vector<Idx,N> { self.view.size()  }
}

impl<'a, T, Param, Idx, const N : usize> Index<Vector<Idx,N>> for GridParamView<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex 
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.view.index(index) }
}