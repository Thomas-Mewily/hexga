use crate::*;

pub trait ISliceMut<T, const N : usize, I> : ISlice<T,N,I> + IndexMut<Vector<I,N>,Output=T>
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    fn get_mut(&mut self, pos : Vector<I,N>) -> Option<&mut T>;
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<I,N>) -> &mut T { &mut self[pos] }

    fn subslice_mut<'a>(&'a mut self, rect : Rectangle<I, N>) -> SliceMut<'a,T,N,I>;

    fn swap(&mut self, pos_a : Vector<I,N>, pos_b : Vector<I,N>) -> bool;

    fn replace(&mut self, val : T, pos : Vector<I,N>) ->  Option<T> { self.get_mut(pos).map(|v| std::mem::replace(v, val)) }
    /// Do nothings if the index is outside the range
    fn set(&mut self, val : T, pos : Vector<I,N>) -> &mut Self { self.get_mut(pos).map(|v| *v = val); self }
}


/// A mutable slice inside a [Grid]
#[derive(Debug)]
pub struct SliceMut<'a, T, const N : usize, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    grid : &'a mut GridBase<T,N,I>,
    view : Rectangle<I,N>,
}

impl<'a, T, const N : usize, I> SliceMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    pub fn from_grid(grid : &'a mut GridBase<T,N,I>) -> Self 
    {
        let view = grid.rect();
        Self { grid, view }
    }
    pub fn new(grid : &'a mut GridBase<T,N,I>, view : Rectangle<I,N>) -> Self 
    {
        let view = grid.rect().intersect_or_empty(view);
        Self { grid, view }
    }
    pub unsafe fn new_unchecked(grid : &'a mut GridBase<T,N,I>, view : Rectangle<I,N>) -> Self 
    {
        Self { grid, view }
    }
}

impl<'a, T, const N : usize, I> ISlice<T,N,I> for SliceMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn get(&self, pos : Vector<I,N>) -> Option<&T> { self.grid.get(self.view.pos + pos) }
    fn size(&self) -> Vector<I,N> { self.view.size() }
    fn begin(&self) -> Vector<I,N> { self.view.pos }
    fn subslice<'b>(&'b self, rect : Rectangle<I, N>) -> Slice<'b,T,N,I> { Slice::new(self.grid, self.view.intersect_or_empty(rect.moved_by(self.position()))) }
    
    unsafe fn get_unchecked(&self, pos : Vector<I,N>) -> &T { unsafe { self.grid.get_unchecked(pos) } }
}

impl<'a, T, const N : usize, I> ISliceMut<T,N,I> for SliceMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn get_mut(&mut self, pos : Vector<I,N>) -> Option<&mut T> { self.grid.get_mut(self.view.pos + pos) }
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<I,N>) -> &mut T { unsafe { self.grid.get_unchecked_mut(self.view.pos + pos) } }
    fn subslice_mut<'b>(&'b mut self, rect : Rectangle<I, N>) -> SliceMut<'b,T,N,I> { SliceMut::new(self.grid, self.view.intersect_or_empty(rect.moved_by(self.position()))) }
    
    fn swap(&mut self, pos_a : Vector<I,N>, pos_b : Vector<I,N>) -> bool 
    {
        let offset = self.position();
        self.grid.swap(pos_a + offset, pos_b + offset)
    }
}

impl<'a, T, const N : usize, I> PartialEq for SliceMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>,
    T : PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() { return false; }
        self.size().iter_idx().all(|p| unsafe { self.get_unchecked(p) == other.get_unchecked(p) })
    }
}

impl<'a, T, const N : usize, I> Eq for SliceMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>, 
    T : Eq { }

impl<'a, T, const N : usize, I> Index<Vector<I,N>> for SliceMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    type Output=T;
    fn index(&self, index: Vector<I,N>) -> &Self::Output { self.get(index).unwrap() }
}

impl<'a, T, const N : usize, I> IndexMut<Vector<I,N>> for SliceMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn index_mut(&mut self, index: Vector<I,N>) -> &mut Self::Output { self.get_mut(index).unwrap() }
}
