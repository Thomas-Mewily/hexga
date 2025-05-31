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
      IRectangle<Idx,N>

    + Get<Vector<Idx,N>,Output=T>
    + Index<Vector<Idx,N>,Output=T>

    + GetMut<Vector<Idx,N>,Output = T> + GetManyMut<Vector<Idx,N>,Output=T>
    + IndexMut<Vector<Idx,N>, Output = T>

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
        Ok(unsafe { Self::from_vec_unchecked(size, value) })
    }

    unsafe fn from_vec_unchecked(size : Vector::<Idx,N>, value : Vec<T>) -> Self;

    /*
    /// [-1.0..=1.0]
    fn from_fn_ndc_with_precision<S,P,F>(size : S, mut f : F) -> Self
        where
        S : Into<Vector::<Idx,N>>,
        P : Float,
        F : FnMut(Vector<P,N>) -> T,
    {
        let size : Vector::<Idx,N> = size.into();

        //let size_float = Vector::<Idx,N> as CastIntoComposite<:: size.cast_into_composite::<P>();

        Self::from_fn(size, |p|
            {
                f(p.to_float() / size_float)
            })
    }
    */

    /// Create a grid from a function
    fn from_fn<S,F>(size : S, mut f : F) -> Self
        where
        F : FnMut(S) -> T,
        S : Into<Vector::<Idx,N>>,
        Vector::<Idx,N> : Into<S>,
    {
        let size = size.into();
        let area = size.area_usize();
        let mut value = Vec::with_capacity(area);
        for idx in size.iter_index()
        {
            value.push(f(unsafe { Self::external_position_to_position_unchecked(idx,size) }.into()));
        }
        unsafe { Self::from_vec_unchecked(size, value) }
    }
    /// Create a grid from a function in parallel
    fn from_fn_par<S,F>(size : S, f : F) -> Self
        where
        F : Fn(S) -> T + Sync,
        S : Into<Vector::<Idx,N>>,
        Vector::<Idx,N> : Into<S>,

        T : Send,
        Idx : Sync,
    {
        let size = size.into();
        let area = size.area_usize();
        let mut values = Vec::with_capacity(area);
        (0..area).into_par_iter().map(|i| f(unsafe { Self::external_position_to_position_unchecked(Vector::<Idx, N>::from_index_unchecked(i, size), size) }.into())).collect_into_vec(&mut values);
        unsafe { Self::from_vec_unchecked(size, values) }
    }

    /// Create a new grid and fill it with the [Default] value
    fn new(size : Vector::<Idx,N>) -> Self where T : Default { Self::try_from_vec(size, (0..size.area_usize()).map(|_| ___()).collect()).unwrap() }

    /// Create a new grid and fill it by [Clone] the value
    fn new_uniform(size : Vector::<Idx,N>, value : T) -> Self where T : Clone { Self::try_from_vec(size, vec![value; size.area_usize()]).unwrap() }
}


impl<T, Idx, const N : usize, S> IGridView<Self, T, Idx, N> for S where S : IGrid<T, Idx, N>, Idx : Integer
{
    type WithLifetime<'a> = GridView<'a, Self, T, Idx, N> where Self: 'a;
    fn view<'a,'b>(&'a self) -> Self::WithLifetime<'b> where 'a : 'b { GridView::new(self) }

    unsafe fn grid_unchecked(&self) -> &Self { self }

    unsafe fn subview_from_translated_rect_unchecked<'a, 'b>(&'a self, rect: Rectangle<Idx, N>) -> Self::WithLifetime<'b> where 'a: 'b
    { unsafe { GridView::from_rect_unchecked(self, rect) } }
}

impl<T, Idx, const N : usize, S> IGridViewMut<Self, T, Idx, N> for S where S : IGrid<T, Idx, N>, Idx : Integer
{
    type WithLifetimeMut<'a> = GridViewMut<'a, Self, T, Idx, N> where Self: 'a;
    fn view_mut<'a,'b>(&'a mut self) -> Self::WithLifetimeMut<'b> where 'a : 'b { GridViewMut::new(self) }

    fn iter_mut(&mut self) -> GridViewIterMut<'_, Self, T, Idx, N> { GridViewIterMut::new(self) }
    fn for_each_mut<F>(&mut self, f : F) where F : FnMut((Vector<Idx,N>,&mut T)) { self.view_mut().for_each_mut(f) }

    unsafe fn grid_mut_unchecked(&mut self) -> &mut Self { self }
}



pub trait ToGrid<T, Idx, const N : usize> where Idx : Integer
{
    type Output : IGrid<T, Idx, N>;
    fn to_grid(self) -> Self::Output;
}

impl<T, Idx, const N : usize> ToGrid<T, Idx, N> for GridBase<T, Idx, N> where Idx : Integer
{
    type Output=Self;
    fn to_grid(self) -> Self::Output { self }
}

impl<'a, T, Idx, const N : usize> ToGrid<T, Idx, N> for GridView<'a,GridBase<T, Idx, N>, T, Idx, N> where Idx : Integer, T : Clone
{
    type Output=GridBase<T, Idx, N>;
    fn to_grid(self) -> Self::Output
    {
        GridBase::<T, Idx, N>::from_fn(self.size(), |idx| unsafe { self.get_unchecked(idx).clone() })
    }
}

impl<'a, T, Idx, const N : usize> ToGrid<T, Idx, N> for GridViewMut<'a,GridBase<T, Idx, N>, T, Idx, N> where Idx : Integer, T : Clone
{
    type Output=GridBase<T, Idx, N>;
    fn to_grid(self) -> Self::Output
    {
        GridBase::<T, Idx, N>::from_fn(self.size(), |idx| unsafe { self.get_unchecked(idx).clone() })
    }
}