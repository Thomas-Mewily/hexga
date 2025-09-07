use crate::*;

/// A N-dimensional grid
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename = "Grid"))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
pub struct GridBase<T, Idx, const N : usize> where Idx : Integer
{
    // The size is will be deserialized first before the values,
    // then we can give a hint to serde about the size of the vector to alloc when deserializing
    pub(crate) size   : Vector<Idx,N>,
    pub(crate) values : Vec<T>,
}

impl<T, Idx, const N : usize> Composite for GridBase<T, Idx, N> where Idx : Integer
{
    type Inside=T;
    fn map_intern<F>(mut self, f: F) -> Self where F: FnMut(Self::Inside) -> Self::Inside {
        self.values = self.values.map_intern(f);
        self
    }
}
impl<T, Idx, const N : usize> CompositeGeneric for GridBase<T, Idx, N> where Idx : Integer
{
    type WithType<T2> = GridBase<T2, Idx, N>;
    type Inside=T;

    fn map<T2,F>(self, f: F) -> Self::WithType<T2> where F: FnMut(Self::Inside) -> T2 {
        unsafe { Self::WithType::<T2>::from_vec_unchecked(self.size, self.values.map(f)) }
    }
}

impl<T, Idx, const N : usize> IGrid<T, Idx, N> for GridBase<T, Idx, N> where Idx : Integer
{
    type WithType<U> = GridBase<U, Idx, N>;

    fn values(&self) -> &[T] { &self.values }
    fn values_mut(&mut self) -> &mut [T] { &mut self.values }

    fn into_size_and_values(self) -> (Vector<Idx,N>, Vec<T>) {  (self.size, self.values) }
    unsafe fn from_vec_unchecked<P>(size : P, value : Vec<T>) -> Self where P : Into<Vector::<Idx,N>> { Self { size : size.into(), values: value } }
}


impl<T, Idx, const N : usize> GetPosition<Idx, N> for GridBase<T, Idx, N> where Idx : Integer
{
    #[inline(always)]
    fn pos(&self) -> Vector<Idx,N> { zero() }
}
impl<T, Idx, const N : usize> GetRectangle<Idx, N> for GridBase<T, Idx, N> where Idx : Integer
{
    #[inline(always)]
    fn size(&self) -> Vector<Idx, N> { self.size }

    fn iter_x(&self) -> Range<Idx> where Vector<Idx,N> : HaveX<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_x() }
    fn iter_y(&self) -> Range<Idx> where Vector<Idx,N> : HaveY<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_y() }
    fn iter_z(&self) -> Range<Idx> where Vector<Idx,N> : HaveZ<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_z() }
    fn iter_w(&self) -> Range<Idx> where Vector<Idx,N> : HaveW<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_w() }

    #[inline(always)] fn is_inside_x(&self, x : Idx) -> bool where Vector<Idx,N> : HaveX<Idx> { x >= Idx::ZERO && x < self.size_x() }
    #[inline(always)] fn is_inside_y(&self, y : Idx) -> bool where Vector<Idx,N> : HaveY<Idx> { y >= Idx::ZERO && y < self.size_y() }
    #[inline(always)] fn is_inside_z(&self, z : Idx) -> bool where Vector<Idx,N> : HaveZ<Idx> { z >= Idx::ZERO && z < self.size_z() }
    #[inline(always)] fn is_inside_w(&self, w : Idx) -> bool where Vector<Idx,N> : HaveW<Idx> { w >= Idx::ZERO && w < self.size_w() }
}

impl<P, T, Idx, const N : usize> TryGet<P> for GridBase<T, Idx,N> where Idx : Integer, P : Into<Vector<Idx,N>>
{
    type Error=IndexOutOfRange<Vector<Idx,N>,Vector<Idx,N>>;

    fn try_get(&self, index : P) -> Result<&Self::Output, Self::Error> {
        let i = index.into();
        self.get(i).ok_or_else(|| IndexOutOfRange::new(i, self.size()))
    }
}

impl<P, T, Idx, const N : usize> Get<P> for GridBase<T, Idx,N> where Idx : Integer, P : Into<Vector<Idx,N>>
{
    type Output = <Self as Index<Vector<Idx,N>>>::Output;
    #[inline(always)]
    fn get(&self, pos : P) -> Option<&Self::Output> { self.position_to_index(pos.into()).and_then(|idx| Some(unsafe { self.values().get_unchecked(idx)} ) ) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, pos : P) -> &Self::Output { unsafe { let idx = self.position_to_index_unchecked(pos.into()); self.values().get_unchecked(idx) } }
}

impl<P, T, Idx, const N : usize> TryGetMut<P> for GridBase<T, Idx,N> where Idx : Integer, P : Into<Vector<Idx,N>>
{
    fn try_get_mut(&mut self, index : P) -> Result<&mut Self::Output, Self::Error> {
        let i = index.into();
        let size = self.size();
        self.get_mut(i).ok_or_else(|| IndexOutOfRange::new(i, size))
    }
}
impl<P, T, Idx, const N : usize> GetMut<P> for GridBase<T, Idx,N> where Idx : Integer, P: Into<Vector<Idx,N>>
{
    #[inline(always)]
    fn get_mut(&mut self, pos : P) -> Option<&mut Self::Output> { self.position_to_index(pos.into()).and_then(|i| Some(unsafe { self.values_mut().get_unchecked_mut(i) })) }
    #[track_caller]
    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, pos : P) -> &mut Self::Output{ unsafe { let idx = self.position_to_index_unchecked(pos.into()); self.values_mut().get_unchecked_mut(idx)} }
}

impl<P, T, Idx, const N : usize> GetManyMut<P> for GridBase<T, Idx,N> where Idx : Integer, P: Into<Vector<Idx,N>>
{
    #[inline(always)]
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [P; N2]) -> Result<[&mut Self::Output;N2], ManyMutError> {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        let indices = indices.map(|pos| self.position_to_index(pos.into()));
        if indices.any(|x| x.is_none())
        {
            Err(ManyMutError::IndexOutOfBounds)
        } else
        {
            self.values_mut().try_get_many_mut(indices.map(|idx| idx.unwrap()))
        }
    }
    
    fn get_many_mut<const N2: usize>(&mut self, indices: [P; N2]) -> Option<[&mut Self::Output;N2]> {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        let indices = indices.map(|pos| self.position_to_index(pos.into()));
        if indices.any(|x| x.is_none())
        {
            None
        } else
        {
            self.values_mut().get_many_mut(indices.map(|idx| idx.unwrap()))
        }
    }
}


impl<P, T, Idx, const N : usize> Index<P> for GridBase<T, Idx, N> where Idx : Integer, P: Into<Vector<Idx,N>>
{
    type Output=T;
    #[inline(always)]
    fn index(&self, index: P) -> &Self::Output { self.get_or_panic(index) }
}
impl<P, T, Idx, const N : usize> IndexMut<P> for GridBase<T, Idx, N> where Idx : Integer, P: Into<Vector<Idx,N>>
{
    #[inline(always)]
    fn index_mut(&mut self, index: P) -> &mut Self::Output { self.get_mut_or_panic(index) }
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
    use crate::prelude::*;

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
        assert_eq!(grid.values()[indice], grid[point2(0,0)]);
        assert_eq!(grid.values()[indice], 0);

        indice += 1;
        assert_eq!(grid.values()[indice], grid[point2(1,0)]);
        assert_eq!(grid.values()[indice], 1);

        indice += 1;
        assert_eq!(grid.values()[indice], grid[point2(0,1)]);
        assert_eq!(grid.values()[indice], 10);

        indice += 1;
        assert_eq!(grid.values()[indice], grid[point2(1,1)]);
        assert_eq!(grid.values()[indice], 11);

        indice += 1;
        assert_eq!(grid.values()[indice], grid[point2(0,2)]);
        assert_eq!(grid.values()[indice], 20);

        indice += 1;
        assert_eq!(grid.values()[indice], grid[point2(1,2)]);
        assert_eq!(grid.values()[indice], 21);
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