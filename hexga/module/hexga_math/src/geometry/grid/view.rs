use crate::*;

/// A view to a N dimensional grid.
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
    fn pos(&self) -> Vector<Idx,N> { zero() }
    fn size(&self) -> Vector<Idx,N> { self.rect.size }
}

impl<'a, G, T, Idx, const N : usize> Get<Vector<Idx,N>> for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    type Output = <G as Get<Vector<Idx,N>>>::Output;

    fn try_get(&self, index : Vector<Idx,N>) -> Result<&Self::Output, ()> { self.grid.try_get(index + self.rect.pos) }
    fn get(&self, index : Vector<Idx,N>) -> Option<&Self::Output> { self.grid.get(index + self.rect.pos) }
    unsafe fn get_unchecked(&self, index : Vector<Idx,N>) -> &Self::Output { unsafe { self.grid.get_unchecked(index + self.rect.pos) } }
}

impl<'a, G, T, Idx, const N : usize> Index<Vector<Idx,N>> for GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
{
    type Output = <Self as Get<Vector<Idx,N>>>::Output;

    #[track_caller]
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get_or_panic(index) }
}


impl<'a, G, T, Idx, const N : usize> GridView<'a, G, T, Idx, N>
    where
    G : IGrid<T, Idx, N>,
    Idx : Integer
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


    pub fn subview(&self, mut rect : Rectangle<Idx,N>) -> Option<Self>
    {
        rect.move_by(self.rect.pos);
        if !self.rect().is_rect_inside(rect) {
            return None;
        }else
        {
            Some(unsafe { Self::from_rect_unchecked(self.grid, rect) })
        }
    }
    pub fn subview_intersect(&self, mut rect : Rectangle<Idx,N>) -> Self
    {
        rect.move_by(self.rect.pos);
        unsafe { Self::from_rect_unchecked(self.grid, self.rect().intersect_or_empty(rect)) }
    }

    pub fn to_grid(&self) -> G where T : Clone
    {
        G::from_fn(self.size(), |idx| unsafe { self.get_unchecked(idx).clone() })
    }

    /// `self.crop_intersect(subrect).to_grid()`
    pub fn subgrid(&self, subrect : Rectangle<Idx, N>) -> G where T : Clone, Self : Crop<Idx,N> { self.subview_intersect(subrect).to_grid() }

    pub fn transform<Dest, F>(&self, mut f : F) -> <G as IGrid<T, Idx, N>>::WithType<Dest>
        where F : FnMut(&T) -> Dest
    {
        <G as IGrid<T, Idx, N>>::WithType::<Dest>::from_fn(self.size(), |idx| f(unsafe { self.get_unchecked(idx) }))
    }

    pub fn transform_par<Dest, F>(&self, f : F) -> <G as IGrid<T, Idx, N>>::WithType<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, G : Sync
    {
        <G as IGrid<T, Idx, N>>::WithType::<Dest>::from_fn_par(self.size(), |idx| f(unsafe { self.get_unchecked(idx) }))
    }

    pub fn iter(&self) -> GridViewIter<'_, G, T, Idx, N> { GridViewIter::from_rect(self.grid, self.rect()).unwrap() }
    pub fn for_each<F>(&self, f : F) where F : FnMut((Vector<Idx,N>,&T)) { self.iter().for_each(f); }
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
