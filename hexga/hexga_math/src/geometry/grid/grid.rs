use super::*;

/// A N-dimensional grid
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename = "Grid"))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
pub struct GridBase<T, Idx, const N : usize> where Idx : Integer
{
    // The size is will be deserialized first before the values,
    // then we can give a hint to serde about the size of the vector to alloc when deserializing
    pub(crate) size   : Vector<Idx,N>,
    pub(crate) values : Vec<T>,
}

macro_rules! impl_grid_fmt_method {
    ($trait_name :ident) => {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            const SEP :&'static str = " ";
            let size = self.size();

            let strings = self.iter()
                .map(|(_, v)| {
                    let mut s = String::new();
                    let mut tmp_f = std::fmt::Formatter::new(&mut s, ___());
                    std::fmt::$trait_name::fmt(v, &mut tmp_f)?;
                    Ok(s)
                })
                .collect::<Result<Vec<_>, _>>()?;

            match N
            {
                2 =>
                {
                    let width = strings.iter().map(|s| s.len()).max().unwrap_or(0);
                    let g = unsafe { GridBase::from_vec_unchecked(size, strings) };

                    for y in (0..size[1].to_usize()).rev()
                    {
                        for x in 0..size[0].to_usize()
                        {
                            let mut idx = Vector::<Idx,N>::ZERO;
                            idx[0] = Idx::cast_from(x);
                            idx[1] = Idx::cast_from(y);
                            write!(f, "{:>width$}", g[idx], width = width)?;
                            f.write_str(SEP)?;
                        }
                        writeln!(f)?;
                    }
                }
                _ =>
                {
                    for v in &strings
                    {
                        write!(f, "{}", v)?;
                        f.write_str(SEP)?;
                    }
                }
            }
            writeln!(f, "size: {:?}", size)
        }
    };
}

map_on_std_fmt!(
    ($trait_name :ident) =>
    {
        impl<T, Idx, const N : usize> std::fmt::$trait_name for GridBase<T, Idx, N> where Idx : Integer, T: std::fmt::$trait_name
        {
            impl_grid_fmt_method!($trait_name);
        }

        impl<'a, G, T, Idx, const N : usize> std::fmt::$trait_name for GridView<'a, G, T, Idx, N> where G : IGrid<T, Idx, N>, Idx : Integer, T: std::fmt::$trait_name
        {
            impl_grid_fmt_method!($trait_name);
        }

        impl<'a, G, T, Idx, const N : usize> std::fmt::$trait_name for GridViewMut<'a, G, T, Idx, N> where G : IGrid<T, Idx, N>, Idx : Integer, T: std::fmt::$trait_name
        {
            impl_grid_fmt_method!($trait_name);
        }
    }
);

impl<T, Idx, const N : usize> MapIntern for GridBase<T, Idx, N> where Idx : Integer
{
    type Item=T;
    fn map_intern<F>(mut self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item {
        self.values = self.values.map_intern(f);
        self
    }
}
impl<T, Idx, const N : usize> MapInternWith for GridBase<T, Idx, N> where Idx : Integer
{
    fn map_with_intern<F>(mut self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item {
        assert_eq!(self.size(), other.size(), "size mismatch");
        self.values = self.values.map_with_intern(other.values, f);
        self
    }
}
impl<T, Idx, const N : usize> Map for GridBase<T, Idx, N> where Idx : Integer
{
    type WithType<R> = GridBase<R, Idx, N>;

    fn map<R,F>(self, f: F) -> Self::WithType<R> where F: FnMut(Self::Item) -> R {
        unsafe { Self::WithType::<R>::from_vec_unchecked(self.size, self.values.map(f)) }
    }
}
impl<T, Idx, const N : usize> MapWith for GridBase<T, Idx, N> where Idx : Integer
{
    fn map_with<R, Item2, F>(self, other: Self::WithType<Item2>, f : F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R
    {
        assert_eq!(self.size(), other.size(), "size mismatch");
        unsafe { Self::WithType::<R>::from_vec_unchecked(self.size, self.values.map_with(other.values, f)) }
    }
}

impl<T, Idx, const N : usize> GridBase<T, Idx, N> where Idx : Integer
{
    impl_number_basic_trait!();
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