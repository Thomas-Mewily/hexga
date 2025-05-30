use crate::*;

/// A N-dimensional grid
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename = "Grid"))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
pub struct GridBase<T, Idx, const N : usize> where Idx : Integer
{
    pub(crate) size   : Vector<Idx,N>,
    pub(crate) values : Vec<T>,
}


impl<T, Idx, const N : usize> IGrid<T, Idx, N> for GridBase<T, Idx, N> where Idx : Integer
{
    type WithType<U> = GridBase<U, Idx, N>;

    fn values(&self) -> &[T] { &self.values }
    fn values_mut(&mut self) -> &mut [T] { &mut self.values }

    fn into_size_and_values(self) -> (Vector<Idx,N>, Vec<T>) {  (self.size, self.values) }
    unsafe fn from_vec_unchecked(size : Vector::<Idx,N>, value : Vec<T>) -> Self { Self { size, values: value } }
}


impl<T, Idx, const N : usize> AsRef<[T]> for GridBase<T, Idx, N> where Idx : Integer
{
    fn as_ref(&self) -> &[T] { &self.values }
}
impl<T, Idx, const N : usize> AsMut<[T]> for GridBase<T, Idx, N> where Idx : Integer
{
    fn as_mut(&mut self) -> &mut [T] { &mut self.values }
}

impl<T, Idx, const N : usize> IRectangle<Idx, N> for GridBase<T, Idx, N> where Idx : Integer
{
    #[inline(always)]
    fn size(&self) -> Vector<Idx, N> { self.size }
    #[inline(always)]
    fn pos(&self) -> Vector<Idx,N> { zero() }

    fn iter_x(&self) -> Range<Idx> where Vector<Idx,N> : HaveX<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_x() }
    fn iter_y(&self) -> Range<Idx> where Vector<Idx,N> : HaveY<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_y() }
    fn iter_z(&self) -> Range<Idx> where Vector<Idx,N> : HaveZ<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_z() }
    fn iter_w(&self) -> Range<Idx> where Vector<Idx,N> : HaveW<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_w() }

    #[inline(always)] fn is_inside_x(&self, x : Idx) -> bool where Vector<Idx,N> : HaveX<Idx> { x >= Idx::ZERO && x < self.size_x() }
    #[inline(always)] fn is_inside_y(&self, y : Idx) -> bool where Vector<Idx,N> : HaveY<Idx> { y >= Idx::ZERO && y < self.size_y() }
    #[inline(always)] fn is_inside_z(&self, z : Idx) -> bool where Vector<Idx,N> : HaveZ<Idx> { z >= Idx::ZERO && z < self.size_z() }
    #[inline(always)] fn is_inside_w(&self, w : Idx) -> bool where Vector<Idx,N> : HaveW<Idx> { w >= Idx::ZERO && w < self.size_w() }
}


impl<T, Idx, const N : usize> Get<Vector<Idx,N>> for GridBase<T, Idx,N>  where Idx : Integer
{
    type Output = <Self as Index<Vector<Idx,N>>>::Output;
    #[inline(always)]
    fn try_get(&self, pos : Vector<Idx,N>) -> Result<&Self::Output, ()> { self.get(pos).ok_or_void() }
    #[inline(always)]
    fn get(&self, pos : Vector<Idx,N>) -> Option<&Self::Output> { self.position_to_index(pos).and_then(|idx| Some(unsafe { self.get_unchecked(idx)} ) ) }
    #[inline(always)]
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &Self::Output { unsafe { let idx = self.position_to_index_unchecked(pos); self.get_unchecked(idx) } }
}

impl<T, Idx, const N : usize> GetMut<Vector<Idx,N>> for GridBase<T, Idx,N> where Idx : Integer
{
    #[inline(always)]
    fn try_get_mut(&mut self, pos : Vector<Idx,N>) -> Result<&mut Self::Output, ()> { self.get_mut(pos).ok_or_void() }
    #[inline(always)]
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut Self::Output> { self.position_to_index(pos).and_then(|i| Some(unsafe { self.get_unchecked_mut(i) })) }
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


impl<T, Idx, const N : usize> Crop<Idx,N> for GridBase<T, Idx, N>
    where Idx : Integer,
    T : Clone
{
    fn crop(self, subrect : Rectangle<Idx, N>) -> Option<Self> {
        self.view().crop(subrect).map(|v| v.to_grid())
    }
}




#[cfg(test)]
mod grid_test
{
    use crate::other::point2;

    #[test]
    fn index_order()
    {
        use crate::*;
        //let x = Grid::<char>::new(point2(2, 4));
        let size = point2(2, 3);

        let grid = Grid2::from_fn(size, |p| p.x + 10 * p.y);

        /*
        dbg!(&grid);
        for y in (0..size.y).rev()
        {
            for x in 0..size.x
            {
                print!("{:2} ", grid[point2(x, y)]);
            }
            println!()
        }
        */

        let mut indice = 0;
        assert_eq!(grid[indice], grid[point2(0,0)]);
        assert_eq!(grid[indice], 0);

        indice += 1;
        assert_eq!(grid[indice], grid[point2(1,0)]);
        assert_eq!(grid[indice], 1);

        indice += 1;
        assert_eq!(grid[indice], grid[point2(0,1)]);
        assert_eq!(grid[indice], 10);

        indice += 1;
        assert_eq!(grid[indice], grid[point2(1,1)]);
        assert_eq!(grid[indice], 11);

        indice += 1;
        assert_eq!(grid[indice], grid[point2(0,2)]);
        assert_eq!(grid[indice], 20);

        indice += 1;
        assert_eq!(grid[indice], grid[point2(1,2)]);
        assert_eq!(grid[indice], 21);
    }

    #[test]
    fn out_of_range()
    {
        use crate::*;
        let grid = Grid2::from_fn(point2(2, 3), |_| 42);

        assert_eq!(grid.get(point2(0, 0)), Some(&42));
        assert_eq!(grid.get(point2(-1, 0)), None);
        assert_eq!(grid.get(point2(1, 0)), Some(&42));
        assert_eq!(grid.get(point2(2, 0)), None);

        assert_eq!(grid.get(point2(0, 0)), Some(&42));
        assert_eq!(grid.get(point2(0, -1)), None);
        assert_eq!(grid.get(point2(0, 2)), Some(&42));
        assert_eq!(grid.get(point2(0, 3)), None);
    }

    #[test]
    fn slice_cmp()
    {
        use crate::*;

        let size = point2(2, 3);
        let grid = Grid2::from_fn(size, |p|  p.x + 10 * p.y);

        assert_eq!(grid.size(), grid.view().size());

        let subgrid = grid.view().subgrid(size.to_rect());

        assert_eq!(grid, subgrid);

        let smaller_size = point2(1, 2);
        let smaller_grid = Grid2::from_fn(smaller_size, |p|  p.x + 10 * p.y);
        let smaller_grid_from_bigger_grid = grid.view().subgrid(smaller_size.to_rect());
        assert_eq!(smaller_grid, smaller_grid_from_bigger_grid);

        let top_right_grid_size = point2(1, 2);
        let offset = Vector2::ONE;
        let top_right_grid = Grid2::from_fn(smaller_size, |p|  { let p = p + offset; p.x + 10 * p.y });

        assert_eq!(top_right_grid, grid.subgrid(top_right_grid_size.to_rect().moved_by(offset)));
    }

    #[test]
    fn subslice()
    {
        use crate::*;

        let size = point2(2, 3);

        let mut grid1 = Grid2::from_fn(size, |p|  p.x + 10 * p.y);
        let mut grid2 = Grid2::from_fn(size, |p|  p.x + 10 * p.y);

        assert_eq!(grid1, grid2);
        assert_eq!(grid1.view(), grid2.view());
        assert_eq!(grid1.view_mut(), grid2.view_mut());

        grid2[point2(1, 0)] = 42;

        assert_ne!(grid1, grid2);
        assert_ne!(grid1.view(), grid2.view());
        assert_ne!(grid1.view_mut(), grid2.view_mut());

        let rect = rect2p(0, 0, 1, size.y);
        assert_eq!(grid1.view().crop_intersect(rect), grid2.view().crop_intersect(rect));
        assert_eq!(grid1.view_mut().crop_intersect(rect), grid2.view_mut().crop_intersect(rect));
        assert_eq!(grid1.view_mut().subview(rect), grid2.view_mut().subview(rect));
    }
}