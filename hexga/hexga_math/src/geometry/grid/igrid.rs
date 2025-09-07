use crate::*;

#[non_exhaustive]
#[derive(Clone)]
pub enum GridBaseError<Idx, const N : usize> where Idx : Integer
{
    NegativeSize(Vector::<Idx,N>),
    /// (dim, got)
    WrongDimension(Vector<Idx,N>, usize),
    ToBig(Vector<Idx,N>),
}
impl<Idx, const N : usize> Debug for GridBaseError<Idx, N> where Idx : Debug, Idx : Integer
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            GridBaseError::NegativeSize(size) => write!(f, "Can't have a negative size {:?}", size),
            GridBaseError::WrongDimension(size, got) => write!(f, "Wrong dimension : expected {:?} elements for a {:?} grid but got {:?} elements", size.area_usize(), size, got),
            GridBaseError::ToBig(size) => write!(f, "The size {:?} is too big !", size),
        }
    }
}


pub trait IGrid<T, Idx, const N : usize> :
      GetRectangle<Idx,N> + GetPosition<Idx,N>

    + TryGet<Vector<Idx,N>,Output=T>
    + Get<Vector<Idx,N>,Output=T>
    + Index<Vector<Idx,N>,Output=T>

    /*
    + for<'a> Get<Rectangle<Idx,N>,Output=GridView<'a,>
    + Index<Rectangle<Idx,N>,Output=T>
*/
    + TryGetMut<Vector<Idx,N>,Output = T>
    + GetMut<Vector<Idx,N>,Output = T> 
    + GetManyMut<Vector<Idx,N>,Output=T>
    + IndexMut<Vector<Idx,N>, Output = T>

    + Composite<Inside=T>
    + CompositeGeneric<Inside=T>

    + Sized
    where Idx : Integer
{
    type WithType<U> : IGrid<U, Idx, N>;

    // Those expose the implementation, put in another trait like IGridVector
    fn values(&self) -> &[T];
    fn values_mut(&mut self) -> &mut [T];
    fn into_values(self) -> Vec<T> { self.into_size_and_values().1 }
    fn into_size_and_values(self) -> (Vector<Idx,N>, Vec<T>);


    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    ///
    /// If you override this, you probably also want to override [IGrid::index_to_position_unchecked] and [IGrid::external_position_to_position_unchecked]
    unsafe fn position_to_index_unchecked<P>(&self, pos : P) -> usize where P: Into<Vector<Idx,N>> { unsafe { Vector::<Idx,N>::to_index_unchecked(pos.into(), self.size()) } }
    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    ///
    /// If you override this, you probably also want to override [IGrid::position_to_index_unchecked] and [IGrid::external_position_to_position_unchecked]
    unsafe fn index_to_position_unchecked(&self, index : usize) -> Vector<Idx,N> { unsafe { Vector::<Idx,N>::from_index_unchecked(index, self.size()) } }
    #[allow(unused_variables)]
    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    ///
    /// If you override this, you probably also want to override [IGrid::position_to_index_unchecked] and [IGrid::index_to_position_unchecked]
    unsafe fn external_position_to_position_unchecked<P>(pos : P, size : P) -> P where P: Into<Vector<Idx,N>> + From<Vector<Idx,N>> { pos }

    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    #[inline(always)]
    fn position_to_index<P>(&self, pos: P) -> Option<usize> where P: Into<Vector<Idx,N>> { let pos = pos.into(); pos.is_inside(self.size()).then(|| unsafe { self.position_to_index_unchecked(pos) }) }
    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    #[inline(always)]
    fn index_to_position(&self, index : usize) -> Option<Vector<Idx,N>> { (index < self.size().area_usize()).then(|| unsafe { self.index_to_position_unchecked(index) }) }



    fn from_vec<P>(size : P, value : Vec<T>) -> Option<Self> where P: Into<Vector<Idx,N>> { Self::try_from_vec(size, value).ok() }
    fn try_from_vec<P>(size : P, value : Vec<T>) -> Result<Self, GridBaseError<Idx,N>> where P: Into<Vector<Idx,N>>
    {
        let size = size.into();
        if *size.min_element() < Idx::ZERO { return Err(GridBaseError::NegativeSize(size)); }

        let area_size = match size.area_usize_checked()
        {
            Some(v) => v,
            None => return Err(GridBaseError::ToBig(size)),
        };
        if area_size != value.len() { return Err(GridBaseError::WrongDimension(size, area_size)); }
        Ok(unsafe { Self::from_vec_unchecked(size, value) })
    }

    unsafe fn from_vec_unchecked<P>(size : P, value : Vec<T>) -> Self where P: Into<Vector<Idx,N>>;


    /// Create a grid from a function
    fn from_fn<P,F>(size : P, mut f : F) -> Self
        where
        F : FnMut(P) -> T,
        P : Into<Vector::<Idx,N>>,
        Vector::<Idx,N> : Into<P>,
    {
        let size = size.into();
        let area = size.area_usize();
        let mut value = Vec::with_capacity(area);
        for idx in size.iter_index()
        {
            value.push(f(unsafe { Self::external_position_to_position_unchecked::<Vector<Idx,N>>(idx,size) }.into()));
        }
        unsafe { Self::from_vec_unchecked(size, value) }
    }
    /// Create a grid from a function in parallel
    fn from_fn_par<P,F>(size : P, f : F) -> Self
        where
        F : Fn(P) -> T + Sync,
        P : Into<Vector::<Idx,N>>,
        Vector::<Idx,N> : Into<P>,

        T : Send,
        Idx : Sync,
    {
        let size = size.into();
        let area = size.area_usize();
        let mut values = Vec::with_capacity(area);
        (0..area).into_par_iter().map(|i| f(unsafe { Self::external_position_to_position_unchecked::<Vector<Idx,N>>(Vector::<Idx, N>::from_index_unchecked(i, size), size) }.into())).collect_into_vec(&mut values);
        unsafe { Self::from_vec_unchecked(size, values) }
    }

    /// Create a new grid and fill it with the [Default] value
    fn new(size : Vector::<Idx,N>) -> Self where T: Default { Self::try_from_vec(size, (0..size.area_usize()).map(|_| ___()).collect()).unwrap() }

    /// Create a new grid and fill it by [Clone] the value
    fn new_uniform(size : Vector::<Idx,N>, value : T) -> Self where T: Clone { Self::try_from_vec(size, vec![value; size.area_usize()]).unwrap() }




    /// Create a grid from a Normalized Device Coordinates.
    ///
    /// Each component of the vector will be between `-1.0..=1.0`
    fn from_fn_ndc<P,F>(size : P, f : F) -> Self
    where
    P : Into<Vector::<Idx,N>>,
    Vector::<Idx,N> : CastIntoComposite<float,Output = Vector::<float,N>>,
    F : FnMut(Vector<float,N>) -> T{ Self::from_fn_ndc_with_precision(size, f) }

    /// Create a grid from a Normalized Device Coordinates.
    ///
    /// Each component of the vector will be between `-1.0..=1.0`
    fn from_fn_ndc_with_precision<P,Precision,F>(size : P, mut f : F) -> Self
        where
        P : Into<Vector::<Idx,N>>,
        Precision : Float,
        Vector::<Idx,N> : CastIntoComposite<Precision,Output = Vector::<Precision,N>>,
        F : FnMut(Vector<Precision,N>) -> T,
    {
        let size : Vector::<Idx,N> = size.into();
        let size_float : Vector::<Precision,N> = size.cast_into_composite();

        Self::from_fn(size, |p|
            {
                let pos_float : Vector::<Precision,N> = p.cast_into_composite();
                f(pos_float / size_float * Precision::TWO - Vector::<Precision,N>::ONE)
            })
    }





    /// Create a grid from a Normalized Coordinates.
    ///
    /// Each component of the vector will be between `0..=1.0`
    fn from_fn_normalized<P,F>(size : P, f : F) -> Self
    where
    P : Into<Vector::<Idx,N>>,
    Vector::<Idx,N> : CastIntoComposite<float,Output = Vector::<float,N>>,
    F : FnMut(Vector<float,N>) -> T{ Self::from_fn_normalized_with_precision(size, f) }

    /// Create a grid from a Normalized Coordinates.
    ///
    /// Each component of the vector will be between `0..=1.0`
    fn from_fn_normalized_with_precision<P,Precision,F>(size : P, mut f : F) -> Self
        where
        P : Into<Vector::<Idx,N>>,
        Precision : Float,
        Vector::<Idx,N> : CastIntoComposite<Precision,Output = Vector::<Precision,N>>,
        F : FnMut(Vector<Precision,N>) -> T,
    {
        let size : Vector::<Idx,N> = size.into();
        let size_float : Vector::<Precision,N> = size.cast_into_composite();

        Self::from_fn(size, |p|
            {
                let pos_float : Vector::<Precision,N> = p.cast_into_composite();
                f(pos_float / size_float)
            })
    }

    fn map<Dest,F>(self, mut f : F) -> <Self as IGrid<T,Idx,N>>::WithType<Dest> where F : FnMut(T) -> Dest
    {
        let size = self.size();
        unsafe { <Self as IGrid<T,Idx,N>>::WithType::from_vec_unchecked(size, self.into_values().into_iter().map(|v| f(v)).collect()) }
    }
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