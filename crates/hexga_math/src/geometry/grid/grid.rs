use super::*;

/// A N-dimensional grid
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename = "Grid"))] // Todo: use an enum to serialize it with different algo for compressing the grid
pub struct GridOf<T, Idx, const N: usize>
where
    Idx: Integer,
{
    pub(crate) size: Vector<Idx, N>,
    pub(crate) values: Vec<T>,
}

impl<T, Idx, const N: usize> Default for GridOf<T, Idx, N>
where
    Idx: Integer,
{
    fn default() -> Self {
        Self { size: ___(), values: ___() }
    }
}

macro_rules! impl_grid_fmt_method {
    ($trait_name :ident) => {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
        {
            writeln!(f)?;

            const SEP: &'static str = " ";
            let size = self.size();

            use ::core::fmt::Write;

            let strings = self
                .iter()
                .map(|(_, v)| {
                    let mut s = String::new();
                    let wrapper = fmt::from_fn(|f| ::core::fmt::$trait_name::fmt(v, f));
                    write!(s, "{}", wrapper)?;
                    Ok(s)
                })
                .collect::<Result<Vec<_>, _>>()?;

            match N
            {
                2 =>
                {
                    let width = strings.iter().map(|s| s.len()).max().unwrap_or(0);
                    let g = unsafe { GridOf::from_vec_unchecked(size, strings) };

                    for y in (0..size[1].to_usize()).rev()
                    {
                        for x in 0..size[0].to_usize()
                        {
                            let mut idx = Vector::<Idx, N>::ZERO;
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
        impl<T, Idx, const N : usize> std::fmt::$trait_name for GridOf<T, Idx, N> where Idx : Integer, T: std::fmt::$trait_name
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

impl<T, Idx, const N: usize> Map for GridOf<T, Idx, N>
where Idx: Integer,
{
    type Item = T;
    fn map<F>(mut self, f: F) -> Self where F: FnMut(T) -> T {
        self.values = self.values.map(f);
        self
    }
}
impl<T, Idx, const N: usize> MapInto for GridOf<T, Idx, N>
where Idx: Integer,
{
    type WithType<R> = GridOf<R, Idx, N>;
    fn map_into<F, R>(self, f: F) -> Self::WithType<R> where F: FnMut(T) -> R {
        unsafe { Self::WithType::<R>::from_vec_unchecked(self.size, self.values.map_into(f)) }
    }
}
impl<T, Idx, const N: usize> MapZip for GridOf<T, Idx, N>
where Idx: Integer,
{
    fn map_zip<F, const M: usize>(this: [Self; M], f: F) -> Self where F: FnMut([T; M]) -> T {
        if M == 0 { return ___(); }
        let size = this[0].size();
        assert!((&this).all(|g| g.size() == size), "size mismatch");
        unsafe { Self::from_vec_unchecked(size, <Vec<T> as MapZip>::map_zip(this.map(|g| g.values), f)) }
    }
}
impl<T, Idx, const N: usize> MapZipInto for GridOf<T, Idx, N>
where Idx: Integer,
{
    fn map_zip_into<F, R, const M: usize>(this: [Self; M], f: F) -> Self::WithType<R> where F: FnMut([T; M]) -> R {
        if M == 0 { return ___(); }
        let size = this[0].size();
        assert!((&this).all(|g| g.size() == size), "size mismatch");
        unsafe { Self::WithType::<R>::from_vec_unchecked(size, <Vec<T> as MapZipInto>::map_zip_into(this.map(|g| g.values), f)) }
    }
}
impl<T, Idx, const N: usize> GridOf<T, Idx, N>
where
    Idx: Integer,
{
    impl_number_basic_trait!();
}

impl<T, Idx, const N: usize> IGrid<T, Idx, N> for GridOf<T, Idx, N>
where
    Idx: Integer,
{
    type WithType<U> = GridOf<U, Idx, N>;

    fn values(&self) -> &[T] { &self.values }
    fn values_mut(&mut self) -> &mut [T] { &mut self.values }

    fn into_size_and_values(self) -> (Vector<Idx, N>, Vec<T>) { (self.size, self.values) }
    unsafe fn from_vec_unchecked<P>(size: P, values: Vec<T>) -> Self
    where
        P: Into<Vector<Idx, N>>,
    {
        let size = size.into();
        debug_assert_eq!(size.area_usize_checked(), Some(values.len()));
        Self {
            size,
            values,
        }
    }
}

impl<T, Idx, const N: usize> GetPosition<Idx, N> for GridOf<T, Idx, N>
where
    Idx: Integer,
{
    #[inline(always)]
    fn pos(&self) -> Vector<Idx, N> { zero() }
}
impl<T, Idx, const N: usize> GetSize<Idx, N> for GridOf<T, Idx, N>
where
    Idx: Integer,
{
    #[inline(always)]
    fn size(&self) -> Vector<Idx, N> { self.size }
}

impl<T, Idx, const N: usize> CollectionBijective for GridOf<T, Idx, N> where Idx: Integer {}
impl<T, Idx: Integer, const N: usize> Collection<T> for GridOf<T, Idx, N>
{
    type Ref<'a> = &'a T where Self: 'a, T: 'a;
}
impl<T, Idx: Integer, const N: usize> CollectionMut<T> for GridOf<T, Idx, N>
{
    type RefMut<'a> = &'a mut T where Self: 'a, T: 'a;
}

impl<P, T, Idx, const N: usize> CollectionIndex<P> for GridOf<T, Idx, N>
where
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    type Output = <Self as Index<Vector<Idx, N>>>::Output;
    type Ref<'a> = &'a Self::Output where Self: 'a, Self::Output: 'a;
}
impl<P, T, Idx, const N: usize> Get<P> for GridOf<T, Idx, N>
where
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    #[inline(always)]
    fn get<'a>(&'a self, pos: P) -> Option<Self::Ref<'a>> where Self::Output: 'a { self.position_to_index(pos.into()).map(|idx| unsafe { self.values().get_unchecked(idx) }) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked<'a>(&'a self, pos: P) -> Self::Ref<'a> where Self::Output: 'a
    {
        unsafe {
            let idx = self.position_to_index_unchecked(pos.into());
            self.values().get_unchecked(idx)
        }
    }
}
impl<P, T, Idx, const N: usize> TryGet<P> for GridOf<T, Idx, N>
where
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    type Error = IndexOutOfBounds<Vector<Idx, N>, Vector<Idx, N>>;

    fn try_get<'a>(&'a self, index: P) -> Result<Self::Ref<'a>, Self::Error> where Self::Output: 'a
    {
        let i = index.into();
        self.get(i).ok_or_else(|| IndexOutOfBounds::new(i, self.size()))
    }
}

impl<P, T, Idx, const N: usize> TryGetMut<P> for GridOf<T, Idx, N>
where
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    fn try_get_mut<'a>(&'a mut self, index: P) -> Result<Self::RefMut<'a>, Self::Error> where Self::Output: 'a
    {
        let i = index.into();
        let size = self.size();
        self.get_mut(i).ok_or_else(|| IndexOutOfBounds::new(i, size))
    }
}
impl<P, T, Idx, const N: usize> CollectionIndexMut<P> for GridOf<T, Idx, N>
where
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    type RefMut<'a> = &'a mut Self::Output where Self: 'a, Self::Output: 'a;
}
impl<P, T, Idx, const N: usize> GetMut<P> for GridOf<T, Idx, N>
where
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    #[inline(always)]
    fn get_mut<'a>(&'a mut self, pos: P) -> Option<Self::RefMut<'a>> where Self::Output: 'a { self.position_to_index(pos.into()).map(|i| unsafe { self.values_mut().get_unchecked_mut(i) }) }
    #[track_caller]
    #[inline(always)]
    unsafe fn get_unchecked_mut<'a>(&'a mut self, pos: P) -> Self::RefMut<'a> where Self::Output: 'a
    {
        unsafe {
            let idx = self.position_to_index_unchecked(pos.into());
            self.values_mut().get_unchecked_mut(idx)
        }
    }
}

impl<P, T, Idx, const N: usize> GetManyMut<P> for GridOf<T, Idx, N>
where
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    #[inline(always)]
    fn try_get_many_mut<'a, const N2: usize>(&'a mut self, indices: [P; N2]) -> Result<[Self::RefMut<'a>; N2], ManyMutError>
     where Self::Output: 'a{
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        let indices = indices.map(|pos| self.position_to_index(pos.into()));
        if indices.any(|x| x.is_none())
        {
            Err(ManyMutError::IndexOutOfBounds)
        }
        else
        {
            self.values_mut().try_get_many_mut(indices.map(|idx| idx.unwrap()))
        }
    }

    fn get_many_mut<'a, const N2: usize>(&'a mut self, indices: [P; N2]) -> Option<[Self::RefMut<'a>; N2]>
     where Self::Output: 'a{
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        let indices = indices.map(|pos| self.position_to_index(pos.into()));
        if indices.any(|x| x.is_none())
        {
            None
        }
        else
        {
            self.values_mut().get_many_mut(indices.map(|idx| idx.unwrap()))
        }
    }
}

impl<P, T, Idx, const N: usize> Index<P> for GridOf<T, Idx, N>
where
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    type Output = T;
    #[inline(always)]
    fn index(&self, index: P) -> &Self::Output { self.get_or_panic(index) }
}
impl<P, T, Idx, const N: usize> IndexMut<P> for GridOf<T, Idx, N>
where
    Idx: Integer,
    P: Into<Vector<Idx, N>>,
{
    #[inline(always)]
    fn index_mut(&mut self, index: P) -> &mut Self::Output { self.get_mut_or_panic(index) }
}

impl<T, Idx, const N: usize> Length for GridOf<T, Idx, N>
where
    Idx: Integer,
{
    #[inline(always)]
    fn len(&self) -> usize { self.values().len() }
}

impl<T, Idx, const N: usize> Crop<Idx, N> for GridOf<T, Idx, N>
where
    Idx: Integer,
    T: Clone,
{
    fn crop(self, subrect: Rectangle<Idx, N>) -> Option<Self> { self.view().crop(subrect).map(|v| v.to_grid()) }
}

#[cfg(test)]
mod grid_test
{
    use super::*;

    #[test]
    fn index_order()
    {
        use super::*;
        //let x = Grid::<char>::new(vec2i(2, 4));
        let size = vec2i(2, 3);

        let grid = Grid2::from_fn(size, |p| p.x + 10 * p.y);

        /*
        dbg!(&grid);
        for y in (0..size.y).rev()
        {
            for x in 0..size.x
            {
                print!("{:2} ", grid[vec2i(x, y)]);
            }
            println!()
        }
        */

        let mut indice = 0;
        assert_eq!(grid.values()[indice], grid[vec2i(0, 0)]);
        assert_eq!(grid.values()[indice], 0);

        indice += 1;
        assert_eq!(grid.values()[indice], grid[vec2i(1, 0)]);
        assert_eq!(grid.values()[indice], 1);

        indice += 1;
        assert_eq!(grid.values()[indice], grid[vec2i(0, 1)]);
        assert_eq!(grid.values()[indice], 10);

        indice += 1;
        assert_eq!(grid.values()[indice], grid[vec2i(1, 1)]);
        assert_eq!(grid.values()[indice], 11);

        indice += 1;
        assert_eq!(grid.values()[indice], grid[vec2i(0, 2)]);
        assert_eq!(grid.values()[indice], 20);

        indice += 1;
        assert_eq!(grid.values()[indice], grid[vec2i(1, 2)]);
        assert_eq!(grid.values()[indice], 21);
    }

    #[test]
    fn out_of_range()
    {
        use super::*;
        let grid = Grid2::from_fn(vec2i(2, 3), |_| 42);

        assert_eq!(grid.get(vec2i(0, 0)), Some(&42));
        assert_eq!(grid.get(vec2i(-1, 0)), None);
        assert_eq!(grid.get(vec2i(1, 0)), Some(&42));
        assert_eq!(grid.get(vec2i(2, 0)), None);

        assert_eq!(grid.get(vec2i(0, 0)), Some(&42));
        assert_eq!(grid.get(vec2i(0, -1)), None);
        assert_eq!(grid.get(vec2i(0, 2)), Some(&42));
        assert_eq!(grid.get(vec2i(0, 3)), None);
    }

    #[test]
    fn slice_cmp()
    {
        use super::*;

        let size = vec2i(2, 3);
        let grid = Grid2::from_fn(size, |p| p.x + 10 * p.y);

        assert_eq!(grid.size(), grid.view().size());

        let subgrid = grid.view().subgrid(size.to_rect());

        assert_eq!(grid, subgrid);

        let smaller_size = vec2i(1, 2);
        let smaller_grid = Grid2::from_fn(smaller_size, |p| p.x + 10 * p.y);
        let smaller_grid_from_bigger_grid = grid.view().subgrid(smaller_size.to_rect());
        assert_eq!(smaller_grid, smaller_grid_from_bigger_grid);

        let top_right_grid_size = vec2i(1, 2);
        let offset = Vector2::ONE;
        let top_right_grid = Grid2::from_fn(smaller_size, |p| {
            let p = p + offset;
            p.x + 10 * p.y
        });

        assert_eq!(top_right_grid, grid.subgrid(top_right_grid_size.to_rect().moved_by(offset)));
    }

    #[test]
    fn subslice()
    {
        use super::*;

        let size = vec2i(2, 3);

        let mut grid1 = Grid2::from_fn(size, |p| p.x + 10 * p.y);
        let mut grid2 = Grid2::from_fn(size, |p| p.x + 10 * p.y);

        assert_eq!(grid1, grid2);
        assert_eq!(grid1.view(), grid2.view());
        assert_eq!(grid1.view_mut(), grid2.view_mut());

        grid2[vec2i(1, 0)] = 42;

        assert_ne!(grid1, grid2);
        assert_ne!(grid1.view(), grid2.view());
        assert_ne!(grid1.view_mut(), grid2.view_mut());

        let rect = rect2i(0, 0, 1, size.y);
        assert_eq!(grid1.view().crop_intersect(rect), grid2.view().crop_intersect(rect));
        assert_eq!(grid1.view_mut().crop_intersect(rect), grid2.view_mut().crop_intersect(rect));
        assert_eq!(grid1.view_mut().subview(rect), grid2.view_mut().subview(rect));
    }
}
