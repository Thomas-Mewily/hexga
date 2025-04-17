use crate::*;

// Todo : remettre le trait IterArea pour la grille les + Vecteur, + faire les itérateur sur la grille pour itérer dans un sous rectangle 

/* 
Todo : impl AsMut<&[T]> AsRef<&[T]>
+ method as_mut_slice(&self) as_slice(&mut self)
*/

/// A N dimensional grid
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct GridBase<T, Idx, const N : usize> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    size  : Vector<Idx,N>,
    value : Vec<T>,
}

#[derive(Debug)]
pub struct IterMut<'a, T, Idx, const N : usize> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    value : std::iter::Enumerate<std::slice::IterMut<'a, T>>,
    size  : Vector<Idx,N>,
}
impl<'a, T, Idx, const N : usize> IterMut<'a,T,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    pub fn new(grid : &'a mut GridBase<T,Idx,N>) -> Self { Self { value: grid.value.iter_mut().enumerate(), size: grid.size }}
}
impl<'a, T, Idx, const N : usize> Iterator for IterMut<'a,T,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    type Item=(Vector<Idx,N>, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        self.value.next().map(|(idx, v)| (unsafe { Vector::<Idx,N>::from_index_unchecked(idx, self.size) }, v))
    }
}

impl<'a, T, Idx, const N : usize> IntoIterator for &'a mut GridBase<T, Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    type Item = (Vector<Idx,N>, &'a mut T);
    type IntoIter = IterMut::<'a,T,Idx,N>;

    fn into_iter(self) -> Self::IntoIter { IterMut::new(self) }
}


#[derive(Clone, Debug)]
pub struct Iter<'a, T, Idx, const N : usize> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    value : std::iter::Enumerate<std::slice::Iter<'a, T>>,
    size  : Vector<Idx,N>,
}
impl<'a, T, Idx, const N : usize> Iter<'a,T,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    pub fn new(grid : &'a  GridBase<T,Idx,N>) -> Self { Self { value: grid.value.iter().enumerate(), size: grid.size }}
}
impl<'a, T, Idx, const N : usize> Iterator for Iter<'a,T,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    type Item=(Vector<Idx,N>, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.value.next().map(|(idx, v)| (unsafe { Vector::<Idx,N>::from_index_unchecked(idx, self.size) }, v))
    }
}

impl<'a, T, Idx, const N : usize> IntoIterator for &'a GridBase<T, Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    type Item = (Vector<Idx,N>, &'a T);
    type IntoIter = Iter::<'a,T,Idx,N>;

    fn into_iter(self) -> Self::IntoIter { Iter::new(self) }
}



#[derive(Debug)]
pub struct IntoIter<T, Idx, const N : usize> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    value : std::iter::Enumerate<std::vec::IntoIter<T>>,
    size  : Vector<Idx,N>,
}
impl<T, Idx, const N : usize> IntoIter<T,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    pub fn new(grid : GridBase<T,Idx,N>) -> Self { Self { value: grid.value.into_iter().enumerate(), size: grid.size }}
}
impl<T, Idx, const N : usize> Iterator for IntoIter<T,Idx,N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    type Item=(Vector<Idx,N>, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.value.next().map(|(idx, v)| (unsafe { Vector::<Idx,N>::from_index_unchecked(idx, self.size) }, v))
    }
}

impl<T, Idx, const N : usize> IntoIterator for GridBase<T, Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    type Item = (Vector<Idx,N>, T);
    type IntoIter = IntoIter::<T,Idx,N>;
    fn into_iter(self) -> Self::IntoIter { IntoIter::new(self) }
}

 

#[derive(Clone)]
pub enum GridBaseError<Idx, const N : usize> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    NegativeSize(Vector::<Idx,N>),
    /// (dim, got)
    WrongDimension(Vector<Idx,N>, usize),
}

impl<Idx, const N : usize> Debug for GridBaseError<Idx, N> where Idx : Debug, Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult {
        match self {
            Self::NegativeSize(size) => write!(f, "grid have a negative size {:?}", size),
            Self::WrongDimension(size, got) => write!(f, "grid vector have a wrong dimension : expected {:?} elements for a {:?} grid but only got {:?} elements", size.area(), size, got),
        }
    }
}

impl<T, Idx, const N : usize> GridBase<T, Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    pub fn from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Option<Self> { Self::try_from_vec(size, value).ok() }
    pub fn try_from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Result<Self, GridBaseError<Idx,N>>
    {
        if *size.min_element() <= Idx::ZERO { return Err(GridBaseError::NegativeSize(size)); }
        if size.area().to_usize() != value.len() { return Err(GridBaseError::NegativeSize(size)); }
        Ok(Self { size, value })
    }

    pub fn from_fn<F>(size : Vector::<Idx,N>, mut f : F) -> Self where F : FnMut(Vector::<Idx,N>) -> T 
    {
        let s = size.area().to_usize();
        let mut values = Vec::with_capacity(s);
        for idx in 0.. s
        {
            values.push(f(unsafe { Vector::<Idx,N>::from_index_unchecked(idx, size) }));
        }
        Self { size, value: values }
    }

    /// Fill the grid with the [Default] value
    pub fn new(size : Vector::<Idx,N>) -> Self where T : Default { Self::from_fn(size, |_| ___())}
    /// Fill the grid by cloning the value
    pub fn new_uniform(size : Vector::<Idx,N>, value : T) -> Self where T : Clone { Self::from_fn(size, |_idx| value.clone()) }
}

// To avoid conflict of impl with [IGrid], [IGridView] and [IGridViewMut] when calling get(), get_mut()...
impl<T, Idx, const N : usize> GridBase<T, Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    pub fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { IGrid::get(self, pos) }
    pub fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut T> { IGrid::get_mut(self, pos) }
    
    pub unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { IGrid::get_unchecked(self, pos) } }
    pub unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut T { unsafe { IGrid::get_unchecked_mut(self, pos) } }

    pub fn swap(&mut self, pos_a : Vector<Idx,N>, pos_b : Vector<Idx,N>) -> bool { IGrid::swap(self, pos_a, pos_b) }
    pub fn replace(&mut self, val : T, pos : Vector<Idx,N>) ->  Option<T> { IGrid::replace(self, val, pos) }
    pub fn set(&mut self, val : T, pos : Vector<Idx,N>) -> &mut Self { IGrid::set(self, val, pos) }

    pub fn len(&self) -> usize { IGrid::len(self) }
}

/// Param is just used to know if it is clonable or not because of [GridParam]
pub trait IGrid<T, Param, Idx, const N : usize> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, 
    Self : IRectangle<Idx,N> 
        + Index<Vector<Idx,N>,Output=T> + IndexMut<Vector<Idx,N>,Output=T>
        // impl details :
        + Index<usize,Output=T> + IndexMut<usize,Output=T>
{
    // Todo : Don't expose how to grid is stored inside ?
    fn values(&self) -> &[T];
    fn values_mut(&mut self) -> &mut [T];
    fn into_values(self) -> Vec<T>;

    #[inline] fn is_index_inside(&self, index : usize) -> bool { index < self.area().to_usize()  }
    #[inline] fn is_index_outside(&self, index : usize) -> bool { !self.is_index_inside(index) }

    unsafe fn position_to_index_unchecked(&self, pos : Vector<Idx,N>) -> usize { unsafe { Vector::<Idx,N>::to_index_unchecked(pos, self.size()) } }
    unsafe fn index_to_position_unchecked(&self, index : usize) -> Vector<Idx,N> { unsafe { Vector::<Idx,N>::from_index_unchecked(index, self.size()) } }

    fn position_to_index(&self, pos : Vector<Idx,N>) -> Option<usize> { Vector::<Idx,N>::to_index(pos, self.size()) }
    fn index_to_position(&self, index : usize) -> Option<Vector<Idx,N>> { Vector::<Idx,N>::from_index(index, self.size()) }

    fn get_index(&self, index : usize) -> Option<&T> { self.values().get(index) }
    fn get_index_mut(&mut self, index : usize) -> Option<&mut T> { self.values_mut().get_mut(index) }

    fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { self.position_to_index(pos).and_then(|i| self.get_index(i)) }
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut T> { self.position_to_index(pos).and_then(|i| self.get_index_mut(i)) }

    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { let idx = self.position_to_index_unchecked(pos); self.values().get_unchecked(idx) } }
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut T { unsafe { let idx = self.position_to_index_unchecked(pos); self.values_mut().get_unchecked_mut(idx)} }

    fn swap(&mut self, pos_a : Vector<Idx,N>, pos_b : Vector<Idx,N>) -> bool
    {
        match (self.position_to_index(pos_a), self.position_to_index(pos_b))
        {
            (Some(a), Some(b)) => { self.values_mut().swap(a, b); true }
            _ => false
        }
    }

    fn swap_index(&mut self, index_a : usize, index_b : usize) -> bool
    {
        if self.is_index_inside(index_a) && self.is_index_inside(index_b)
        {
            self.values_mut().swap(index_a, index_b);
            true
        }else { false }
    }

    fn replace_index(&mut self, val : T, index : usize) -> Option<T> { self.get_index_mut(index).map(|v| std::mem::replace(v, val)) }
    fn replace(&mut self, val : T, pos : Vector<Idx,N>) ->  Option<T> { self.get_mut(pos).map(|v| std::mem::replace(v, val)) }
    
    /// Do nothings if the index is outside the range
    fn set_index(&mut self, val : T, idx : usize) -> &mut Self { self.get_index_mut(idx).map(|v| *v = val); self }
    /// Do nothings if the index is outside the range
    fn set(&mut self, val : T, pos : Vector<Idx,N>) -> &mut Self { self.get_mut(pos).map(|v| *v = val); self }

    type Map<Dest>;
    /// map a function on each tile to create a new grid
    fn map<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, Param : Clone;

    /// `map_into`, transform the current grid by value
    fn transform<Dest, F>(self, f : F) -> Self::Map<Dest> where F : FnMut(T) -> Dest, Param : Clone, Self : Sized;

    fn intersect_rect(&self, r : Rectangle<Idx,N>) -> Rectangle<Idx,N>  where Vector<Idx,N> : UnitArithmetic, Idx : PartialOrd { r.intersect_or_empty(self.rect()) }

    fn len(&self) -> usize { self.values().len() }

    fn crop_margin(&self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> Self where T : Clone, Param : Clone, Self : Sized;

    type View<'a> : IGridView<T,Param,Idx,N> where Self: 'a;
    fn view<'a>(&'a self) -> Self::View<'a>;

    type ViewMut<'a> : IGridView<T,Param,Idx,N> where Self: 'a;
    fn view_mut<'a>(&'a mut self) -> Self::ViewMut<'a>;
}

impl<T, Idx, const N : usize> IGrid<T,(),Idx,N> for GridBase<T, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
{
    fn values(&self) -> &[T] { &self.value }
    fn values_mut(&mut self) -> &mut [T] { &mut self.value }

    fn into_values(self) -> Vec<T> { self.value }

    type Map<Dest> = GridBase<Dest, Idx, N>;
    fn map<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest { GridBase { size: self.size, value: self.value.iter().map(f).collect() } }

    fn transform<Z, F>(self, f : F) -> GridBase<Z, Idx, N> where F : FnMut(T) -> Z { GridBase { size: self.size, value: self.value.into_iter().map(f).collect() } }

    fn crop_margin(&self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> Self where T : Clone { self.view().crop_margin(margin_start, margin_end).to_grid() }

    type View<'a> = grid::GridView<'a, T,Idx,N> where Self: 'a;
    fn view<'a>(&'a self) -> grid::GridView<'a, T,Idx,N> { grid::GridView::from_grid(self) }

    type ViewMut<'a> = grid::GridViewMut<'a, T,Idx,N> where Self: 'a;
    fn view_mut<'a>(&'a mut self) -> grid::GridViewMut<'a, T,Idx,N> { grid::GridViewMut::from_grid(self) }
}

impl<T, Idx, const N : usize> IGridView<T,(),Idx,N> for GridBase<T, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
{
    fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { self.get(pos) }
    unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { self.get_unchecked(pos) } }

    
    type ToGrid=Self;
    fn to_grid(self) -> Self::ToGrid where T : Clone { self }
    
    type SubView<'b> = GridView<'b,T,Idx,N> where Self: 'b;
    fn subview<'a>(&'a self, rect : Rectangle<Idx, N>) -> Self::SubView<'a> where T : Clone { GridView::new(self, rect) }
    fn subgrid(&self, rect : Rectangle<Idx, N>) -> Self::ToGrid where T : Clone, Self : Sized { self.view().subgrid(rect) }
}

impl<T, Idx, const N : usize> IRectangle<Idx, N> for GridBase<T, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>,
{
    fn size(&self) -> Vector<Idx, N> { self.size }
    fn begin(&self) -> Vector<Idx,N> { zero() }

    fn iter_x(&self) -> Range<Idx> where Vector<Idx,N> : HaveX<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_x() }
    fn iter_y(&self) -> Range<Idx> where Vector<Idx,N> : HaveY<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_y() }
    fn iter_z(&self) -> Range<Idx> where Vector<Idx,N> : HaveZ<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_z() }
    fn iter_w(&self) -> Range<Idx> where Vector<Idx,N> : HaveW<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_w() }

    #[inline] fn is_inside_x(&self, x : Idx) -> bool where Vector<Idx,N> : HaveX<Idx> { x >= Idx::ZERO && x < self.size_x() }
    #[inline] fn is_inside_y(&self, y : Idx) -> bool where Vector<Idx,N> : HaveY<Idx> { y >= Idx::ZERO && y < self.size_y() }
    #[inline] fn is_inside_z(&self, z : Idx) -> bool where Vector<Idx,N> : HaveZ<Idx> { z >= Idx::ZERO && z < self.size_z() }
    #[inline] fn is_inside_w(&self, w : Idx) -> bool where Vector<Idx,N> : HaveW<Idx> { w >= Idx::ZERO && w < self.size_w() }
}

impl<T, Idx, const N : usize> IGridViewMut<T,(),Idx,N> for GridBase<T, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx> 
{
    fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut T> { self.get_mut(pos) }
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut T { unsafe { self.get_unchecked_mut(pos) } }

    fn swap(&mut self, pos_a : Vector<Idx,N>, pos_b : Vector<Idx,N>) -> bool { self.swap(pos_a, pos_b) }
    fn replace(&mut self, val : T, pos : Vector<Idx,N>) ->  Option<T> { self.replace(val, pos) }
    fn set(&mut self, val : T, pos : Vector<Idx,N>) -> &mut Self { self.set(val, pos) }
    
    type SubViewMut<'b> = GridViewMut<'b,T,Idx,N> where Self: 'b;
    fn subview_mut<'a>(&'a mut self, rect : Rectangle<Idx, N>) -> Self::SubViewMut<'a> { GridViewMut::new(self, rect) }
}

impl<T, Idx, const N : usize> Index<usize> for GridBase<T, Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    type Output=T;
    fn index(&self, index: usize) -> &Self::Output { self.get_index(index).unwrap() }
}
impl<T, Idx, const N : usize> IndexMut<usize> for GridBase<T, Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_index_mut(index).unwrap() }
}

impl<T, Idx, const N : usize> Index<Vector<Idx,N>> for GridBase<T, Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get(index).unwrap() }
}
impl<T, Idx, const N : usize> IndexMut<Vector<Idx,N>> for GridBase<T, Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    fn index_mut(&mut self, index: Vector<Idx,N>) -> &mut Self::Output { self.get_mut(index).unwrap() }
}

impl<T, Idx, const N : usize> GridBase<T, Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    pub fn iter(&self) -> Iter<'_,T,Idx,N> { self.into_iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_,T,Idx,N> { self.into_iter() }
}

impl<T, Idx, const N : usize> Length for GridBase<T, Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{ 
    fn len(&self) -> usize { self.len() }
}
/* 
impl<T, Idx, const N : usize> GridOf<T, N, I> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>
{
    /// map a function on each tile to create a new grid
    pub fn map<Z, F>(&self, f : F) -> GridOf<Z, N, I> where F : FnMut(&T) -> Z { GridOf { size: self.size, value: self.value.iter().map(f).collect() } }
    /// transform the current grid
    pub fn map_into<Z, F>(self, f : F) -> GridOf<Z, N, I> where F : FnMut(T) -> Z { GridOf { size: self.size, value: self.value.into_iter().map(f).collect() } }

    pub fn intersect_rect(&self, r : Rectangle<Idx,N>) -> Rectangle<Idx,N>  where Vector<Idx,N> : UnitArithmetic, I : PartialOrd { r.intersect_or_empty(self.rect()) }
}
*/




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

        let subgrid = grid.subgrid(size.to_rect());

        assert_eq!(grid, subgrid);

        let smaller_size = point2(1, 2);
        let smaller_grid = Grid2::from_fn(smaller_size, |p|  p.x + 10 * p.y);
        let smaller_grid_from_bigger_grid = grid.subgrid(smaller_size.to_rect());
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
        assert_eq!(grid1.view().subview(rect), grid2.view().subview(rect));
        assert_eq!(grid1.view_mut().subview(rect), grid2.view_mut().subview(rect));
        assert_eq!(grid1.view_mut().subview_mut(rect), grid2.view_mut().subview_mut(rect));
    }
}