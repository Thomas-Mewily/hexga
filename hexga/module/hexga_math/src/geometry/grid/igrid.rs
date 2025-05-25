use crate::*;

#[derive(Clone)]
pub enum GridBaseError<Idx, const N : usize> where Idx : Integer
{
    NegativeSize(Vector::<Idx,N>),
    /// (dim, got)
    WrongDimension(Vector<Idx,N>, usize),
}
impl<Idx, const N : usize> Debug for GridBaseError<Idx, N> where Idx : Debug, Idx : Integer
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult {
        match self {
            Self::NegativeSize(size) => write!(f, "grid have a negative size {:?}", size),
            Self::WrongDimension(size, got) => write!(f, "grid vector have a wrong dimension : expected {:?} elements for a {:?} grid but only got {:?} elements", size.area(), size, got),
        }
    }
}

pub trait IGrid<T, Idx, const N : usize> : 
    Get<Vector<Idx,N>,Output = T> + GetMut<Vector<Idx,N>,Output = T> + GetManyMut<Vector<Idx,N>,Output=T>
    + Index<Vector<Idx,N>, Output = T>
    + IndexMut<Vector<Idx,N>, Output = T>
    + IRectangle<Idx, N>
    + AsRef<[T]>
    + AsMut<[T]>
    //+ Crop<Idx,N> // if T is Clone
    + IGridViewMut<Self,T,Idx,N>
    + Sized
    where Idx : Integer
{
    type WithType<U> : IGrid<U, Idx, N>;

    // Those expose the implementation
    fn values(&self) -> &[T];
    fn values_mut(&mut self) -> &mut [T];
    fn into_values(self) -> Vec<T> { self.into_size_and_values().1 }
    fn into_size_and_values(self) -> (Vector<Idx,N>, Vec<T>);


    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    ///
    /// If you override this, you probably also want to override [IGrid::index_to_position_unchecked] and [IGrid::external_position_to_position_unchecked]
    unsafe fn position_to_index_unchecked(&self, pos : Vector<Idx,N>) -> usize { unsafe { Vector::<Idx,N>::to_index_unchecked(pos, self.size()) } }
    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    ///
    /// If you override this, you probably also want to override [IGrid::position_to_index_unchecked] and [IGrid::external_position_to_position_unchecked]
    unsafe fn index_to_position_unchecked(&self, index : usize) -> Vector<Idx,N> { unsafe { Vector::<Idx,N>::from_index_unchecked(index, self.size()) } }
    #[allow(unused_variables)]
    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    ///
    /// If you override this, you probably also want to override [IGrid::position_to_index_unchecked] and [IGrid::index_to_position_unchecked]
    unsafe fn external_position_to_position_unchecked(pos : Vector<Idx,N>, size : Vector<Idx,N>) -> Vector<Idx,N> { pos }

    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    #[inline(always)]
    fn position_to_index(&self, pos : Vector<Idx,N>) -> Option<usize> { pos.is_inside(self.size()).then(|| unsafe { self.position_to_index_unchecked(pos) }) }
    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    #[inline(always)]
    fn index_to_position(&self, index : usize) -> Option<Vector<Idx,N>> { (index < self.size().area_usize()).then(|| unsafe { self.index_to_position_unchecked(index) }) }



    fn from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Option<Self> { Self::try_from_vec(size, value).ok() }
    fn try_from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Result<Self, GridBaseError<Idx,N>>
    {
        if *size.min_element() <= Idx::ZERO { return Err(GridBaseError::NegativeSize(size)); }
        let area_size = size.area_usize();
        if area_size != value.len() { return Err(GridBaseError::WrongDimension(size, area_size)); }
        Ok(unsafe { Self::unchecked_from_vec(size, value) })
    }

    unsafe fn unchecked_from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Self;

    /// Create a grid from a function
    fn from_fn<F>(size : Vector::<Idx,N>, mut f : F) -> Self where F : FnMut(Vector::<Idx,N>) -> T 
    {
        let area = size.area_usize();
        let mut value = Vec::with_capacity(area);
        for idx in size.iter_index()
        {
            value.push(f(unsafe { Self::external_position_to_position_unchecked(idx,size) }));
        }
        unsafe { Self::unchecked_from_vec(size, value) }
    }
    /// Create a grid from a function in parallel
    fn from_fn_par<F>(size : Vector::<Idx,N>, f : F) -> Self where F : Fn(Vector::<Idx,N>) -> T + Sync, T : Send, Idx : Sync 
    {
        let area = size.area_usize();
        let mut values = Vec::with_capacity(area);
        (0..area).into_par_iter().map(|i| f(unsafe { Self::external_position_to_position_unchecked(Vector::<Idx, N>::from_index_unchecked(i, size), size) })).collect_into_vec(&mut values);
        unsafe { Self::unchecked_from_vec(size, values) }
    }

    /// Create a new grid and fill it with the [Default] value
    fn new(size : Vector::<Idx,N>) -> Self where T : Default { Self::try_from_vec(size, (0..size.area_usize()).map(|_| ___()).collect()).unwrap() }

    /// Create a new grid and fill it by [Clone] the value
    fn new_uniform(size : Vector::<Idx,N>, value : T) -> Self where T : Clone { Self::try_from_vec(size, vec![value; size.area_usize()]).unwrap() }





    
    fn iter(&self) -> GridViewIter<'_, Self, T, Idx, N> { GridViewIter::new(self) }
    fn iter_mut(&mut self) -> GridViewIterMut<'_, Self, T, Idx, N> { GridViewIterMut::new(self) }

    //fn view(&self) -> GridView<'_,Self,T,Idx,N> { GridView::new(self) }
    fn view_mut(&mut self) -> GridViewMut<'_,Self,T,Idx,N> { GridViewMut::new(self) }
    
    /// `self.view().crop_intersect(subrect).to_grid()`
    fn subgrid(&self, rect : Rectangle<Idx,N>) -> Self where T : Clone { self.view().subgrid(rect) }
}


