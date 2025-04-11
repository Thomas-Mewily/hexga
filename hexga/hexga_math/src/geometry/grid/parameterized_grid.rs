use std::iter::Map;

use crate::*;

/* 
/// ParameterizedGrid
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct GridParamBase<T, Param, Idx, const N : usize> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    pub grid    : GridBase<T,Idx, N>,
    pub param   : Param,
}

impl<T,Param,Idx,const N : usize> GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    pub fn new_with_param(grid : GridBase<T,Idx, N>, param : Param) -> Self { Self { grid, param }}
    pub fn new(grid : GridBase<T,Idx, N>) -> Self where Param : Default { Self::new_with_param(grid, Param::default()) }

    pub fn grid(&self) -> &GridBase<T,Idx, N> { &self.grid }
    pub fn grid_mut(&mut self) -> &mut GridBase<T,Idx, N> { &mut self.grid }

    pub fn param(&self) -> &Param { &self.param }
    pub fn param_mut(&mut self) -> &mut Param { &mut self.param }
}

impl<T,Param,Idx,const N : usize> IRectangle<Idx,N> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
{
    fn size (&self) -> Vector<Idx,N> { self.grid.size()  }
    fn begin(&self) -> Vector<Idx,N> { self.grid.begin() }
}

impl<T,Param,Idx,const N : usize> IGrid<T,Idx,N> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    fn values(&self) -> &[T] { self.grid.values() }
    fn values_mut(&mut self) -> &mut [T] { self.grid.values_mut() }

    fn into_values(self) -> Vec<T> { self.grid.into_values() }
    
    type Map<Dest>=GridParamBase<Dest,Param,Idx,N>;
    fn map<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest {
        GridParamBase::new_with_param(self.grid.map(f), self.param.clone())
    }
    
    fn transform<Dest, F>(self, f : F) -> Self::Map<Dest> where F : FnMut(T) -> Dest, Self : Sized {
        GridParamBase::new_with_param(self.grid.transform(f), self.param)
    }
    
    fn crop_margin(&self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> Self where T : Clone, Self : Sized {
        GridParamBase::new_with_param(self.grid.crop_margin(margin_start, margin_end), self.param.clone())
    }
    
    type View<'a>  where Self: 'a;
    fn view<'a>(&'a self) -> Self::View<'a> {
        
    }
    
    type ViewMut<'a>  where Self: 'a;
    
    fn view_mut<'a>(&'a mut self) -> Self::ViewMut<'a> {
        todo!()
    }
}



/// A slice inside a [Grid]
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


impl<'a, T, Param, Idx, const N : usize> IGridView<T,Idx,N> for GridParamView<'a, T, Param, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
{
    fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { self.view.get(pos) }
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { self.view.get_unchecked(self.view.pos + pos) } }
    

    
    type ToGrid=GridParamBase<T,Param,Idx,N>;
    fn to_grid(self) -> Self::ToGrid where T : Clone {
        todo!()
    }
    
    type SubView<'b> = GridParamView<'b,T,Param,Idx,N> where Self: 'b;
    fn subview<'b>(&'b self, rect : Rectangle<Idx, N>) -> Self::SubView<'b> where T : Clone {
        Self::from_view(self.view.subview(rect), self.param) }
        
    fn subgrid(&self, rect : Rectangle<Idx, N>) -> Self::ToGrid where T : Clone, Self : Sized {
        todo!()
    }
    


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
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get(index).unwrap() }
}


*/