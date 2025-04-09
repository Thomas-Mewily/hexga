use crate::*;

/// A slice view to a [Grid]
//pub trait ISlice<'a, T:'a, const N : usize, I> : Index<Vector<I,N>,Output = T> + IntoIterator<Item = (Vector<I,N>, &'a T)>
pub trait ISlice<T, const N : usize, I> : Index<Vector<I,N>,Output = T> + IRectangle<I, N>
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    /* 
    // Can't be easily optimized (they will call `self.get(pos)`), so it is better to omit them
    fn get_index(&self, index : usize) -> Option<&T>;
    fn is_index_inside(&self, index : usize) -> bool { index < self.area().to_usize()  }
    fn is_index_outside(&self, index : usize) -> bool { !self.is_index_inside(index) }
    */

    /// The Zero is located at the `position()`/`begin()` 
    fn get(&self, pos : Vector<I,N>) -> Option<&T>;
    unsafe fn get_unchecked(&self, pos : Vector<I,N>) -> &T { &self[pos] }

    /// Clone this [Slice] into a [Grid]
    /// 
    /// Can't impl the trait [std::borrow::ToOwned] right now because the lifetime are impossible to express 
    fn to_grid(&self) -> GridBase<T,N,I> where T : Clone
    {
        GridBase::from_fn(self.size(), |p| self[p].clone())
    }

    fn subslice<'b>(&'b self, rect : Rectangle<I, N>) -> Slice<'b,T,N,I>;
    fn subgrid(&self, rect : Rectangle<I, N>) -> GridBase<T,N,I> where T : Clone { self.subslice(rect).to_grid() }
}

/* 
pub struct SliceIter
{

}
*/

/// A slice inside a [Grid]
#[derive(Clone, Debug, Copy)]
pub struct Slice<'a, T, const N : usize, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    grid : &'a GridBase<T,N,I>,
    view : Rectangle<I,N>,
}

impl<'a, T, const N : usize, I> PartialEq for Slice<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>,
    T : PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() { return false; }
        self.size().iter_idx().all(|p| unsafe { self.get_unchecked(p) == other.get_unchecked(p) })
    }
}

impl<'a, T, const N : usize, I> Eq for Slice<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>, 
    T : Eq { }

impl<'a, T, const N : usize, I> Slice<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    pub fn from_grid(grid : &'a GridBase<T,N,I>) -> Self 
    {
        Self { grid, view: grid.rect() }
    }
    pub fn new(grid : &'a GridBase<T,N,I>, view : Rectangle<I,N>) -> Self 
    {
        let view = grid.rect().intersect_or_empty(view);
        Self { grid, view }
    }
    pub unsafe fn new_unchecked(grid : &'a GridBase<T,N,I>, view : Rectangle<I,N>) -> Self 
    {
        Self { grid, view }
    }
}


impl<'a, T, const N : usize, I> ISlice<T,N,I> for Slice<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn get(&self, pos : Vector<I,N>) -> Option<&T> { self.grid.get(self.view.pos + pos) }
    unsafe fn get_unchecked(&self, pos : Vector<I,N>) -> &T { unsafe { self.grid.get_unchecked(self.view.pos + pos) } }

    fn subslice<'b>(&'b self, rect : Rectangle<I, N>) -> Slice<'b,T,N,I> { Slice::new(self.grid, self.view.intersect_or_empty(rect.moved_by(self.position()))) }
}

impl<'a, T, const N : usize, I> IRectangle<I,N> for Slice<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn begin(&self) -> Vector<I,N> { self.view.begin() }
    fn size (&self) -> Vector<I,N> { self.view.size()  }
}

impl<'a, T, const N : usize, I> Index<Vector<I,N>> for Slice<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    type Output=T;
    fn index(&self, index: Vector<I,N>) -> &Self::Output { self.get(index).unwrap() }
}


