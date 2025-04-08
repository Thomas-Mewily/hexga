use crate::*;

pub trait ISliceMut<T, const N : usize, I> : ISlice<T,N,I> + IndexMut<Vector<I,N>>
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    fn get_mut(&mut self, pos : Vector<I,N>) -> Option<&mut T>;
}


/// A mutable slice inside a [Grid]
#[derive(PartialEq, Eq, Debug)]
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
}

impl<'a, T, const N : usize, I> ISlice<T,N,I> for SliceMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn get(&self, pos : Vector<I,N>) -> Option<&T> { self.grid.get(self.view.pos + pos) }
    fn size(&self) -> Vector<I,N> { self.view.size() }
    fn begin(&self) -> Vector<I,N> { self.view.pos }
}

impl<'a, T, const N : usize, I> ISliceMut<T,N,I> for SliceMut<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn get_mut(&mut self, pos : Vector<I,N>) -> Option<&mut T> {
        self.grid.get_mut(self.view.pos + pos)
    }
}

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
