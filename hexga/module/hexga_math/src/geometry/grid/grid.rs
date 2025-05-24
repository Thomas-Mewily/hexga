use crate::*;

/// A N dimensional grid
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct GridBase<T, Idx, const N : usize> where Idx : Integer
{
    size  : Vector<Idx,N>,
    value : Vec<T>,
}

#[derive(Debug)]
pub struct IterMut<'a, T, Idx, const N : usize> where Idx : Integer
{
    value : std::iter::Enumerate<std::slice::IterMut<'a, T>>,
    size  : Vector<Idx,N>,
}
impl<'a, T, Idx, const N : usize> IterMut<'a,T,Idx,N> where Idx : Integer
{
    pub fn new(grid : &'a mut GridBase<T,Idx,N>) -> Self { Self { value: grid.value.iter_mut().enumerate(), size: grid.size }}
}
impl<'a, T, Idx, const N : usize> Iterator for IterMut<'a,T,Idx,N> where Idx : Integer
{
    type Item=(Vector<Idx,N>, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        self.value.next().map(|(idx, v)| (unsafe { Vector::<Idx,N>::from_index_unchecked(idx, self.size) }, v))
    }
}

impl<'a, T, Idx, const N : usize> IntoIterator for &'a mut GridBase<T, Idx, N> where Idx : Integer
{
    type Item = (Vector<Idx,N>, &'a mut T);
    type IntoIter = IterMut::<'a,T,Idx,N>;

    fn into_iter(self) -> Self::IntoIter { IterMut::new(self) }
}


#[derive(Clone, Debug)]
pub struct Iter<'a, T, Idx, const N : usize> where Idx : Integer
{
    value : std::iter::Enumerate<std::slice::Iter<'a, T>>,
    size  : Vector<Idx,N>,
}
impl<'a, T, Idx, const N : usize> Iter<'a,T,Idx,N> where Idx : Integer
{
    pub fn new(grid : &'a  GridBase<T,Idx,N>) -> Self { Self { value: grid.value.iter().enumerate(), size: grid.size }}
}
impl<'a, T, Idx, const N : usize> Iterator for Iter<'a,T,Idx,N> where Idx : Integer
{
    type Item=(Vector<Idx,N>, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.value.next().map(|(idx, v)| (unsafe { Vector::<Idx,N>::from_index_unchecked(idx, self.size) }, v))
    }
}

impl<'a, T, Idx, const N : usize> IntoIterator for &'a GridBase<T, Idx, N> where Idx : Integer
{
    type Item = (Vector<Idx,N>, &'a T);
    type IntoIter = Iter::<'a,T,Idx,N>;

    fn into_iter(self) -> Self::IntoIter { Iter::new(self) }
}



#[derive(Debug)]
pub struct IntoIter<T, Idx, const N : usize> where Idx : Integer
{
    value : std::iter::Enumerate<std::vec::IntoIter<T>>,
    size  : Vector<Idx,N>,
}
impl<T, Idx, const N : usize> IntoIter<T,Idx,N> where Idx : Integer
{
    pub fn new(grid : GridBase<T,Idx,N>) -> Self { Self { value: grid.value.into_iter().enumerate(), size: grid.size }}
}
impl<T, Idx, const N : usize> Iterator for IntoIter<T,Idx,N> where Idx : Integer
{
    type Item=(Vector<Idx,N>, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.value.next().map(|(idx, v)| (unsafe { Vector::<Idx,N>::from_index_unchecked(idx, self.size) }, v))
    }
}

impl<T, Idx, const N : usize> IntoIterator for GridBase<T, Idx, N> where Idx : Integer
{
    type Item = (Vector<Idx,N>, T);
    type IntoIter = IntoIter::<T,Idx,N>;
    fn into_iter(self) -> Self::IntoIter { IntoIter::new(self) }
}

 

#[derive(Clone)]
pub enum GridBaseError<Idx, const N : usize> where Idx : Integer
{
    NegativeSize(Vector::<Idx,N>),
    /// (dim, got)
    WrongDimension(Vector<Idx,N>, usize),
}

impl<Idx, const N : usize> Debug for GridBaseError<Idx, N> where Idx : Debug, Idx : Integer
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult {
        match self {
            Self::NegativeSize(size) => write!(f, "grid have a negative size {:?}", size),
            Self::WrongDimension(size, got) => write!(f, "grid vector have a wrong dimension : expected {:?} elements for a {:?} grid but only got {:?} elements", size.area(), size, got),
        }
    }
}

impl<T, Idx, const N : usize> GridBase<T, Idx, N> where Idx : Integer
{
    pub fn from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Option<Self> { Self::try_from_vec(size, value).ok() }
    pub fn try_from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Result<Self, GridBaseError<Idx,N>>
    {
        if *size.min_element() <= Idx::ZERO { return Err(GridBaseError::NegativeSize(size)); }
        if size.area().to_usize() != value.len() { return Err(GridBaseError::NegativeSize(size)); }
        Ok(Self { size, value })
    }

    /// Create a grid from a function
    pub fn from_fn<F>(size : Vector::<Idx,N>, mut f : F) -> Self where F : FnMut(Vector::<Idx,N>) -> T 
    {
        let area = size.area().to_usize();
        let mut value = Vec::with_capacity(area);
        for idx in size.iter_index()
        {
            value.push(f(idx));
        }
        Self { size, value }
    }

    /// Fill the grid with the [Default] value
    pub fn new(size : Vector::<Idx,N>) -> Self where T : Default { Self::from_fn(size, |_| ___()) }

    /// Fill the grid by cloning the value
    pub fn new_uniform(size : Vector::<Idx,N>, value : T) -> Self where T : Clone { Self::from_fn(size, |_| value.clone()) }

    /// Create a grid from a function in parallel
    pub fn from_fn_par<F>(size : Vector::<Idx,N>, f : F) -> Self where F : Fn(Vector::<Idx,N>) -> T + Sync, T : Send, Idx : Sync 
    {
        let area = size.area().to_usize();
        let mut values = Vec::with_capacity(area);
        (0..area).into_par_iter().map(|i| f(unsafe { Vector::<Idx, N>::from_index_unchecked(i, size) })).collect_into_vec(&mut values);
        Self { size, value: values }
    }

    /// Fill the grid with the [Default] value in parallel
    pub fn new_par(size : Vector::<Idx,N>) -> Self where T : Default + Send, Idx : Sync { Self::from_fn_par(size, |_| ___()) }

    /// Fill the grid by cloning the value in parallel
    pub fn new_uniform_par(size : Vector::<Idx,N>, value : T) -> Self where T : Clone + Sync + Send, Idx : Sync { Self::from_fn_par(size, |_| value.clone()) }
}

/// Param is just used to know if it is clonable or not because of [GridParam]
pub trait IGrid<T, Param, Idx, const N : usize> where Idx : Integer, 
    Self : IRectangle<Idx,N> 
        + IGridView<T, Param, Idx, N> + IGridViewMut<T, Param, Idx, N>
        // impl details :
        + Index<usize,Output=T> + IndexMut<usize,Output=T>
{
    // Todo : Don't expose how to grid is stored inside ? Use iterator instead ?
    fn values(&self) -> &[T];
    fn values_mut(&mut self) -> &mut [T];
    fn into_values(self) -> Vec<T>;

    unsafe fn position_to_index_unchecked(&self, pos : Vector<Idx,N>) -> usize { unsafe { Vector::<Idx,N>::to_index_unchecked(pos, self.size()) } }
    unsafe fn index_to_position_unchecked(&self, index : usize) -> Vector<Idx,N> { unsafe { Vector::<Idx,N>::from_index_unchecked(index, self.size()) } }

    #[inline(always)]
    fn position_to_index(&self, pos : Vector<Idx,N>) -> Option<usize> { Vector::<Idx,N>::to_index(pos, self.size()) }
    #[inline(always)]
    fn index_to_position(&self, index : usize) -> Option<Vector<Idx,N>> { Vector::<Idx,N>::from_index(index, self.size()) }


    /*
    #[inline(always)]
    fn get_index(&self, index : usize) -> Option<&T> { self.get(index) }
    #[inline(always)]
    fn get_index_mut(&mut self, index : usize) -> Option<&mut T> { self.get_mut(index) }


     
    fn replace_index(&mut self, val : T, index : usize) -> Option<T> { self.get_index_mut(index).map(|v| std::mem::replace(v, val)) }
    /// Do nothings if the index is outside the range
    fn set_index(&mut self, val : T, idx : usize) -> bool { self.get_index_mut(idx).map(|v| *v = val).is_some() }
    */

    /*
    fn subgrid(&self, r : Rectangle<Idx,N>) -> Option<Self>
    {
        self.view().subgrid(r)
    }
    */

    fn intersect_rect(&self, r : Rectangle<Idx,N>) -> Rectangle<Idx,N>  where Vector<Idx,N> : UnitArithmetic, Idx : PartialOrd { r.intersect_or_empty(self.rect()) }

    //fn crop_margin(&self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> Self where T : Clone, Param : Clone;

    fn transform<Dest, F>(self, f : F) -> <Self as IGridView<T,Param,Idx,N>>::Map<Dest> where F : FnMut(T) -> Dest, Param : Clone;
    fn transform_par<Dest, F>(self, f : F) -> <Self as IGridView<T,Param,Idx,N>>::Map<Dest> where F : Fn(T) -> Dest + Sync + Send, T : Send + Sync, Dest : Send, Idx : Sync, Param : Clone;

    type View<'a> : IGridView<T,Param,Idx,N> where Self: 'a;
    fn view<'a>(&'a self) -> Self::View<'a>;

    type ViewMut<'a> : IGridView<T,Param,Idx,N> where Self: 'a;
    fn view_mut<'a>(&'a mut self) -> Self::ViewMut<'a>;
}

// Todo : Remove T: Clone constraint
impl<T, Idx, const N : usize> Crop<Idx,N> for GridBase<T, Idx, N> where Idx : Integer, T : Clone
{
    fn crop(self, subrect : Rectangle<Idx, N>) -> Option<Self>  { self.view().crop(subrect).map(|v| v.to_grid()) }
    unsafe fn crop_unchecked(self, subrect : Rectangle<Idx, N>) -> Self { unsafe { self.view().crop_unchecked(subrect).to_grid() } }
}


impl<T, Idx, const N : usize> IGrid<T,(),Idx,N> for GridBase<T, Idx, N> where Idx : Integer,
{
    fn values(&self) -> &[T] { &self.value }
    fn values_mut(&mut self) -> &mut [T] { &mut self.value }

    fn into_values(self) -> Vec<T> { self.value }

    fn transform<Dest, F>(self, f : F) -> <Self as IGridView<T,(),Idx,N>>::Map<Dest> where F : FnMut(T) -> Dest 
    { GridBase { size: self.size, value: self.value.into_iter().map(f).collect() } }
    fn transform_par<Dest, F>(self, f : F) -> <Self as IGridView<T,(),Idx,N>>::Map<Dest> where F : Fn(T) -> Dest + Sync + Send, T : Send + Sync, Dest : Send, Idx : Sync, () : Clone 
    { GridBase { size: self.size, value: self.value.into_par_iter().map(f).collect() } }
    
    //fn crop_margin(&self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> Self where T : Clone { self.view().crop_margin(margin_start, margin_end).to_grid() }

    type View<'a> = grid::GridView<'a, T,Idx,N> where Self: 'a;
    fn view<'a>(&'a self) -> grid::GridView<'a, T,Idx,N> { grid::GridView::from_grid(self) }

    type ViewMut<'a> = grid::GridViewMut<'a, T,Idx,N> where Self: 'a;
    fn view_mut<'a>(&'a mut self) -> grid::GridViewMut<'a, T,Idx,N> { grid::GridViewMut::from_grid(self) }
}

impl<T, Idx, const N : usize> IGridView<T,(),Idx,N> for GridBase<T, Idx, N> where Idx : Integer 
{

    type Map<Dest>=GridBase<Dest, Idx, N>;
    fn map<Dest, F>(&self, mut f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, () : Clone { GridBase::from_fn(self.size(), |p| f(&self[p])) }
    fn map_par<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, () : Clone  { GridBase::from_fn_par(self.size(), |p| f(&self[p])) }

    /* 
    fn subgrid(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone { self.view().subgrid(rect) }
    fn subgrid_par(&self, rect : Rectangle<Idx, N>) -> Self::Map<T> where T : Clone + Send + Sync, Idx : Sync { self.view().subgrid_par(rect) }

    type SubView<'b> = GridView<'b,T,Idx,N> where Self: 'b;
    fn crop_intersect<'a>(&'a self, rect : Rectangle<Idx, N>) -> Self::SubView<'a> where T : Clone { GridView::new(self, rect) }
    */
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

impl<T, Idx, const N : usize> GridBase<T, Idx, N> where Idx : Integer
{
    pub fn iter(&self) -> Iter<'_,T,Idx,N> { self.into_iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_,T,Idx,N> { self.into_iter() }
}

impl<T, Idx, const N : usize> Length for GridBase<T, Idx, N> 
    where Idx : Integer
{
    #[inline(always)]
    fn len(&self) -> usize { self.values().len() }
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
        assert_eq!(grid1.view_mut().subview_mut(rect), grid2.view_mut().subview_mut(rect));
    }
}