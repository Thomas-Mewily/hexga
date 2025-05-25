use crate::*;



/// A N dimensional grid
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct GridBase<T, Idx, const N : usize> where Idx : Integer
{
    size  : Vector<Idx,N>,
    value : Vec<T>,
}

impl<T, Idx, const N : usize> IGrid<T, Idx, N> for GridBase<T, Idx, N> where Idx : Integer 
{
    fn values(&self) -> &[T] { &self.value }
    fn values_mut(&mut self) -> &mut [T] { &mut self.value}

    fn into_size_and_values(self) -> (Vector<Idx,N>, Vec<T>) {  (self.size, self.value) }

    unsafe fn unchecked_from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Self { Self { size, value } }
}



impl<T, Idx, const N : usize> IRectangle<Idx, N> for GridBase<T, Idx, N> where Idx : Integer,
{
    #[inline(always)]
    fn size(&self) -> Vector<Idx, N> { self.size }
    #[inline(always)]
    fn begin(&self) -> Vector<Idx,N> { zero() }

    fn iter_x(&self) -> Range<Idx> where Vector<Idx,N> : HaveX<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_x() }
    fn iter_y(&self) -> Range<Idx> where Vector<Idx,N> : HaveY<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_y() }
    fn iter_z(&self) -> Range<Idx> where Vector<Idx,N> : HaveZ<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_z() }
    fn iter_w(&self) -> Range<Idx> where Vector<Idx,N> : HaveW<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_w() }

    #[inline(always)] fn is_inside_x(&self, x : Idx) -> bool where Vector<Idx,N> : HaveX<Idx> { x >= Idx::ZERO && x < self.size_x() }
    #[inline(always)] fn is_inside_y(&self, y : Idx) -> bool where Vector<Idx,N> : HaveY<Idx> { y >= Idx::ZERO && y < self.size_y() }
    #[inline(always)] fn is_inside_z(&self, z : Idx) -> bool where Vector<Idx,N> : HaveZ<Idx> { z >= Idx::ZERO && z < self.size_z() }
    #[inline(always)] fn is_inside_w(&self, w : Idx) -> bool where Vector<Idx,N> : HaveW<Idx> { w >= Idx::ZERO && w < self.size_w() }
}

impl<T, Idx, const N : usize> IGridViewMut<T,(),Idx,N> for GridBase<T, Idx, N> 
    where Idx : Integer 
{
    type SubViewMut<'b> = GridViewMut<'b,T,Idx,N> where Self: 'b;
    fn subview_mut<'a>(&'a mut self, rect : Rectangle<Idx, N>) -> Self::SubViewMut<'a> { GridViewMut::new_intersect(self, rect) }
}

impl<T, Idx, const N : usize> Get<Vector<Idx,N>> for GridBase<T, Idx,N>  where Idx : Integer 
{
    type Output = <Self as Index<Vector<Idx,N>>>::Output;
    #[inline(always)]
    fn try_get(&self, pos : Vector<Idx,N>) -> Result<&Self::Output, ()> { self.get(pos).ok_or_void() }
    #[inline(always)]
    fn get(&self, pos : Vector<Idx,N>) -> Option<&Self::Output> { self.position_to_index(pos).and_then(|idx| self.get(idx)) }
    #[inline(always)]
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &Self::Output { unsafe { let idx = self.position_to_index_unchecked(pos); self.get_unchecked(idx) } }
}

impl<T, Idx, const N : usize> GetMut<Vector<Idx,N>> for GridBase<T, Idx,N> where Idx : Integer 
{
    #[inline(always)]
    fn try_get_mut(&mut self, pos : Vector<Idx,N>) -> Result<&mut Self::Output, ()> { self.get_mut(pos).ok_or_void() }
    #[inline(always)]
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut Self::Output> { self.position_to_index(pos).and_then(|i| self.get_mut(i)) }
    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut Self::Output{ unsafe { let idx = self.position_to_index_unchecked(pos); self.values_mut().get_unchecked_mut(idx)} }
}

impl<T, Idx, const N : usize> GetManyMut<Vector<Idx,N>> for GridBase<T, Idx,N> where Idx : Integer 
{
    #[inline(always)]
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [Vector<Idx,N>; N2]) -> Result<[&mut Self::Output;N2], ()> {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        let indices = indices.map(|pos| self.position_to_index(pos));
        if indices.any(|x| x.is_none()) 
        {
            Err(())
        } else 
        {
            self.try_get_many_mut(indices.map(|idx| idx.unwrap()))
        }
    }
}

impl<T, Idx, const N : usize> Get<usize> for GridBase<T, Idx,N> 
    where Idx : Integer 
{
    type Output = <Self as Index<usize>>::Output;
    #[inline(always)]
    fn try_get(&self, index : usize) -> Result<&T, ()> { self.get(index).ok_or_void() }
    #[inline(always)]
    fn get(&self, index : usize) -> Option<&T> { self.values().get(index) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, index : usize) -> &T { unsafe { self.values().get_unchecked(index) } }
}

impl<T, Idx, const N : usize> GetMut<usize> for GridBase<T, Idx,N> where Idx : Integer 
{
    #[inline(always)]
    fn try_get_mut(&mut self, index : usize) -> Result<&mut T, ()> { self.get_mut(index).ok_or_void() }
    #[inline(always)]
    fn get_mut(&mut self, index : usize) -> Option<&mut T> { self.values_mut().get_mut(index) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(&mut self, index : usize) -> &mut T{ unsafe { self.values_mut().get_unchecked_mut(index)} }
}

impl<T, Idx, const N : usize> GetManyMut<usize> for GridBase<T, Idx,N> where Idx : Integer 
{
    #[inline(always)]
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [usize; N2]) -> Result<[&mut Self::Output;N2], ()> {
        self.values_mut().try_get_many_mut(indices)
    }
}

impl<T, Idx, const N : usize> Index<usize> for GridBase<T, Idx, N> where Idx : Integer
{
    type Output=T;
    #[inline(always)]
    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output { self.get_or_panic(index) }
}
impl<T, Idx, const N : usize> IndexMut<usize> for GridBase<T, Idx, N> where Idx : Integer
{
    #[inline(always)]
    #[track_caller]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_mut_or_panic(index) }
}

impl<T, Idx, const N : usize> Index<Vector<Idx,N>> for GridBase<T, Idx, N> where Idx : Integer
{
    type Output=T;
    #[inline(always)]
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get_or_panic(index) }
}
impl<T, Idx, const N : usize> IndexMut<Vector<Idx,N>> for GridBase<T, Idx, N> where Idx : Integer
{
    #[inline(always)]
    fn index_mut(&mut self, index: Vector<Idx,N>) -> &mut Self::Output { self.get_mut_or_panic(index) }
}

impl<T, Idx, const N : usize> Length for GridBase<T, Idx, N> 
    where Idx : Integer
{
    #[inline(always)]
    fn len(&self) -> usize { self.values().len() }
}