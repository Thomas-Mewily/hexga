use super::*;

/// A N-dimensional grid
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename = "Grid"))]
pub struct Grid2Of<T, Idx, C, B, const N : usize>
    where Idx : Integer, B: PositionBijection<Idx,N>, C: DerefMut<Target=[T]>
{
    pub(crate) size  : Vector<Idx,N>,
    pub(crate) values: C,
    pub(crate) bijection: PhantomData<B>,
}

impl<T, Idx, C, B, const N : usize> Hash for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, T: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.size.hash(state);
        self.values.hash(state);
    }
}

impl<T, Idx, C, B, const N : usize> Clone for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, T: Clone
{
    fn clone(&self) -> Self {
        Self { size: self.size.clone(), values: self.values.clone(), bijection: PhantomData }
    }
}

impl<T, Idx, C, B, const N : usize> PartialEq for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.values == other.values
    }
}
impl<T, Idx, C, B, const N : usize> Eq for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, T: Eq
{}

impl<T, Idx, C, B, const N : usize> PartialOrd for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, T: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.size.partial_cmp(&other.size) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.values.partial_cmp(&other.values)
    }
}
impl<T, Idx, C, B, const N : usize> Ord for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, T: Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.size.cmp(&other.size) {
            Ordering::Equal => self.values.cmp(&other.values),
            ord => ord,
        }
    }
}

impl<T, Idx, C, B, const N : usize> MapIntern for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>
{
    type Item=T;
    fn map_intern<F>(mut self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item {
        self.values = self.values.map_intern(f);
        self
    }
}
impl<T, Idx, C, B, const N : usize> MapInternWith for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>
{
    fn map_with_intern<F>(mut self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item {
        assert_eq!(self.size(), other.size(), "size mismatch");
        self.values = self.values.map_with_intern(other.values, f);
        self
    }
}
impl<T, Idx, C, B, const N : usize> Map for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>
{
    type WithType<R> = Grid2Of<R, Idx, N>;

    fn map<R,F>(self, f: F) -> Self::WithType<R> where F: FnMut(Self::Item) -> R {
        unsafe { Self::WithType::<R>::from_vec_unchecked(self.size, self.values.map(f)) }
    }
}
impl<T, Idx, C, B, const N : usize> MapWith for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>
{
    fn map_with<R, Item2, F>(self, other: Self::WithType<Item2>, f : F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R
    {
        assert_eq!(self.size(), other.size(), "size mismatch");
        unsafe { Self::WithType::<R>::from_vec_unchecked(self.size, self.values.map_with(other.values, f)) }
    }
}

impl<T, Idx, C, B, const N : usize> Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>
{
    impl_number_basic_trait!();
}

impl<T, Idx, C, B, const N : usize> GetPosition<Idx, N> for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>
{
    #[inline(always)]
    fn pos(&self) -> Vector<Idx,N> { zero() }
}
impl<T, Idx, C, B, const N : usize> GetSize<Idx, N> for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>
{
    #[inline(always)]
    fn size(&self) -> Vector<Idx, N> { self.size }
}

impl<P, T, Idx, C, B, const N : usize> TryGet<P> for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, P : Into<Vector<Idx,N>>
{
    type Error=IndexOutOfRange<Vector<Idx,N>,Vector<Idx,N>>;

    fn try_get(&self, index : P) -> Result<&Self::Output, Self::Error> {
        let i = index.into();
        self.get(i).ok_or_else(|| IndexOutOfRange::new(i, self.size()))
    }
}

impl<P, T, Idx, C, B, const N : usize> Get<P> for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, P : Into<Vector<Idx,N>>
{
    type Output = <Self as Index<Vector<Idx,N>>>::Output;
    #[inline(always)]
    fn get(&self, pos : P) -> Option<&Self::Output> { self.position_to_index(pos.into()).and_then(|idx| Some(unsafe { self.values().get_unchecked(idx)} ) ) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, pos : P) -> &Self::Output { unsafe { let idx = self.position_to_index_unchecked(pos.into()); self.values().get_unchecked(idx) } }
}

impl<P, T, Idx, C, B, const N : usize> TryGetMut<P> for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, P : Into<Vector<Idx,N>>
{
    fn try_get_mut(&mut self, index : P) -> Result<&mut Self::Output, Self::Error> {
        let i = index.into();
        let size = self.size();
        self.get_mut(i).ok_or_else(|| IndexOutOfRange::new(i, size))
    }
}
impl<P, T, Idx, C, B, const N : usize> GetMut<P> for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, P: Into<Vector<Idx,N>>
{
    #[inline(always)]
    fn get_mut(&mut self, pos : P) -> Option<&mut Self::Output> { self.position_to_index(pos.into()).and_then(|i| Some(unsafe { self.values_mut().get_unchecked_mut(i) })) }
    #[track_caller]
    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, pos : P) -> &mut Self::Output{ unsafe { let idx = self.position_to_index_unchecked(pos.into()); self.values_mut().get_unchecked_mut(idx)} }
}

impl<P, T, Idx, C, B, const N : usize> GetManyMut<P> for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, P: Into<Vector<Idx,N>>
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


impl<P, T, Idx, C, B, const N : usize> Index<P> for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, P: Into<Vector<Idx,N>>
{
    type Output=T;
    #[inline(always)]
    fn index(&self, index: P) -> &Self::Output { self.get_or_panic(index) }
}
impl<P, T, Idx, C, B, const N : usize> IndexMut<P> for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, P: Into<Vector<Idx,N>>
{
    #[inline(always)]
    fn index_mut(&mut self, index: P) -> &mut Self::Output { self.get_mut_or_panic(index) }
}

impl<T, Idx, C, B, const N : usize> Length for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>
{
    #[inline(always)]
    fn len(&self) -> usize { self.values().len() }
}


impl<T, Idx, C, B, const N : usize> Crop<Idx,N> for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>, T : Clone
{
    fn crop(self, subrect : Rectangle<Idx, N>) -> Option<Self> {
        self.view().crop(subrect).map(|v| v.to_grid())
    }
}

pub struct Grid2ViewOf<T, Idx, N>
{

}

impl<T, Idx, C, B, const N : usize> View for Grid2Of<T, Idx, C, B, N>
    where Idx : Integer, B: PositionBijection<Idx,N>,
{
    type View<'v> where Self: 'v;

    fn view<'s>(&'s self) -> Self::View<'s> {
        todo!()
    }
}

pub trait PositionBijection<Idx, const N: usize> where Idx: Integer
{
    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    fn position_to_index<P>(pos: P, size: Vector<Idx,N>) -> Option<usize> where P: Into<Vector<Idx,N>>;
    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    fn index_to_position(index : usize, size: Vector<Idx,N>) -> Option<Vector<Idx,N>>;

    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    ///
    /// If you override this, you probably also want to override [`IGrid::index_to_position_unchecked`] and [`IGrid::external_position_to_position_unchecked`]
    #[inline(always)]
    #[track_caller]
    unsafe fn position_to_index_unchecked<P>(pos : P, size: Vector<Idx,N>) -> usize where P: Into<Vector<Idx,N>> { Self::position_to_index(pos, size).unwrap() }
    /// Used for the memory bijection between the one dimensional vector and the N-dimensional grid.
    ///
    /// If you override this, you probably also want to override [`IGrid::position_to_index_unchecked`] and [`IGrid::external_position_to_position_unchecked`]
    #[inline(always)]
    #[track_caller]
    unsafe fn index_to_position_unchecked(index : usize, size: Vector<Idx,N>) -> Vector<Idx,N> { Self::index_to_position(index, size).unwrap() }
}
pub struct DefaultPositionBijection;

#[cfg(test)]
mod grid_test
{
    use super::*;

    #[test]
    fn index_order()
    {
        use super::*;
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
        use super::*;
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
        use super::*;

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
        use super::*;

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

        let rect = rect2i(0, 0, 1, size.y);
        assert_eq!(grid1.view().crop_intersect(rect), grid2.view().crop_intersect(rect));
        assert_eq!(grid1.view_mut().crop_intersect(rect), grid2.view_mut().crop_intersect(rect));
        assert_eq!(grid1.view_mut().subview(rect), grid2.view_mut().subview(rect));
    }
}