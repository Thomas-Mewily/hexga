use crate::*;

/*
/// Just for [GridView] and [GridViewMut] that don't own the underlying grid
pub trait IGridViewNonOwned<G, T, Idx, const N : usize> :
      IRectangle<Idx,N>
    + Get<Vector<Idx,N>,Output=T>
    + Index<Vector<Idx,N>,Output=T>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
}
*/

/// Trait to a view into an N-dimensional grid.
///
/// Can only shrink / be cropped.
pub trait IGridView<G, T, Idx, const N : usize> :
      IRectangle<Idx,N>
    + Get<Vector<Idx,N>,Output=T>
    + Index<Vector<Idx,N>,Output=T>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    type WithLifetime<'a> : IGridView<G,T,Idx,N> where Self: 'a;
    fn view<'a,'b>(&'a self) -> Self::WithLifetime<'b> where 'a : 'b;

    fn subview<'a, 'b>(&'a self, mut rect : Rectangle<Idx,N>) -> Option<Self::WithLifetime<'b>> where 'a: 'b
    {
        rect.move_by(self.pos());
        if !self.rect().is_rect_inside(rect)
        {
            return None;
        }else
        {
            Some(unsafe { self.subview_from_translated_rect_unchecked(rect) })
        }
    }
    fn subview_intersect<'a,'b>(&'a self, mut rect : Rectangle<Idx,N>) -> Self::WithLifetime<'b> where 'a: 'b
    {
        rect.move_by(self.pos());
        unsafe { self.subview_from_translated_rect_unchecked(self.rect().intersect_or_empty(rect)) }
    }


    fn iter(&self) -> GridViewIter<'_, G, T, Idx, N> { unsafe { GridViewIter::from_rect_unchecked( self.grid_unchecked(), self.rect()) } }
    fn for_each<F>(&self, f : F) where F : FnMut((Vector<Idx,N>,&T)) { self.iter().for_each(f); }

    /// `self.crop_intersect(subrect).to_grid()`
    fn subgrid<'a,'b>(&'a self, subrect : Rectangle<Idx, N>) -> G
    where
        T : Clone,
        Self : Crop<Idx,N>,
        Self::WithLifetime<'b> : ToGrid<T,Idx,N,Output = G>, 'a: 'b
    {
        self.subview_intersect(subrect).to_grid()
    }

    fn transform<Dest, F>(&self, mut f : F) -> <G as IGrid<T, Idx, N>>::WithType<Dest>
        where F : FnMut(&T) -> Dest
    {
        <G as IGrid<T, Idx, N>>::WithType::<Dest>::from_fn(self.size(), |idx| f(unsafe { self.get_unchecked(idx) }))
    }

    fn transform_par<Dest, F>(&self, f : F) -> <G as IGrid<T, Idx, N>>::WithType<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, G : Sync, Self : Sync
    {
        <G as IGrid<T, Idx, N>>::WithType::<Dest>::from_fn_par(self.size(), |idx| f(unsafe { self.get_unchecked(idx) }))
    }

    fn update_into_or_panic<O,G2>(&self, other : &mut O)
        where
        O : IGridViewMut<G2,T,Idx,N>,
        G2 : IGrid<T,Idx,N>,
        T : Clone,
        Self : Sized
    { self.update_into(other).unwrap() }
    fn update_into<O,G2>(&self, other : &mut O) -> Result<(), ()>
        where
        O : IGridViewMut<G2,T,Idx,N>,
        G2 : IGrid<T,Idx,N>,
        T : Clone,
        Self : Sized
    {
        other.update_from(self)
    }


    #[doc(hidden)]
    unsafe fn grid_unchecked(&self) -> &G;

    #[doc(hidden)]
    unsafe fn subview_from_translated_rect_unchecked<'a, 'b>(&'a self, rect: Rectangle<Idx, N>) -> Self::WithLifetime<'b> where 'a: 'b;
}


/// A view to a N-dimensional grid.
///
/// Can only shrink / be cropped.
pub struct GridView<'a, G, T, Idx, const N : usize>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    grid : &'a G,
    rect : Rectangle<Idx,N>,
    phantom : std::marker::PhantomData<T>,
}

/*
impl<'c, G, T, Idx, const N : usize> IGridViewNonOwned<G, T, Idx, N> for GridView<'c, G, T, Idx, N> where
    G : IGrid<T, Idx, N>,
    Idx : Integer {}
*/

impl<'c, G, T, Idx, const N : usize> IGridView<G, T, Idx, N> for GridView<'c, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    type WithLifetime<'a> = GridView<'a, G, T, Idx, N> where Self: 'a;
    fn view<'a,'b>(&'a self) -> Self::WithLifetime<'b> where 'a : 'b { unsafe { GridView::from_rect_unchecked(self.grid, self.rect) } }

    unsafe fn grid_unchecked(&self) -> &G { self.grid }
    unsafe fn subview_from_translated_rect_unchecked<'a, 'b>(&'a self, rect: Rectangle<Idx, N>) -> Self::WithLifetime<'b> where 'a: 'b
    { unsafe { GridView::from_rect_unchecked(self.grid, rect) } }
}

impl<'a, G, T, Idx, const N : usize> Debug for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    T : Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        f.debug_struct("GridView").field("grid", &self.iter().map(|(_,v)| v).to_vec()).field("rect", &self.rect).finish()
    }
}


impl<'a, G, T, Idx, const N : usize> Copy for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{}
impl<'a, G, T, Idx, const N : usize> Clone for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    fn clone(&self) -> Self {
        Self {
            grid: self.grid,
            rect: self.rect.clone(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, G, T, Idx, const N : usize> PartialEq for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    T : PartialEq
{
    fn eq(&self, other: &Self) -> bool
    {
        self.rect == other.rect && self.iter().eq(other.iter())
    }
}

impl<'a, G, T, Idx, const N : usize> Eq for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    T : Eq
{}

impl<'a, G, T, Idx, const N : usize> Hash for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    T : Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rect.hash(state);
        for (_, v) in self.iter()
        {
            v.hash(state);
        }
    }
}

impl<'a, G, T, Idx, const N : usize> PartialOrd for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    T : PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rect == other.rect {
            self.iter().partial_cmp(other.iter())
        }else
        {
            self.rect.partial_cmp(&other.rect)
        }
    }
}

impl<'a, G, T, Idx, const N : usize> Ord for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    T : Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rect == other.rect {
            self.iter().cmp(other.iter())
        }else
        {
            self.rect.cmp(&other.rect)
        }
    }
}


impl<'a, G, T, Idx, const N : usize> IRectangle<Idx,N> for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    /// The view is relative to itself, so the position is zero
    #[inline(always)]
    fn pos(&self) -> Vector<Idx,N> { zero() }
    #[inline(always)]
    fn size(&self) -> Vector<Idx,N> { self.rect.size }
    #[inline(always)]
    fn rect(&self) -> Rectangle<Idx, N> { self.rect }
}

impl<'a, P, G, T, Idx, const N : usize> Get<P> for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    P : Into<Vector<Idx,N>>
{
    type Output = <G as Get<Vector<Idx,N>>>::Output;

    fn try_get(&self, index : P) -> Result<&Self::Output, ()> { self.grid.try_get(index.into() + self.rect.pos) }
    fn get(&self, index : P) -> Option<&Self::Output> { self.grid.get(index.into() + self.rect.pos) }
    unsafe fn get_unchecked(&self, index : P) -> &Self::Output { unsafe { self.grid.get_unchecked(index.into() + self.rect.pos) } }
}

impl<'a, P, G, T, Idx, const N : usize> Index<P> for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    P : Into<Vector<Idx,N>>
{
    type Output = <Self as Get<Vector<Idx,N>>>::Output;

    #[track_caller]
    fn index(&self, index: P) -> &Self::Output { self.get_or_panic(index) }
}


impl<'a, G, T, Idx, const N : usize> GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
{
    pub fn new(grid : &'a G) -> Self { unsafe { Self::from_rect_unchecked(grid, grid.rect()) } }

    pub fn from_rect(grid : &'a G, rect : Rectangle<Idx,N>) -> Option<Self>
    {
        if !grid.rect().is_rect_inside(rect) {
            return None;
        }else
        {
            Some(unsafe { Self::from_rect_unchecked(grid, rect) })
        }
    }
    pub unsafe fn from_rect_unchecked(grid : &'a G, rect : Rectangle<Idx,N>) -> Self { Self { grid, rect, phantom: PhantomData }}

    pub fn from_rect_intersect(grid : &'a G, rect : Rectangle<Idx,N>) -> Self
    {
        unsafe { Self::from_rect_unchecked(grid, grid.rect().intersect_or_empty(rect)) }
    }
}


impl<'a, G, T, Idx, const N : usize> Crop<Idx,N> for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    fn crop(self, subrect : Rectangle<Idx, N>) -> Option<Self>
    {
        self.rect.crop(subrect).map(|r| unsafe { Self::from_rect_unchecked(self.grid, r) })
    }
}


// I'm not sure about the `T : CastIntoComposite<Dest> + Copy` semantic for the moment, `&'a T : CastIntoComposite<Dest>` make more sens
impl<'a, 'b, G, T, Idx, const N : usize, Dest> CastIntoComposite<Dest> for GridView<'a,G, T, Idx, N>
    where Idx : Integer, G : IGrid<T, Idx, N>, T : CastIntoComposite<Dest> + Copy
{
    type Output = G::WithType<T::Output>;
    fn cast_into_composite(self) -> Self::Output {
        self.transform(|v| v.cast_into_composite())
    }
}
impl<'a, G, T, Idx, const N : usize, Dest> CastRangeIntoComposite<Dest> for GridView<'a,G, T, Idx, N>
    where Idx : Integer, G : IGrid<T, Idx, N>, T : CastRangeIntoComposite<Dest> + Copy
{
    type Output = G::WithType<T::Output>;
    fn cast_range_into_composite(self) -> Self::Output {
        self.transform(|v| v.cast_range_into_composite())
    }
}
impl<'a, G, T, Idx, const N : usize> ToUnsigned for GridView<'a,G, T, Idx, N>
    where Idx : Integer, G : IGrid<T, Idx, N>, T : ToUnsigned + Copy
{
    type Output = G::WithType<T::Output>;

    fn to_unsigned(self) -> Self::Output {
        self.transform(|v| v.to_unsigned())
    }
}
impl<'a, G, T, Idx, const N : usize> ToSigned for GridView<'a,G, T, Idx, N>
    where Idx : Integer, G : IGrid<T, Idx, N>, T : ToSigned + Copy
{
    type Output = G::WithType<T::Output>;

    fn to_signed(self) -> Self::Output {
        self.transform(|v| v.to_signed())
    }
}
impl<'a, G, T, Idx, const N : usize> Abs for GridView<'a,G, T, Idx, N>
    where Idx : Integer, G : IGrid<T, Idx, N>, T : Abs + Copy
{
    type Output = G::WithType<T::Output>;
    fn abs(self) -> Self::Output {
        self.transform(|v| v.abs())
    }
}