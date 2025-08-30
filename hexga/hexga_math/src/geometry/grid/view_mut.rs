use crate::*;

use crate::*;


pub trait IGridViewMut<G, T, Idx, const N : usize> :
      IGridView<G,T,Idx,N>
    + GetMut<Vector<Idx,N>,Output = T> + GetManyMut<Vector<Idx,N>,Output=T>
    + IndexMut<Vector<Idx,N>, Output = T>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    type WithLifetimeMut<'a> : IGridViewMut<G,T,Idx,N> where Self: 'a;
    fn view_mut<'a,'b>(&'a mut self) -> Self::WithLifetimeMut<'b> where 'a : 'b;

    fn iter_mut(&mut self) -> GridViewIterMut<'_, G, T, Idx, N>;
    fn for_each_mut<F>(&mut self, f : F) where F : FnMut((Vector<Idx,N>,&mut T));

    fn fill_fn<F>(&mut self, mut f : F) where F : FnMut(Vector::<Idx,N>) -> T
    {
        self.for_each_mut(|(idx,v)| { *v = f(idx); })
    }
    //fn fill_fn_par<F>(&mut self, f : F) -> Self where F : Fn(Vector::<Idx,N>) -> T + Sync, T : Send, Idx : Sync;

    fn fill_uniform(&mut self, value : T) where T: Clone { self.for_each_mut(|(_,v)| { *v = value.clone(); }); }

    fn update_from_or_panic<O,G2>(&mut self, other : &O)
            where
        O : IGridView<G2,T,Idx,N>,
        G2 : IGrid<T,Idx,N>,
        T : Clone,
        { self.update_from(other).unwrap() }

    fn update_from<O,G2>(&mut self, other : &O) -> Result<(), ()>
        where
        O : IGridView<G2,T,Idx,N>,
        G2 : IGrid<T,Idx,N>,
        T : Clone,
    {
        if self.size() != other.size() { return Err(()); }

        for idx in self.size().iter_index()
        {
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
pub struct GridViewMut<'a, G, T, Idx, const N : usize>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    grid : &'a mut G,
    rect : Rectangle<Idx,N>,
    phantom : std::marker::PhantomData<T>,
}

impl<'c, G, T, Idx, const N : usize> IGridView<G,T,Idx,N> for GridViewMut<'c, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    type WithLifetime<'a> = GridView<'a, G, T, Idx, N> where Self: 'a;
    fn view<'a,'b>(&'a self) -> Self::WithLifetime<'b> where 'a : 'b { unsafe { GridView::from_rect_unchecked(self.grid_unchecked(), self.rect()) } }

    unsafe fn grid_unchecked(&self) -> &G { self.grid }
    unsafe fn subview_from_translated_rect_unchecked<'a, 'b>(&'a self, rect: Rectangle<Idx, N>) -> Self::WithLifetime<'b> where 'a: 'b
    { unsafe { GridView::from_rect_unchecked(self.grid_unchecked(), rect) } }
}

/*
impl<'c, G, T, Idx, const N : usize> IGridViewNonOwned<G, T, Idx, N> for GridViewMut<'c, G, T, Idx, N> where
    G : IGrid<T, Idx, N>,
    Idx : Integer {}
*/

impl<'c, G, T, Idx, const N : usize> IGridViewMut<G,T,Idx,N> for GridViewMut<'c, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    type WithLifetimeMut<'a> = GridViewMut<'a, G, T, Idx, N> where Self: 'a;
    fn view_mut<'a,'b>(&'a mut self) -> Self::WithLifetimeMut<'b> where 'a : 'b { unsafe { let r = self.rect(); GridViewMut::from_rect_unchecked(self.grid_mut_unchecked(), r) } }


    fn iter_mut(&mut self) -> GridViewIterMut<'_, G, T, Idx, N> { GridViewIterMut::from_rect(self.grid, self.rect()).unwrap() }
    fn for_each_mut<F>(&mut self, f : F) where F : FnMut((Vector<Idx,N>,&mut T)) { self.iter_mut().for_each(f); }

    #[doc(hidden)]
    unsafe fn grid_mut_unchecked(&mut self) -> &mut G { self.grid }
}


impl<'a, G, T, Idx, const N : usize> Debug for GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    T : Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        f.debug_struct("GridViewMut").field("grid", &self.iter().map(|(_,v)| v).to_vec()).field("rect", &self.rect).finish()
    }
}

impl<'a, G, T, Idx, const N : usize> PartialEq for GridViewMut<'a, G, T, Idx, N>
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

impl<'a, G, T, Idx, const N : usize> Eq for GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    T : Eq
{}

impl<'a, G, T, Idx, const N : usize> Hash for GridViewMut<'a, G, T, Idx, N>
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

impl<'a, G, T, Idx, const N : usize> PartialOrd for GridViewMut<'a, G, T, Idx, N>
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

impl<'a, G, T, Idx, const N : usize> Ord for GridViewMut<'a, G, T, Idx, N>
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


impl<'a, G, T, Idx, const N : usize> GetPosition<Idx,N> for GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    /// The view is relative to itself, so the position is zero
    #[inline(always)]
    fn pos(&self) -> Vector<Idx,N> { zero() }
}
impl<'a, G, T, Idx, const N : usize> GetRectangle<Idx,N> for GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    #[inline(always)]
    fn size(&self) -> Vector<Idx,N> { self.rect.size }
    #[inline(always)]
    fn rect(&self) -> Rectangle<Idx, N> { self.rect }
}


impl<'a, P, G, T, Idx, const N : usize> TryGet<P> for GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    P : Into<Vector<Idx,N>>,
{
    type Error=<G as TryGet<Vector<Idx,N>>>::Error;

    #[inline(always)]
    fn try_get(&self, index : P) -> Result<&Self::Output, Self::Error> { self.grid.try_get(index.into() + self.rect.pos) }
}
impl<'a, P, G, T, Idx, const N : usize> Get<P> for GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    P : Into<Vector<Idx,N>>,
{
    type Output = <G as Get<Vector<Idx,N>>>::Output;

    #[inline(always)]
    fn get(&self, index : P) -> Option<&Self::Output> { self.grid.get(index.into() + self.rect.pos) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, index : P) -> &Self::Output { unsafe { self.grid.get_unchecked(index.into() + self.rect.pos) } }
}

impl<'a, P, G, T, Idx, const N : usize> Index<P> for GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    P : Into<Vector<Idx,N>>,
{
    type Output = <Self as Get<Vector<Idx,N>>>::Output;

    #[track_caller]
    fn index(&self, index: P) -> &Self::Output { self.get_or_panic(index) }
}


impl<'a, P, G, T, Idx, const N : usize> TryGetMut<P> for GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    P : Into<Vector<Idx,N>>,
{
    #[inline(always)]
    fn try_get_mut(&mut self, index : P) -> Result<&mut Self::Output, Self::Error> { self.grid.try_get_mut(index.into() + self.rect.pos) }
}

impl<'a, P, G, T, Idx, const N : usize> GetMut<P> for GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    P : Into<Vector<Idx,N>>,
{
    #[inline(always)]
    fn get_mut(&mut self, index : P) -> Option<&mut Self::Output> { self.grid.get_mut(index.into() + self.rect.pos) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(&mut self, index : P) -> &mut Self::Output { unsafe { self.grid.get_unchecked_mut(index.into() + self.rect.pos) } }
}

impl<'a, P, G, T, Idx, const N : usize> IndexMut<P> for GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    P : Into<Vector<Idx,N>>,
{
    #[inline(always)]
    #[track_caller]
    fn index_mut(&mut self, index: P) -> &mut Self::Output { self.get_mut_or_panic(index) }
}

impl<'a, P, G, T, Idx, const N : usize> GetManyMut<P> for GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer,
    P : Into<Vector<Idx,N>>,
{
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [P; N2]) -> Result<[&mut Self::Output;N2], ManyMutError> { self.grid.try_get_many_mut(indices.map(|idx| idx.into() + self.rect.pos)) }
    #[inline(always)]
    fn get_many_mut<const N2: usize>(&mut self, indices: [P; N2]) -> Option<[&mut Self::Output;N2]> { self.grid.try_get_many_mut(indices.map(|idx| idx.into() + self.rect.pos)).ok() }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<const N2: usize>(&mut self, indices: [P; N2]) -> [&mut Self::Output;N2] { self.grid.get_many_mut(indices.map(|idx| idx.into() + self.rect.pos)).expect("invalid index") }
}


impl<'a, G, T, Idx, const N : usize> GridViewMut<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    pub fn new(grid : &'a mut G) -> Self { unsafe { Self::from_rect_unchecked(grid, grid.rect()) } }

    pub fn from_rect(grid : &'a mut G, rect : Rectangle<Idx,N>) -> Option<Self>
    {
        if !grid.rect().is_rect_inside(rect) {
            return None;
        }else
        {
            Some(unsafe { Self::from_rect_unchecked(grid, rect) })
        }
    }
    pub unsafe fn from_rect_unchecked(grid : &'a mut G, rect : Rectangle<Idx,N>) -> Self { Self { grid, rect, phantom: PhantomData }}

    pub fn from_rect_intersect(grid : &'a mut G, rect : Rectangle<Idx,N>) -> Self
    {
        unsafe { Self::from_rect_unchecked(grid, grid.rect().intersect_or_empty(rect)) }
    }
}


impl<'a, G, T, Idx, const N : usize> Crop<Idx,N> for GridViewMut<'a, G, T, Idx, N>
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
impl<'a, 'b, G, T, Idx, const N : usize, Dest> CastIntoComposite<Dest> for GridViewMut<'a,G, T, Idx, N>
    where Idx : Integer, G : IGrid<T, Idx, N>, T : CastIntoComposite<Dest> + Copy
{
    type Output = G::WithType<T::Output>;
    fn cast_into_composite(self) -> Self::Output {
        self.transform(|v| v.cast_into_composite())
    }
}
impl<'a, G, T, Idx, const N : usize, Dest> CastRangeIntoComposite<Dest> for GridViewMut<'a,G, T, Idx, N>
    where Idx : Integer, G : IGrid<T, Idx, N>, T : CastRangeIntoComposite<Dest> + Copy
{
    type Output = G::WithType<T::Output>;
    fn cast_range_into_composite(self) -> Self::Output {
        self.transform(|v| v.cast_range_into_composite())
    }
}
impl<'a, G, T, Idx, const N : usize> ToUnsigned for GridViewMut<'a,G, T, Idx, N>
    where Idx : Integer, G : IGrid<T, Idx, N>, T : ToUnsigned + Copy
{
    type Output = G::WithType<T::Output>;

    fn to_unsigned(self) -> Self::Output {
        self.transform(|v| v.to_unsigned())
    }
}
impl<'a, G, T, Idx, const N : usize> ToSigned for GridViewMut<'a,G, T, Idx, N>
    where Idx : Integer, G : IGrid<T, Idx, N>, T : ToSigned + Copy
{
    type Output = G::WithType<T::Output>;

    fn to_signed(self) -> Self::Output {
        self.transform(|v| v.to_signed())
    }
}
impl<'a, G, T, Idx, const N : usize> Abs for GridViewMut<'a,G, T, Idx, N>
    where Idx : Integer, G : IGrid<T, Idx, N>, T : Abs + Copy
{
    type Output = G::WithType<T::Output>;
    fn abs(self) -> Self::Output {
        self.transform(|v| v.abs())
    }
}