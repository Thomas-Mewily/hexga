use crate::*;

pub trait IGridParam<T, Param, Idx, const N : usize> where Idx : IntegerIndex, Self : Sized + IGrid<T,Param,Idx,N>
{
    fn grid(&self) -> &GridBase<T,Idx, N>;
    fn grid_mut(&mut self) -> &mut GridBase<T,Idx, N>;

    fn param(&self) -> &Param;
    fn param_mut(&mut self) -> &mut Param;

    fn set_param(&mut self, param : Param) -> &mut Self { *self.param_mut() = param; self }
    fn with_param(mut self, param : Param) -> Self { *self.param_mut() = param; self }

    fn set_grid(&mut self, grid : GridBase<T,Idx, N>) -> &mut Self { *self.grid_mut() = grid; self }
    fn with_grid(mut self, grid : GridBase<T,Idx, N>) -> Self { *self.grid_mut() = grid; self }

    fn from_grid_with_param(grid : GridBase<T,Idx, N>, param : Param) -> Self;
    fn unpack(self) -> (GridBase<T,Idx, N>, Param);

    fn from_grid(grid : GridBase<T,Idx, N>) -> Self where Param : Default { Self::from_grid_with_param(grid, Param::___()) }

    fn from_vec_with_param(size : Vector::<Idx,N>, value : Vec<T>, param : Param) -> Option<Self> { Self::try_from_vec_with_param(size, value, param).ok() }
    fn from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Option<Self> where Param : Default { Self::try_from_vec(size, value).ok() }
    
    fn try_from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Result<Self, GridBaseError<Idx,N>> where Param : Default { GridBase::try_from_vec(size, value).map(|g| Self::from_grid(g)) }
    fn try_from_vec_with_param(size : Vector::<Idx,N>, value : Vec<T>, param : Param) -> Result<Self, GridBaseError<Idx,N>>
    {
        GridBase::try_from_vec(size, value).map(|g| Self::from_grid_with_param(g, param))
    }

    /// Create a grid from a function
    fn from_fn_with_param<F>(size : Vector::<Idx,N>, param : Param, f : F) -> Self where F : FnMut(Vector::<Idx,N>) -> T  { Self::from_grid_with_param(GridBase::from_fn(size, f), param) }
    /// Create a grid from a function
    fn from_fn<F>(size : Vector::<Idx,N>, f : F) -> Self where F : FnMut(Vector::<Idx,N>) -> T, Param : Default { Self::from_grid(GridBase::from_fn(size, f)) }

    /// Fill the grid with the [Default] value
    fn new(size : Vector::<Idx,N>) -> Self where T : Default, Param : Default { Self::from_grid(GridBase::new(size)) }
    /// Fill the grid with the [Default] value
    fn new_with_param(size : Vector::<Idx,N>, param : Param) -> Self where T : Default { Self::from_grid_with_param(GridBase::new(size), param) }
    
    /// Fill the grid by cloning the value
    fn new_uniform(size : Vector::<Idx,N>, value : T) -> Self where T : Clone, Param : Default { Self::from_grid(GridBase::new_uniform(size, value)) }
    /// Fill the grid by cloning the value
    fn new_uniform_with_param(size : Vector::<Idx,N>, value : T, param : Param) -> Self where T : Clone { Self::from_grid_with_param(GridBase::new_uniform(size, value), param) }


    /// Create a grid from a function in parallel
    fn from_fn_with_param_par<F>(size : Vector::<Idx,N>, param : Param, f : F) -> Self where F : Fn(Vector::<Idx,N>) -> T + Sync, T : Send, Idx : Sync { Self::from_grid_with_param(GridBase::from_fn_par(size, f), param) }
    /// Create a grid from a function in parallel
    fn from_fn_par<F>(size : Vector::<Idx,N>, f : F) -> Self where F : Fn(Vector::<Idx,N>) -> T + Sync, T : Send, Idx : Sync, Param : Default { Self::from_fn_with_param_par(size, ___(), f) }
    
    /// Fill the grid with the [Default] value in parallel
    fn new_with_param_par(size : Vector::<Idx,N>, param : Param) -> Self where T : Default + Send, Idx : Sync { Self::from_grid_with_param(GridBase::new_par(size), param) }
    /// Fill the grid with the [Default] value in parallel
    fn new_par(size : Vector::<Idx,N>) -> Self where T : Default + Send, Idx : Sync, Param : Default { Self::new_with_param_par(size, ___()) }
    
    /// Fill the grid by cloning the value in parallel
    fn new_uniform_with_param_par(size : Vector::<Idx,N>, value : T, param : Param) -> Self where T : Clone + Sync + Send, Idx : Sync { Self::from_grid_with_param(GridBase::new_uniform_par(size, value), param) }
    /// Fill the grid by cloning the value in parallel
    fn new_uniform_par(size : Vector::<Idx,N>, value : T) -> Self where T : Clone + Sync + Send, Idx : Sync, Param : Default { Self::new_uniform_with_param_par(size, value, ___()) }
}

/// A Grid that have some parameter associate with it
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct GridParamBase<T, Param, Idx, const N : usize> where Idx : IntegerIndex
{
    pub grid    : GridBase<T,Idx, N>,
    pub param   : Param,
}

impl<T,Param,Idx,const N : usize> IGridParam<T,Param,Idx,N> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex
{
    fn grid(&self) -> &GridBase<T,Idx, N> { &self.grid }
    fn grid_mut(&mut self) -> &mut GridBase<T,Idx, N> { &mut self.grid }

    fn param(&self) -> &Param { &self.param }
    fn param_mut(&mut self) -> &mut Param { &mut self.param }
    
    fn from_grid_with_param(grid : GridBase<T,Idx, N>, param : Param) -> Self { Self { grid, param } }
    fn unpack(self) -> (GridBase<T,Idx, N>, Param) 
    {
        let GridParamBase{ grid, param } = self;
        (grid, param)
    }
}

impl<T,Param,Idx,const N : usize> Get<Vector<Idx,N>> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex
{
    type Output = <Self as Index<Vector<Idx,N>>>::Output;
    #[inline(always)]
    fn try_get(&self, pos : Vector<Idx,N>) -> Result<&Self::Output, ()> { self.grid.try_get(pos) }
    #[inline(always)]
    fn get(&self, pos : Vector<Idx,N>) -> Option<&Self::Output> { self.grid.get(pos) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &Self::Output { unsafe { self.grid.get_unchecked(pos) } }
}
impl<T,Param,Idx,const N : usize> GetMut<Vector<Idx,N>> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex
{
    #[inline(always)]
    fn try_get_mut(&mut self, pos : Vector<Idx,N>) -> Result<&mut Self::Output, ()> { self.grid.try_get_mut(pos) }
    #[inline(always)]
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut Self::Output> { self.grid.get_mut(pos) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut Self::Output { unsafe { self.grid.get_unchecked_mut(pos) } }
}

impl<T,Param,Idx,const N : usize> GetManyMut<Vector<Idx,N>> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex
{
    #[inline(always)]
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [Vector<Idx,N>; N2]) -> Result<[&mut Self::Output;N2], ()> { self.grid.try_get_many_mut(indices) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<const N2: usize>(&mut self, indices: [Vector<Idx,N>; N2]) -> [&mut Self::Output;N2] { unsafe { self.grid.get_many_unchecked_mut(indices) } }
}

impl<T,Param,Idx,const N : usize> Get<usize> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex
{
    type Output = <Self as Index<usize>>::Output;
    #[inline(always)]
    fn try_get(&self, index : usize) -> Result<&Self::Output, ()> { self.grid.try_get(index) }
    #[inline(always)]
    fn get(&self, index : usize) -> Option<&Self::Output> { self.grid.get(index) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, index : usize) -> &Self::Output { unsafe { self.grid.get_unchecked(index) } }
}
impl<T,Param,Idx,const N : usize> GetMut<usize> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex
{
    #[inline(always)]
    fn try_get_mut(&mut self, index : usize) -> Result<&mut Self::Output, ()> { self.grid.try_get_mut(index) }
    #[inline(always)]
    fn get_mut(&mut self, index : usize) -> Option<&mut Self::Output> { self.grid.get_mut(index) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(&mut self, index : usize) -> &mut Self::Output { unsafe { self.grid.get_unchecked_mut(index) } }
}

impl<T,Param,Idx,const N : usize> GetManyMut<usize> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex
{
    #[inline(always)]
    #[track_caller]
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [usize; N2]) -> Result<[&mut Self::Output;N2], ()> { self.grid.try_get_many_mut(indices) }
}

impl<T,Param,Idx,const N : usize> IRectangle<Idx,N> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex,
{
    fn size (&self) -> Vector<Idx,N> { self.grid.size()  }
    fn begin(&self) -> Vector<Idx,N> { self.grid.begin() }

    fn iter_x(&self) -> Range<Idx> where Vector<Idx,N> : HaveX<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_x() }
    fn iter_y(&self) -> Range<Idx> where Vector<Idx,N> : HaveY<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_y() }
    fn iter_z(&self) -> Range<Idx> where Vector<Idx,N> : HaveZ<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_z() }
    fn iter_w(&self) -> Range<Idx> where Vector<Idx,N> : HaveW<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_w() }

    #[inline(always)] fn is_inside_x(&self, x : Idx) -> bool where Vector<Idx,N> : HaveX<Idx> { x >= Idx::ZERO && x < self.size_x() }
    #[inline(always)] fn is_inside_y(&self, y : Idx) -> bool where Vector<Idx,N> : HaveY<Idx> { y >= Idx::ZERO && y < self.size_y() }
    #[inline(always)] fn is_inside_z(&self, z : Idx) -> bool where Vector<Idx,N> : HaveZ<Idx> { z >= Idx::ZERO && z < self.size_z() }
    #[inline(always)] fn is_inside_w(&self, w : Idx) -> bool where Vector<Idx,N> : HaveW<Idx> { w >= Idx::ZERO && w < self.size_w() }
}

impl<T,Param,Idx,const N : usize> Crop<Idx,N> for GridParamBase<T,Param,Idx,N> where Param : Clone, GridBase<T,Idx,N> : Crop<Idx,N>, Idx : IntegerIndex
{
    fn crop(self, subrect : Rectangle<Idx, N>) -> Option<Self>  { self.grid.crop(subrect).map(|g| Self::from_grid_with_param(g, self.param)) }
    unsafe fn crop_unchecked(self, subrect : Rectangle<Idx, N>) -> Self { Self::from_grid_with_param(unsafe { self.grid.crop_unchecked(subrect) }, self.param) }
}
impl<T,Param,Idx,const N : usize> IGrid<T,Param,Idx,N> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex,
{
    fn values(&self) -> &[T] { self.grid.values() }
    fn values_mut(&mut self) -> &mut [T] { self.grid.values_mut() }

    fn into_values(self) -> Vec<T> { self.grid.into_values() }
    
    fn transform<Dest, F>(self, f : F) -> Self::Map<Dest> where F : FnMut(T) -> Dest 
    { GridParamBase::from_grid_with_param(self.grid.transform(f), self.param) }
        
    fn transform_par<Dest, F>(self, f : F) -> <Self as IGridView<T,Param,Idx,N>>::Map<Dest> where F : Fn(T) -> Dest + Sync + Send, T : Send + Sync, Dest : Send, Idx : Sync, Param : Clone 
    { GridParamBase::from_grid_with_param(self.grid.transform_par(f), self.param) }
    
    type View<'a> = GridParamView<'a,T,Param,Idx,N> where Self: 'a;
    fn view<'a>(&'a self) -> Self::View<'a> { Self::View::from_view(self.grid.view(), &self.param) }
    
    type ViewMut<'a> = GridParamViewMut<'a,T,Param,Idx,N> where Self: 'a;
    fn view_mut<'a>(&'a mut self) -> Self::ViewMut<'a> { Self::ViewMut::from_view_mut(self.grid.view_mut(), &mut self.param) }
}

impl<T,Param,Idx,const N : usize> IGridView<T,Param,Idx,N> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex,
{
    type Map<Dest>=GridParamBase<Dest,Param,Idx,N>;

    fn map<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, Param : Clone { GridParamBase::from_grid_with_param(self.grid.map(f), self.param.clone()) }
    fn map_par<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, Param : Clone { GridParamBase::from_grid_with_param(self.grid.map_par(f), self.param.clone()) }
}


impl<T,Param,Idx,const N : usize> IGridViewMut<T,Param,Idx,N> for GridParamBase<T,Param,Idx,N> where Idx : IntegerIndex,
{
    type SubViewMut<'b> = GridParamViewMut<'b, T, Param, Idx, N> where Self: 'b;
    fn subview_mut<'a>(&'a mut self, rect : Rectangle<Idx, N>) -> Self::SubViewMut<'a> 
    { Self::SubViewMut::from_view_mut(self.grid.subview_mut(rect), &mut self.param) }
}

impl<T, Param, Idx, const N : usize> Index<usize> for GridParamBase<T,Param,Idx, N> where Idx : IntegerIndex,
{
    type Output=T;
    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output { self.get_or_panic(index) }
}
impl<T, Param, Idx, const N : usize> IndexMut<usize> for GridParamBase<T,Param,Idx, N> where Idx : IntegerIndex,
{
    #[track_caller]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_mut_or_panic(index) }
}

impl<T, Param, Idx, const N : usize> Index<Vector<Idx,N>> for GridParamBase<T,Param,Idx, N> where Idx : IntegerIndex,
{
    type Output=T;
    #[track_caller]
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get_or_panic(index) }
}
impl<T, Param, Idx, const N : usize> IndexMut<Vector<Idx,N>> for GridParamBase<T,Param,Idx, N> where Idx : IntegerIndex,
{
    #[track_caller]
    fn index_mut(&mut self, index: Vector<Idx,N>) -> &mut Self::Output { self.get_mut_or_panic(index) }
}

impl<T, Param, Idx, const N : usize> GridParamBase<T,Param,Idx, N> where Idx : IntegerIndex,
{
    pub fn iter(&self) -> Iter<'_,T,Idx,N> { self.grid.iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_,T,Idx,N> { self.grid.iter_mut() }
}

impl<T, Param, Idx, const N : usize> Length for GridParamBase<T,Param,Idx, N> 
    where Idx : IntegerIndex,
{ 
    fn len(&self) -> usize { self.grid().len() }
}