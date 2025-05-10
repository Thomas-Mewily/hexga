use crate::*;


/// A mutable slice inside a [GridParam]
#[derive(Debug, Hash)]
pub struct GridParamViewMut<'a, T, Param, Idx, const N : usize> where Idx : IntegerIndex
{
    view  : GridViewMut<'a,T,Idx,N>,
    param : &'a mut Param,
}

impl<'a, T, Param, Idx, const N : usize> Crop<Idx,N> for GridParamViewMut<'a, Param, T, Idx, N> where Idx : IntegerIndex
{
    fn crop(self, subrect : Rectangle<Idx, N>) -> Option<Self> 
    {
        self.view.crop(subrect).map(|r| Self::from_view_mut(r, self.param))
    }
    unsafe fn crop_unchecked(mut self, subrect : Rectangle<Idx, N>) -> Self 
    {
        self.view = unsafe { self.view.crop_unchecked(subrect) };
        self
    }
}

impl<'a, T, Param, Idx, const N : usize> GridParamViewMut<'a, T, Param, Idx, N> where Idx : IntegerIndex 
{
    pub fn from_grid(grid : &'a mut GridParamBase<T,Param,Idx,N>) -> Self 
    { 
        let GridParamBase { grid, param } = grid;
        Self { view : grid.view_mut(), param } 
    }
    pub fn new_intersect(grid : &'a mut GridParamBase<T,Param,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        Self { view : GridViewMut::new_intersect(&mut grid.grid, view), param : &mut grid.param }
    }
    pub const unsafe fn new_unchecked(grid : &'a mut GridParamBase<T,Param,Idx,N>, view : Rectangle<Idx,N>) -> Self 
    {
        Self { view: unsafe { GridViewMut::new_unchecked(&mut grid.grid, view) }, param: &mut grid.param }
    }
    pub const fn from_view_mut(view : GridViewMut<'a,T,Idx,N>, param : &'a mut Param) -> Self 
    {
        Self { view, param }
    }
}

impl<'a, T, Param, Idx, const N : usize> IGridView<T,Param,Idx,N> for GridParamViewMut<'a, T, Param, Idx,N> where Idx : IntegerIndex,
{
    type Map<Dest>=GridParamBase<Dest,Param,Idx,N>;
    fn map<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, Param : Clone { GridParamBase::from_grid_with_param(self.view.map(f), self.param.clone()) }
    fn map_par<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, Param : Clone { GridParamBase::from_grid_with_param(self.view.map_par(f), self.param.clone()) }
}

impl<'a,T,Param,Idx,const N : usize> Get<Vector<Idx,N>> for GridParamViewMut<'a,T,Param,Idx,N> where Idx : IntegerIndex
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

impl<'a,T,Param,Idx,const N : usize> GetMut<Vector<Idx,N>> for GridParamViewMut<'a,T,Param,Idx,N> where Idx : IntegerIndex
{
    #[inline(always)]
    fn try_get_mut(&mut self, pos : Vector<Idx,N>) -> Result<&mut Self::Output, ()> { self.view.try_get_mut(pos) }
    #[inline(always)]
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut Self::Output> { self.view.get_mut(pos) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut Self::Output { unsafe { self.view.get_unchecked_mut(pos) } }
}

impl<'a,T,Param,Idx,const N : usize> GetManyMut<Vector<Idx,N>> for GridParamViewMut<'a,T,Param,Idx,N> where Idx : IntegerIndex
{
    #[inline(always)]
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [Vector<Idx,N>; N2]) -> Result<[&mut Self::Output;N2], ()>  { self.view.try_get_many_mut(indices) }
}

impl<'a, T, Param, Idx, const N : usize> IRectangle<Idx,N> for GridParamViewMut<'a, T, Param, Idx, N> where Idx : IntegerIndex  
{
    fn begin(&self) -> Vector<Idx,N> { self.view.begin() }
    fn size (&self) -> Vector<Idx,N> { self.view.size()  }
}

impl<'a, T, Param, Idx, const N : usize> IGridViewMut<T,Param,Idx,N> for GridParamViewMut<'a, T, Param, Idx, N> where Idx : IntegerIndex,
{
    type SubViewMut<'b> = GridParamViewMut<'b,T,Param,Idx,N> where Self: 'b;
    fn subview_mut<'b>(&'b mut self, rect : Rectangle<Idx, N>) -> Self::SubViewMut<'b> 
    { Self::SubViewMut::from_view_mut(self.view.subview_mut(rect), self.param) }
}

impl<'a, T, Param, Idx, const N : usize> PartialEq for GridParamViewMut<'a, T, Param, Idx, N> where Idx : IntegerIndex ,
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

impl<'a, T, Param, Idx, const N : usize> Index<Vector<Idx,N>> for GridParamViewMut<'a, T, Param, Idx, N> where Idx : IntegerIndex,
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get_or_panic(index) }
}

impl<'a, T, Param, Idx, const N : usize> IndexMut<Vector<Idx,N>> for GridParamViewMut<'a, T, Param, Idx, N> where Idx : IntegerIndex,
{
    fn index_mut(&mut self, index: Vector<Idx,N>) -> &mut Self::Output { self.get_mut_or_panic(index) }
}
