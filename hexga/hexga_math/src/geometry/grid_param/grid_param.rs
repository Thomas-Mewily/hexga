use crate::*;

/// A Grid that have some parameter associate with it
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct GridParamBase<T, Param, Idx, const N : usize> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    pub grid    : GridBase<T,Idx, N>,
    pub param   : Param,
}

impl<T,Param,Idx,const N : usize> GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    pub fn from_grid_with_param(grid : GridBase<T,Idx, N>, param : Param) -> Self { Self { grid, param }}
    pub fn from_grid(grid : GridBase<T,Idx, N>) -> Self where Param : Default { Self::from_grid_with_param(grid, Param::default()) }

    pub fn grid(&self) -> &GridBase<T,Idx, N> { &self.grid }
    pub fn grid_mut(&mut self) -> &mut GridBase<T,Idx, N> { &mut self.grid }

    pub fn param(&self) -> &Param { &self.param }
    pub fn param_mut(&mut self) -> &mut Param { &mut self.param }

    pub fn set_param(&mut self, param : Param) -> &mut Self { self.param = param; self }
    pub fn with_param(mut self, param : Param) -> Self { self.set_param(param); self }

    pub fn set_grid(&mut self, grid : GridBase<T,Idx, N>) -> &mut Self { self.grid = grid; self }
    pub fn with_grid(mut self, grid : GridBase<T,Idx, N>) -> Self { self.set_grid(grid); self }
}

/// Same constructor as grid
impl<T,Param,Idx,const N : usize> GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Default
{
    pub fn from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Option<Self> { Self::try_from_vec(size, value).ok() }
    pub fn try_from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Result<Self, GridBaseError<Idx,N>>
    {
        GridBase::try_from_vec(size, value).map(|g| Self::from_grid(g))
    }

    pub fn from_fn<F>(size : Vector::<Idx,N>, f : F) -> Self where F : FnMut(Vector::<Idx,N>) -> T  { Self::from_grid(GridBase::from_fn(size, f)) }

    /// Fill the grid with the [Default] value
    pub fn new(size : Vector::<Idx,N>) -> Self where T : Default { Self::from_grid(GridBase::new(size)) }
    /// Fill the grid by cloning the value
    pub fn new_uniform(size : Vector::<Idx,N>, value : T) -> Self where T : Clone { Self::from_grid(GridBase::new_uniform(size, value)) }
}

/// Same constructor as grid + with param
impl<T,Param,Idx,const N : usize> GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
{
    pub fn from_vec_with_param(size : Vector::<Idx,N>, value : Vec<T>, param : Param) -> Option<Self> { Self::try_from_vec_with_param(size, value, param).ok() }
    pub fn try_from_vec_with_param(size : Vector::<Idx,N>, value : Vec<T>, param : Param) -> Result<Self, GridBaseError<Idx,N>>
    {
        GridBase::try_from_vec(size, value).map(|g| Self::from_grid_with_param(g, param))
    }

    pub fn from_fn_with_param<F>(size : Vector::<Idx,N>, f : F, param : Param) -> Self where F : FnMut(Vector::<Idx,N>) -> T  { Self::from_grid_with_param(GridBase::from_fn(size, f), param) }

    /// Fill the grid with the [Default] value
    pub fn new_with_param(size : Vector::<Idx,N>, param : Param) -> Self where T : Default { Self::from_grid_with_param(GridBase::new(size), param) }
    /// Fill the grid by cloning the value
    pub fn new_uniform_with_param(size : Vector::<Idx,N>, value : T, param : Param) -> Self where T : Clone { Self::from_grid_with_param(GridBase::new_uniform(size, value), param) }
}

impl<T,Param,Idx,const N : usize> IRectangle<Idx,N> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
{
    fn size (&self) -> Vector<Idx,N> { self.grid.size()  }
    fn begin(&self) -> Vector<Idx,N> { self.grid.begin() }

    fn iter_x(&self) -> Range<Idx> where Vector<Idx,N> : HaveX<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_x() }
    fn iter_y(&self) -> Range<Idx> where Vector<Idx,N> : HaveY<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_y() }
    fn iter_z(&self) -> Range<Idx> where Vector<Idx,N> : HaveZ<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_z() }
    fn iter_w(&self) -> Range<Idx> where Vector<Idx,N> : HaveW<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_w() }

    #[inline] fn is_inside_x(&self, x : Idx) -> bool where Vector<Idx,N> : HaveX<Idx> { x >= Idx::ZERO && x < self.size_x() }
    #[inline] fn is_inside_y(&self, y : Idx) -> bool where Vector<Idx,N> : HaveY<Idx> { y >= Idx::ZERO && y < self.size_y() }
    #[inline] fn is_inside_z(&self, z : Idx) -> bool where Vector<Idx,N> : HaveZ<Idx> { z >= Idx::ZERO && z < self.size_z() }
    #[inline] fn is_inside_w(&self, w : Idx) -> bool where Vector<Idx,N> : HaveW<Idx> { w >= Idx::ZERO && w < self.size_w() }
}

impl<T,Param,Idx,const N : usize> IGrid<T,Idx,N> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    fn values(&self) -> &[T] { self.grid.values() }
    fn values_mut(&mut self) -> &mut [T] { self.grid.values_mut() }

    fn into_values(self) -> Vec<T> { self.grid.into_values() }
    
    type Map<Dest>=GridParamBase<Dest,Param,Idx,N>;
    fn map<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest {
        GridParamBase::from_grid_with_param(self.grid.map(f), self.param.clone())
    }
    
    fn transform<Dest, F>(self, f : F) -> Self::Map<Dest> where F : FnMut(T) -> Dest, Self : Sized {
        GridParamBase::from_grid_with_param(self.grid.transform(f), self.param)
    }
    
    fn crop_margin(&self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> Self where T : Clone, Self : Sized {
        GridParamBase::from_grid_with_param(self.grid.crop_margin(margin_start, margin_end), self.param.clone())
    }
    
    type View<'a> = GridParamView<'a,T,Param,Idx,N> where Self: 'a;
    fn view<'a>(&'a self) -> Self::View<'a> { Self::View::from_view(self.grid.view(), &self.param) }
    
    type ViewMut<'a> = GridParamViewMut<'a,T,Param,Idx,N> where Self: 'a;
    fn view_mut<'a>(&'a mut self) -> Self::ViewMut<'a> { Self::ViewMut::from_view(self.grid.view_mut(), &mut self.param) }
}


impl<T,Param,Idx, const N : usize> GridParamBase<T, Param, Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    pub fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { IGrid::get(self, pos) }
    pub fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut T> { IGrid::get_mut(self, pos) }
    
    pub unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { IGrid::get_unchecked(self, pos) } }
    pub unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut T { unsafe { IGrid::get_unchecked_mut(self, pos) } }

    pub fn swap(&mut self, pos_a : Vector<Idx,N>, pos_b : Vector<Idx,N>) -> bool { IGrid::swap(self, pos_a, pos_b) }
    pub fn replace(&mut self, val : T, pos : Vector<Idx,N>) ->  Option<T> { IGrid::replace(self, val, pos) }
    pub fn set(&mut self, val : T, pos : Vector<Idx,N>) -> &mut Self { IGrid::set(self, val, pos) }

    pub fn len(&self) -> usize { IGrid::len(self) }
}

impl<T,Param,Idx,const N : usize> IGridView<T,Idx,N> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { self.grid.get(pos) }
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { self.grid.get_unchecked(pos) } }
    
    type ToGrid=GridParamBase<T,Param,Idx,N>;
    fn to_grid(self) -> Self::ToGrid where T : Clone { Self::ToGrid::from_grid_with_param(self.grid.to_grid(), self.param.clone()) }
    
    type SubView<'b> = GridParamView<'b, T, Param, Idx, N> where Self: 'b;
    fn subview<'b>(&'b self, rect : Rectangle<Idx, N>) -> Self::SubView<'b> where T : Clone 
    { Self::SubView::from_view(self.grid.subview(rect), &self.param) }
        
    fn subgrid(&self, rect : Rectangle<Idx, N>) -> Self::ToGrid where T : Clone, Self : Sized 
    { self.subview(rect).to_grid() }
}


impl<T,Param,Idx,const N : usize> IGridViewMut<T,Idx,N> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut T> { self.grid.get_mut(pos) }

    type SubViewMut<'b> = GridParamViewMut<'b, T, Param, Idx, N> where Self: 'b;
    fn subview_mut<'a>(&'a mut self, rect : Rectangle<Idx, N>) -> Self::SubViewMut<'a> 
    { Self::SubViewMut::from_view(self.grid.subview_mut(rect), &mut self.param) }

    fn swap(&mut self, pos_a : Vector<Idx,N>, pos_b : Vector<Idx,N>) -> bool { self.grid.swap(pos_a, pos_b) }
}

impl<T, Param, Idx, const N : usize> Index<usize> for GridParamBase<T,Param,Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    type Output=T;
    fn index(&self, index: usize) -> &Self::Output { self.get_index(index).unwrap() }
}
impl<T, Param, Idx, const N : usize> IndexMut<usize> for GridParamBase<T,Param,Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_index_mut(index).unwrap() }
}

impl<T, Param, Idx, const N : usize> Index<Vector<Idx,N>> for GridParamBase<T,Param,Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get(index).unwrap() }
}
impl<T, Param, Idx, const N : usize> IndexMut<Vector<Idx,N>> for GridParamBase<T,Param,Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    fn index_mut(&mut self, index: Vector<Idx,N>) -> &mut Self::Output { self.get_mut(index).unwrap() }
}

impl<T, Param, Idx, const N : usize> GridParamBase<T,Param,Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{
    pub fn iter(&self) -> Iter<'_,T,Idx,N> { self.grid.iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_,T,Idx,N> { self.grid.iter_mut() }
}

impl<T, Param, Idx, const N : usize> have_len::HaveLen for GridParamBase<T,Param,Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
    Param : Clone
{ 
    fn len(&self) -> usize { self.len() }
}