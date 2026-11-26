use super::*;

#[non_exhaustive]
#[derive(Clone)]
pub enum GridBaseError<Idx, const N: usize>
where
    Idx: Integer,
{
    NegativeSize(Vector<Idx, N>),
    /// (dim, got)
    WrongDimension(Vector<Idx, N>, usize),
    ToBig(Vector<Idx, N>),
}
impl<Idx, const N: usize> Debug for GridBaseError<Idx, N>
where
    Idx: Debug,
    Idx: Integer,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            GridBaseError::NegativeSize(size) => write!(f, "Can't have a negative size {:?}", size),
            GridBaseError::WrongDimension(size, got) => write!(
                f,
                "Wrong dimension: expected {:?} elements for a {:?} grid but got {:?} elements",
                size.area_usize(),
                size,
                got
            ),
            GridBaseError::ToBig(size) => write!(f, "The size {:?} is too big !", size),
        }
    }
}
impl<Idx, const N: usize> Display for GridBaseError<Idx, N>
where
    Idx: Debug,
    Idx: Integer,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

pub trait IGrid<T, Idx, const N: usize> :
      GetRectangle<Idx,N> + GetPosition<Idx,N>

    + Get<Vector<Idx,N>,Output=T> + TryGet<Vector<Idx,N>,Output=T>
    + Index<Vector<Idx,N>,Output=T>

    /*
    + for<'a> Get<Rectangle<Idx,N>,Output=GridView<'a,>
    + Index<Rectangle<Idx,N>,Output=T>
*/

    + GetMut<Vector<Idx,N>,Output = T> + TryGetMut<Vector<Idx,N>,Output = T> + GetManyMut<Vector<Idx,N>,Output=T>
    + IndexMut<Vector<Idx,N>, Output = T>

    + Map<Item=T> + MapInto<Item=T>

    + Sized
    where Idx: Integer
{
    /// Allocator used by the grid and by owned results derived from it.
    type Allocator: Allocator + Clone;
    type WithType<U>: IGrid<U, Idx, N, Allocator = Self::Allocator>;

    fn allocator(&self) -> &Self::Allocator;

    // Those expose the implementation, put in another trait like IGridVector
    fn values(&self) -> &[T];
    fn values_mut(&mut self) -> &mut [T];
    
    fn into_values(self) -> Vec<T, Self::Allocator> { self.into_size_and_values().1 }
    fn into_size_and_values(self) -> (Vector<Idx,N>, Vec<T, Self::Allocator>);


    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    ///
    /// If you override this, you probably also want to override [`IGrid::index_to_position_unchecked`] and [`IGrid::external_position_to_position_unchecked`]
    unsafe fn position_to_index_unchecked<P>(&self, pos: P) -> usize where P: Into<Vector<Idx,N>> { unsafe { Vector::<Idx,N>::to_index_unchecked(pos.into(), self.size()) } }
    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    ///
    /// If you override this, you probably also want to override [`IGrid::position_to_index_unchecked`] and [`IGrid::external_position_to_position_unchecked`]
    unsafe fn index_to_position_unchecked(&self, index: usize) -> Vector<Idx,N> { unsafe { Vector::<Idx,N>::from_index_unchecked(index, self.size()) } }
    #[allow(unused_variables)]
    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    ///
    /// If you override this, you probably also want to override [`IGrid::position_to_index_unchecked`] and [`IGrid::index_to_position_unchecked`]
    unsafe fn external_position_to_position_unchecked<P>(pos: P, size: P) -> P where P: Into<Vector<Idx,N>> + From<Vector<Idx,N>> { pos }

    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    #[inline(always)]
    fn position_to_index<P>(&self, pos: P) -> Option<usize> where P: Into<Vector<Idx,N>> { let pos = pos.into(); pos.is_inside(self.size()).then(|| unsafe { self.position_to_index_unchecked(pos) }) }
    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    #[inline(always)]
    fn index_to_position(&self, index: usize) -> Option<Vector<Idx,N>> { (index < self.size().area_usize()).then(|| unsafe { self.index_to_position_unchecked(index) }) }



    fn from_vec<P>(size: P, values: Vec<T, Self::Allocator>) -> Option<Self> where P: Into<Vector<Idx,N>> { Self::try_from_vec(size, values).ok() }
    fn try_from_vec<P>(size: P, values: Vec<T, Self::Allocator>) -> Result<Self, GridBaseError<Idx,N>> where P: Into<Vector<Idx,N>>
    {
        let size = size.into();
        if size.iter().any(|dimension| *dimension < Idx::ZERO)
        {
            return Err(GridBaseError::NegativeSize(size));
        }

        let area_size = match size.area_usize_checked()
        {
            Some(v) => v,
            None => return Err(GridBaseError::ToBig(size)),
        };
        if area_size != values.len()
        {
            return Err(GridBaseError::WrongDimension(size, values.len()));
        }
        Ok(unsafe { Self::from_vec_unchecked(size, values) })
    }

    /// Builds a grid from its backing vector.
    ///
    /// # Safety
    /// The dimensions must be valid and their product must equal `values.len()`.
    unsafe fn from_vec_unchecked<P>(size: P, values: Vec<T, Self::Allocator>) -> Self where P: Into<Vector<Idx,N>>;

    fn sized_one(value: T) -> Self
    where
        Self::Allocator: Default,
    {
        Self::sized_one_in(value, Self::Allocator::default())
    }

    fn sized_one_in(value: T, allocator: Self::Allocator) -> Self
    {
        let mut values = Vec::with_capacity_in(1, allocator);
        values.push(value);
        Self::from_vec(Vector::<Idx, N>::ONE, values).unwrap()
    }

    /// Create a grid from a function using the default allocator.
    fn from_fn<P, F>(size: P, f: F) -> Self
    where
        F: FnMut(P) -> T,
        P: Into<Vector<Idx, N>>,
        Vector<Idx, N>: Into<P>,
        Self::Allocator: Default,
    {
        Self::from_fn_in(size, f, Self::Allocator::default())
    }

    /// Create a grid from a function using the supplied allocator.
    fn from_fn_in<P, F>(size: P, mut f: F, allocator: Self::Allocator) -> Self
    where
        F: FnMut(P) -> T,
        P: Into<Vector<Idx, N>>,
        Vector<Idx, N>: Into<P>,
    {
        let size = size.into();
        let mut values = Vec::with_capacity_in(size.area_usize_checked().expect("invalid grid size"), allocator);
        for index in size.iter_index()
        {
            let position = unsafe { Self::external_position_to_position_unchecked(index, size) };
            values.push(f(position.into()));
        }
        Self::try_from_vec(size, values).unwrap()
    }

    /// Create a grid from a function in parallel using the default allocator.
    fn from_fn_par<P, F>(size: P, f: F) -> Self
    where
        F: Fn(P) -> T + Sync,
        P: Into<Vector<Idx, N>>,
        Vector<Idx, N>: Into<P>,
        T: Send,
        Idx: Sync,
        Self::Allocator: Default,
    {
        Self::from_fn_par_in(size, f, Self::Allocator::default())
    }

    /// Create a grid from a function in parallel using the supplied allocator.
    ///
    /// Parallel generation uses a temporary buffer before moving the values into the grid.
    fn from_fn_par_in<P, F>(size: P, f: F, allocator: Self::Allocator) -> Self
    where
        F: Fn(P) -> T + Sync,
        P: Into<Vector<Idx, N>>,
        Vector<Idx, N>: Into<P>,
        T: Send,
        Idx: Sync,
    {
        let size = size.into();
        let area = size.area_usize_checked().expect("invalid grid size");
        let generated: Vec<T> = (0..area).into_par_iter().map(|index| {
            let position = unsafe {
                Self::external_position_to_position_unchecked(Vector::<Idx, N>::from_index_unchecked(index, size), size)
            };
            f(position.into())
        }).collect();
        let mut values = Vec::with_capacity_in(area, allocator);
        values.extend(generated);
        Self::try_from_vec(size, values).unwrap()
    }

    fn new(size: Vector<Idx, N>) -> Self
    where
        T: Default,
        Self::Allocator: Default,
    {
        Self::new_in(size, Self::Allocator::default())
    }

    fn new_in(size: Vector<Idx, N>, allocator: Self::Allocator) -> Self
    where
        T: Default,
    {
        Self::from_fn_in(size, |_| T::default(), allocator)
    }

    fn new_uniform(size: Vector<Idx, N>, value: T) -> Self
    where
        T: Clone,
        Self::Allocator: Default,
    {
        Self::new_uniform_in(size, value, Self::Allocator::default())
    }

    fn new_uniform_in(size: Vector<Idx, N>, value: T, allocator: Self::Allocator) -> Self
    where
        T: Clone,
    {
        let area = size.area_usize_checked().expect("invalid grid size");
        let mut values = Vec::with_capacity_in(area, allocator);
        values.resize(area, value);
        Self::try_from_vec(size, values).unwrap()
    }

    /// Create a grid from a Normalized Device Coordinates.
    ///
    /// Each component of the vector will be between `-1.0..=1.0`
    fn from_fn_ndc<P,F>(size: P, f: F) -> Self
    where
        Self::Allocator: Default,
        P: Into<Vector<Idx, N>>,
        float: CastFrom<Idx>,
        F: FnMut(Vector<float, N>) -> T,
    {
        Self::from_fn_ndc_with_precision(size, f)
    }

    /// Create a grid from a Normalized Device Coordinates.
    ///
    /// Each component of the vector will be between `-1.0..=1.0`
    fn from_fn_ndc_with_precision<P,Precision,F>(size: P, mut f: F) -> Self
    where
        Self::Allocator: Default,
        P: Into<Vector<Idx, N>>,
        Precision: Float + CastFrom<Idx>,
        F: FnMut(Vector<Precision,N>) -> T,
    {
        let size: Vector::<Idx,N> = size.into();
        let size_float: Vector::<Precision,N> = size.cast_into();

        Self::from_fn(size, |p|
            {
                let pos_float: Vector::<Precision,N> = p.cast_into();
                f(pos_float / size_float * Precision::TWO - Vector::<Precision,N>::ONE)
            })
    }





    /// Create a grid from a coefficients.
    ///
    /// Each component of the vector will be between `0..1.0`
    fn from_fn_coef<P,F>(size: P, f: F) -> Self
    where
        Self::Allocator: Default,
        P: Into<Vector<Idx, N>>,
        float: CastFrom<Idx>,
        F: FnMut(Vector<float, N>) -> T,
    {
        Self::from_fn_coef_with_precision(size, f)
    }

    /// Create a grid from a coefficients.
    ///
    /// Each component of the vector will be between `0..1.0`
    fn from_fn_coef_with_precision<P,Precision,F>(size: P, mut f: F) -> Self
    where
        Self::Allocator: Default,
        P: Into<Vector<Idx, N>>,
        Precision: Float + CastFrom<Idx>,
        F: FnMut(Vector<Precision,N>) -> T,
    {
        let size: Vector::<Idx,N> = size.into();
        let size_float: Vector::<Precision,N> = size.cast_into();

        Self::from_fn(size, |p|
            {
                let pos_float: Vector::<Precision,N> = p.cast_into();
                f(pos_float / size_float)
            })
    }

    /*
    fn map<Dest,F>(self, mut f: F) -> <Self as IGrid<T,Idx,N>>::WithType<Dest> where F: FnMut(T) -> Dest
    {
        let size = self.size();
        unsafe { <Self as IGrid<T,Idx,N>>::WithType::from_vec_unchecked(size, self.into_values().into_iter().map(|v| f(v)).collect()) }
    }
    */
}

impl<T, Idx, const N: usize, S> IGridView<Self, T, Idx, N> for S
where
    S: IGrid<T, Idx, N>,
    Idx: Integer,
{
    type WithLifetime<'a>
        = GridView<'a, Self, T, Idx, N>
    where
        Self: 'a;
    fn view<'a, 'b>(&'a self) -> Self::WithLifetime<'b>
    where
        'a: 'b,
    {
        GridView::new(self)
    }

    unsafe fn grid_unchecked(&self) -> &Self {
        self
    }

    unsafe fn subview_from_translated_rect_unchecked<'a, 'b>(&'a self, rect: Rectangle<Idx, N>) -> Self::WithLifetime<'b>
    where
        'a: 'b,
    {
        unsafe { GridView::from_rect_unchecked(self, rect) }
    }
}

impl<T, Idx, const N: usize, S> IGridViewMut<Self, T, Idx, N> for S
where
    S: IGrid<T, Idx, N>,
    Idx: Integer,
{
    type WithLifetimeMut<'a>
        = GridViewMut<'a, Self, T, Idx, N>
    where
        Self: 'a;
    fn view_mut<'a, 'b>(&'a mut self) -> Self::WithLifetimeMut<'b>
    where
        'a: 'b,
    {
        GridViewMut::new(self)
    }

    fn iter_mut(&mut self) -> GridViewIterMut<'_, Self, T, Idx, N> {
        GridViewIterMut::new(self)
    }
    fn for_each_mut<F>(&mut self, f: F)
    where
        F: FnMut((Vector<Idx, N>, &mut T)),
    {
        self.view_mut().for_each_mut(f)
    }

    unsafe fn grid_mut_unchecked(&mut self) -> &mut Self {
        self
    }
}

pub trait ToGrid<T, Idx, const N: usize>
where
    Idx: Integer,
{
    type Output: IGrid<T, Idx, N>;
    fn to_grid(self) -> Self::Output;
}

impl<T, Idx, const N: usize, A: Allocator + Clone> ToGrid<T, Idx, N> for GridOf<T, Idx, N, A>
where
    Idx: Integer,
{
    type Output = Self;
    fn to_grid(self) -> Self::Output {
        self
    }
}

impl<'a, T, Idx, const N: usize, A: Allocator + Clone> ToGrid<T, Idx, N> for GridView<'a, GridOf<T, Idx, N, A>, T, Idx, N>
where
    Idx: Integer,
    T: Clone,
{
    type Output = GridOf<T, Idx, N, A>;
    fn to_grid(self) -> Self::Output {
        let allocator = unsafe { self.grid_unchecked() }.allocator().clone();
        Self::Output::from_fn_in(self.size(), |index| unsafe { self.get_unchecked(index).clone() }, allocator)
    }
}

impl<'a, T, Idx, const N: usize, A: Allocator + Clone> ToGrid<T, Idx, N> for GridViewMut<'a, GridOf<T, Idx, N, A>, T, Idx, N>
where
    Idx: Integer,
    T: Clone,
{
    type Output = GridOf<T, Idx, N, A>;
    fn to_grid(self) -> Self::Output {
        let allocator = unsafe { self.grid_unchecked() }.allocator().clone();
        Self::Output::from_fn_in(self.size(), |index| unsafe { self.get_unchecked(index).clone() }, allocator)
    }
}
