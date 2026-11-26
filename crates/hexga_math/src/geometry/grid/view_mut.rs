use super::*;

pub trait IGridViewMut<G, T, Idx, const N: usize>:
    IGridView<G, T, Idx, N>
    + GetMut<Vector<Idx, N>, Output = T>
    + TryGetMut<Vector<Idx, N>, Output = T>
    + GetManyMut<Vector<Idx, N>, Output = T>
    + IndexMut<Vector<Idx, N>, Output = T>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
{
    type WithLifetimeMut<'a>: IGridViewMut<G, T, Idx, N>
    where
        Self: 'a;
    fn view_mut<'a, 'b>(&'a mut self) -> Self::WithLifetimeMut<'b>
    where
        'a: 'b;

    fn iter_mut(&mut self) -> GridViewIterMut<'_, G, T, Idx, N>;
    fn for_each_mut<F>(&mut self, f: F)
    where
        F: FnMut((Vector<Idx, N>, &mut T));

    fn fill_fn<F>(&mut self, mut f: F)
    where
        F: FnMut(Vector<Idx, N>) -> T,
    {
        self.for_each_mut(|(idx, v)| {
            *v = f(idx);
        })
    }
    //fn fill_fn_par<F>(&mut self, f : F) -> Self where F : Fn(Vector::<Idx,N>) -> T + Sync, T : Send, Idx : Sync;

    fn fill_uniform(&mut self, value: T)
    where
        T: Clone,
    {
        self.for_each_mut(|(_, v)| {
            *v = value.clone();
        });
    }

    fn update_from_or_panic<O, G2>(&mut self, other: &O)
    where
        O: IGridView<G2, T, Idx, N>,
        G2: IGrid<T, Idx, N>,
        T: Clone,
    {
        self.update_from(other).unwrap()
    }

    fn update_from<O, G2>(&mut self, other: &O) -> Result<(), ()>
    where
        O: IGridView<G2, T, Idx, N>,
        G2: IGrid<T, Idx, N>,
        T: Clone,
    {
        if self.size() != other.size() {
            return Err(());
        }

        for idx in self.size().iter_index() {
            self[idx] = other[idx].clone();
        }

        Ok(())
    }

    #[doc(hidden)]
    unsafe fn grid_mut_unchecked(&mut self) -> &mut G;
}

/// A mutable view to a N-dimensional grid.
///
/// Can only shrink / be cropped.
pub struct GridViewMut<'a, G, T, Idx, const N: usize>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
{
    grid: &'a mut G,
    rect: Rectangle<Idx, N>,
    phantom: std::marker::PhantomData<T>,
}

impl<'c, G, T, Idx, const N: usize> IGridView<G, T, Idx, N> for GridViewMut<'c, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
{
    type WithLifetime<'a>
        = GridView<'a, G, T, Idx, N>
    where
        Self: 'a;
    fn view<'a, 'b>(&'a self) -> Self::WithLifetime<'b>
    where
        'a: 'b,
    {
        unsafe { GridView::from_rect_unchecked(self.grid_unchecked(), self.rect()) }
    }

    unsafe fn grid_unchecked(&self) -> &G {
        self.grid
    }
    unsafe fn subview_from_translated_rect_unchecked<'a, 'b>(&'a self, rect: Rectangle<Idx, N>) -> Self::WithLifetime<'b>
    where
        'a: 'b,
    {
        unsafe { GridView::from_rect_unchecked(self.grid_unchecked(), rect) }
    }
}

/*
impl<'c, G, T, Idx, const N : usize> IGridViewNonOwned<G, T, Idx, N> for GridViewMut<'c, G, T, Idx, N> where
    G : IGrid<T, Idx, N>,
    Idx : Integer {}
*/

impl<'c, G, T, Idx, const N: usize> IGridViewMut<G, T, Idx, N> for GridViewMut<'c, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
{
    type WithLifetimeMut<'a>
        = GridViewMut<'a, G, T, Idx, N>
    where
        Self: 'a;
    fn view_mut<'a, 'b>(&'a mut self) -> Self::WithLifetimeMut<'b>
    where
        'a: 'b,
    {
        unsafe {
            let r = self.rect();
            GridViewMut::from_rect_unchecked(self.grid_mut_unchecked(), r)
        }
    }

    fn iter_mut(&mut self) -> GridViewIterMut<'_, G, T, Idx, N> {
        GridViewIterMut::from_rect(self.grid, self.rect()).unwrap()
    }
    fn for_each_mut<F>(&mut self, f: F)
    where
        F: FnMut((Vector<Idx, N>, &mut T)),
    {
        self.iter_mut().for_each(f);
    }

    #[doc(hidden)]
    unsafe fn grid_mut_unchecked(&mut self) -> &mut G {
        self.grid
    }
}

impl<'a, G, T, Idx, const N: usize> PartialEq for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.rect == other.rect && self.iter().eq(other.iter())
    }
}

impl<'a, G, T, Idx, const N: usize> Eq for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    T: Eq,
{
}

impl<'a, G, T, Idx, const N: usize> Hash for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rect.hash(state);
        for (_, v) in self.iter() {
            v.hash(state);
        }
    }
}

impl<'a, G, T, Idx, const N: usize> PartialOrd for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rect == other.rect {
            self.iter().partial_cmp(other.iter())
        } else {
            self.rect.partial_cmp(&other.rect)
        }
    }
}

impl<'a, G, T, Idx, const N: usize> Ord for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rect == other.rect {
            self.iter().cmp(other.iter())
        } else {
            self.rect.cmp(&other.rect)
        }
    }
}

impl<'a, G, T, Idx, const N: usize> GetPosition<Idx, N> for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
{
    /// The view is relative to itself, so the position is zero
    #[inline(always)]
    fn pos(&self) -> Vector<Idx, N> {
        zero()
    }
}
impl<'a, G, T, Idx, const N: usize> GetSize<Idx, N> for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
{
    #[inline(always)]
    fn size(&self) -> Vector<Idx, N> {
        self.rect.size
    }
}

impl<'a, G, T, Idx, const N: usize> CollectionBijective for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
{
}
impl<'s, G, T, Idx, const N: usize, Output: ?Sized> Collection<Output> for GridViewMut<'s, G, T, Idx, N>
where
    G: IGrid<T, Idx, N> + Collection<Output>,
    Idx: Integer,
{
    type Ref<'a>
        = <G as Collection<Output>>::Ref<'a>
    where
        Self: 'a,
        Output: 'a;
}
impl<'s, G, T, Idx, const N: usize, Output: ?Sized> CollectionMut<Output> for GridViewMut<'s, G, T, Idx, N>
where
    G: IGrid<T, Idx, N> + CollectionMut<Output>,
    Idx: Integer,
{
    type RefMut<'a>
        = <G as CollectionMut<Output>>::RefMut<'a>
    where
        Self: 'a,
        Output: 'a;
}

impl<'a, P, G, T, Idx, const N: usize> CollectionIndex<P> for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    type Output = <G as CollectionIndex<Vector<Idx, N>>>::Output;
    type Ref<'b>
        = <G as CollectionIndex<Vector<Idx, N>>>::Ref<'b>
    where
        Self: 'b,
        Self::Output: 'b;
}
impl<'a, P, G, T, Idx, const N: usize> Get<P> for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    #[inline(always)]
    fn get<'b>(&'b self, index: P) -> Option<Self::Ref<'b>>
    where
        Self::Output: 'b,
    {
        self.grid.get(index.into() + self.rect.pos)
    }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked<'b>(&'b self, index: P) -> Self::Ref<'b>
    where
        Self::Output: 'b,
    {
        unsafe { self.grid.get_unchecked(index.into() + self.rect.pos) }
    }
}
impl<'a, P, G, T, Idx, const N: usize> TryGet<P> for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    type Error = <G as TryGet<Vector<Idx, N>>>::Error;

    #[inline(always)]
    fn try_get<'b>(&'b self, index: P) -> Result<Self::Ref<'b>, Self::Error>
    where
        Self::Output: 'b,
    {
        self.grid.try_get(index.into() + self.rect.pos)
    }
}

impl<'a, P, G, T, Idx, const N: usize> Index<P> for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    type Output = <Self as CollectionIndex<Vector<Idx, N>>>::Output;

    #[track_caller]
    fn index(&self, index: P) -> &Self::Output {
        self.grid.index(index.into() + self.rect.pos)
    }
}

impl<'a, P, G, T, Idx, const N: usize> TryGetMut<P> for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    #[inline(always)]
    fn try_get_mut<'b>(&'b mut self, index: P) -> Result<Self::RefMut<'b>, Self::Error>
    where
        Self::Output: 'b,
    {
        self.grid.try_get_mut(index.into() + self.rect.pos)
    }
}

impl<'a, P, G, T, Idx, const N: usize> CollectionIndexMut<P> for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    type RefMut<'b>
        = <G as CollectionIndexMut<Vector<Idx, N>>>::RefMut<'b>
    where
        Self: 'b,
        Self::Output: 'b;
}
impl<'a, P, G, T, Idx, const N: usize> GetMut<P> for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    #[inline(always)]
    fn get_mut<'b>(&'b mut self, index: P) -> Option<Self::RefMut<'b>>
    where
        Self::Output: 'b,
    {
        self.grid.get_mut(index.into() + self.rect.pos)
    }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut<'b>(&'b mut self, index: P) -> Self::RefMut<'b>
    where
        Self::Output: 'b,
    {
        unsafe { self.grid.get_unchecked_mut(index.into() + self.rect.pos) }
    }
}

impl<'a, P, G, T, Idx, const N: usize> IndexMut<P> for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    #[inline(always)]
    #[track_caller]
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        self.grid.index_mut(index.into() + self.rect.pos)
    }
}

impl<'a, P, G, T, Idx, const N: usize> GetManyMut<P> for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    fn try_get_many_mut<'b, const N2: usize>(&'b mut self, indices: [P; N2]) -> Result<[Self::RefMut<'b>; N2], ManyMutError>
    where
        Self::Output: 'b,
    {
        self.grid.try_get_many_mut(indices.map(|idx| idx.into() + self.rect.pos))
    }
    #[inline(always)]
    fn get_many_mut<'b, const N2: usize>(&'b mut self, indices: [P; N2]) -> Option<[Self::RefMut<'b>; N2]>
    where
        Self::Output: 'b,
    {
        self.grid.try_get_many_mut(indices.map(|idx| idx.into() + self.rect.pos)).ok()
    }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<'b, const N2: usize>(&'b mut self, indices: [P; N2]) -> [Self::RefMut<'b>; N2]
    where
        Self::Output: 'b,
    {
        self.grid.get_many_mut(indices.map(|idx| idx.into() + self.rect.pos)).expect("invalid index")
    }
}

impl<'a, G, T, Idx, const N: usize> GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
{
    pub fn new(grid: &'a mut G) -> Self {
        unsafe { Self::from_rect_unchecked(grid, grid.rect()) }
    }

    pub fn from_rect(grid: &'a mut G, rect: Rectangle<Idx, N>) -> Option<Self> {
        if !grid.rect().is_rect_inside(rect) {
            None
        } else {
            Some(unsafe { Self::from_rect_unchecked(grid, rect) })
        }
    }
    pub unsafe fn from_rect_unchecked(grid: &'a mut G, rect: Rectangle<Idx, N>) -> Self {
        Self {
            grid,
            rect,
            phantom: PhantomData,
        }
    }

    pub fn from_rect_intersect(grid: &'a mut G, rect: Rectangle<Idx, N>) -> Self {
        unsafe { Self::from_rect_unchecked(grid, grid.rect().intersect_or_empty(rect)) }
    }
}

impl<'a, G, T, Idx, const N: usize> Crop<Idx, N> for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
{
    fn crop(self, subrect: Rectangle<Idx, N>) -> Option<Self> {
        self.rect.crop(subrect).map(|r| unsafe { Self::from_rect_unchecked(self.grid, r) })
    }
}
