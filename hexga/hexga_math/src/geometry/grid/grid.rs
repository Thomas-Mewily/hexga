use crate::*;

// Todo : remettre le trait IterArea pour la grille les + Vecteur, + faire les itérateur sur la grille pour itérer dans un sous rectangle 

/* 
Todo : impl AsMut<&[T]> AsRef<&[T]>
+ method as_mut_slice(&self) as_slice(&mut self)
*/

/// A N dimensional grid
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GridBase<T, const N : usize, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    size  : Vector<I,N>,
    value : Vec<T>,
}

#[derive(Debug)]
pub struct IterMut<'a, T, const N : usize,I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    value : std::iter::Enumerate<std::slice::IterMut<'a, T>>,
    size  : Vector<I,N>,
}
impl<'a, T, const N : usize, I> IterMut<'a,T,N,I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    pub fn new(grid : &'a mut GridBase<T,N,I>) -> Self { Self { value: grid.value.iter_mut().enumerate(), size: grid.size }}
}
impl<'a, T, const N : usize, I> Iterator for IterMut<'a,T,N,I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Item=(Vector<I,N>, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        self.value.next().map(|(idx, v)| (unsafe { Vector::<I,N>::from_index_unchecked(idx, self.size) }, v))
    }
}

impl<'a, T, const N : usize, I> IntoIterator for &'a mut GridBase<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Item = (Vector<I,N>, &'a mut T);
    type IntoIter = IterMut::<'a,T,N,I>;

    fn into_iter(self) -> Self::IntoIter { IterMut::new(self) }
}


#[derive(Clone, Debug)]
pub struct Iter<'a, T, const N : usize, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    value : std::iter::Enumerate<std::slice::Iter<'a, T>>,
    size  : Vector<I,N>,
}
impl<'a, T, const N : usize, I> Iter<'a,T,N,I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    pub fn new(grid : &'a  GridBase<T,N,I>) -> Self { Self { value: grid.value.iter().enumerate(), size: grid.size }}
}
impl<'a, T, const N : usize, I> Iterator for Iter<'a,T,N,I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Item=(Vector<I,N>, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.value.next().map(|(idx, v)| (unsafe { Vector::<I,N>::from_index_unchecked(idx, self.size) }, v))
    }
}

impl<'a, T, const N : usize, I> IntoIterator for &'a GridBase<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Item = (Vector<I,N>, &'a  T);
    type IntoIter = Iter::<'a,T,N,I>;

    fn into_iter(self) -> Self::IntoIter { Iter::new(self) }
}



#[derive(Debug)]
pub struct IntoIter<T, const N : usize, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    value : std::iter::Enumerate<std::vec::IntoIter<T>>,
    size  : Vector<I,N>,
}
impl<T, const N : usize, I> IntoIter<T,N,I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    pub fn new(grid : GridBase<T,N,I>) -> Self { Self { value: grid.value.into_iter().enumerate(), size: grid.size }}
}
impl<T, const N : usize, I> Iterator for IntoIter<T,N,I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Item=(Vector<I,N>, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.value.next().map(|(idx, v)| (unsafe { Vector::<I,N>::from_index_unchecked(idx, self.size) }, v))
    }
}

impl<T, const N : usize, I> IntoIterator for GridBase<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Item = (Vector<I,N>, T);
    type IntoIter = IntoIter::<T,N,I>;
    fn into_iter(self) -> Self::IntoIter { IntoIter::new(self) }
}

 

#[derive(Clone)]
pub enum GridBaseError<const N : usize, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    NegativeSize(Vector::<I,N>),
    /// (dim, got)
    WrongDimension(Vector<I,N>, usize),
}

impl<const N : usize, I> Debug for GridBaseError<N, I> where I : Debug, I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult {
        match self {
            Self::NegativeSize(size) => write!(f, "grid have a negative size {:?}", size),
            Self::WrongDimension(size, got) => write!(f, "grid vector have a wrong dimension : expected {:?} elements for a {:?} grid but only got {:?} elements", size.area(), size, got),
        }
    }
}

impl<T, const N : usize, I> GridBase<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    pub fn from_vec(size : Vector::<I,N>, value : Vec<T>) -> Option<Self> { Self::try_from_vec(size, value).ok() }
    pub fn try_from_vec(size : Vector::<I,N>, value : Vec<T>) -> Result<Self, GridBaseError<N,I>>
    {
        if *size.min_element() <= I::ZERO { return Err(GridBaseError::NegativeSize(size)); }
        if size.area().to_usize() != value.len() { return Err(GridBaseError::NegativeSize(size)); }
        Ok(Self { size, value })
    }

    pub fn from_fn<F>(size : Vector::<I,N>, mut f : F) -> Self where F : FnMut(Vector::<I,N>) -> T 
    {
        let s = size.area().to_usize();
        let mut values = Vec::with_capacity(s);
        for idx in 0.. s
        {
            values.push(f(unsafe { Vector::<I,N>::from_index_unchecked(idx, size) }));
        }
        Self { size, value: values }
    }

    /// Fill the grid with the [Default] value
    pub fn new(size : Vector::<I,N>) -> Self where T : Default { Self::from_fn(size, |_| Default::default())}
    /// Fill the grid by cloning the value
    pub fn new_with(size : Vector::<I,N>, value : T) -> Self where T : Clone { Self::from_fn(size, |_idx| value.clone()) }
}


impl<I, T, const N : usize> GridBase<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    pub fn values(&self) -> &[T] { &self.value }
    pub fn values_mut(&mut self) -> &mut [T] { &mut self.value }
    pub fn into_values(self) -> Vec<T> { self.value }

    #[inline] pub fn is_index_inside(&self, index : usize) -> bool { index < self.area().to_usize()  }
    #[inline] pub fn is_index_outside(&self, index : usize) -> bool { !self.is_index_inside(index) }

    pub unsafe fn position_to_index_unchecked(&self, pos : Vector<I,N>) -> usize { unsafe { Vector::<I,N>::to_index_unchecked(pos, self.size) } }
    pub unsafe fn index_to_position_unchecked(&self, index : usize) -> Vector<I,N> { unsafe { Vector::<I,N>::from_index_unchecked(index, self.size) } }

    pub fn position_to_index(&self, pos : Vector<I,N>) -> Option<usize> { Vector::<I,N>::to_index(pos, self.size) }
    pub fn index_to_position(&self, index : usize) -> Option<Vector<I,N>> { Vector::<I,N>::from_index(index, self.size) }



    pub fn get_index(&self, index : usize) -> Option<&T> { self.value.get(index) }
    pub fn get_index_mut(&mut self, index : usize) -> Option<&mut T> { self.value.get_mut(index) }

    pub fn get(&self, pos : Vector<I,N>) -> Option<&T> { self.position_to_index(pos).and_then(|i| self.get_index(i)) }
    pub fn get_mut(&mut self, pos : Vector<I,N>) -> Option<&mut T> { self.position_to_index(pos).and_then(|i| self.get_index_mut(i)) }

    pub unsafe fn get_unchecked(&self, pos : Vector<I,N>) -> &T { unsafe { let idx = self.position_to_index_unchecked(pos); self.value.get_unchecked(idx) } }
    pub unsafe fn get_unchecked_mut(&mut self, pos : Vector<I,N>) -> &mut T { unsafe { let idx = self.position_to_index_unchecked(pos); self.value.get_unchecked_mut(idx)} }


    pub fn swap(&mut self, pos_a : Vector<I,N>, pos_b : Vector<I,N>) -> bool
    {
        match (self.position_to_index(pos_a), self.position_to_index(pos_b))
        {
            (Some(a), Some(b)) => { self.value.swap(a, b); true }
            _ => false
        }
    }

    pub fn swap_index(&mut self, index_a : usize, index_b : usize) -> bool
    {
        if self.is_index_inside(index_a) && self.is_index_inside(index_b)
        {
            self.value.swap(index_a, index_b);
            true
        }else { false }
    }

    pub fn replace_index(&mut self, val : T, index : usize) -> Option<T> { self.get_index_mut(index).map(|v| std::mem::replace(v, val)) }
    pub fn replace(&mut self, val : T, pos : Vector<I,N>) ->  Option<T> { self.get_mut(pos).map(|v| std::mem::replace(v, val)) }
    
    /// Do nothings if the index is outside the range
    pub fn set_index(&mut self, val : T, idx : usize) -> &mut Self { self.get_index_mut(idx).map(|v| *v = val); self }
    /// Do nothings if the index is outside the range
    pub fn set(&mut self, val : T, pos : Vector<I,N>) -> &mut Self { self.get_mut(pos).map(|v| *v = val); self }

    /// map a function on each tile to create a new grid
    pub fn map<Z, F>(&self, f : F) -> GridBase<Z, N, I> where F : FnMut(&T) -> Z { GridBase { size: self.size, value: self.value.iter().map(f).collect() } }
    /// transform the current grid
    pub fn map_into<Z, F>(self, f : F) -> GridBase<Z, N, I> where F : FnMut(T) -> Z { GridBase { size: self.size, value: self.value.into_iter().map(f).collect() } }

    pub fn intersect_rect(&self, r : Rectangle<I,N>) -> Rectangle<I,N>  where Vector<I,N> : UnitArithmetic, I : PartialOrd { r.intersect_or_empty(self.rect()) }

    /* 
    pub fn sub_grid(&self, r : Rectangle<I, N>) -> Self where T : Clone
    {
        let r = self.intersect_rect(r);
        Self::from_fn(r.size(), |p| self[p + r.pos].clone())
    }
    */

    pub fn size(&self) -> Vector<I,N> { self.size }
    pub fn len(&self) -> usize { self.value.len() }

    pub fn view<'a>(&'a self) -> grid::GridView<'a, T,N,I> { grid::GridView::from_grid(self) }
    pub fn view_mut<'a>(&'a mut self) -> grid::GridViewMut<'a, T,N,I> { grid::GridViewMut::from_grid(self) }

    pub fn crop_margin(&self, margin_start : Vector<I,N>, margin_end : Vector<I,N>) -> Self where T : Clone { self.view().crop_margin(margin_start, margin_end).to_grid() }
}

impl<T, const N : usize, I> IGridView<T,N,I> for GridBase<T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn get(&self, pos : Vector<I,N>) -> Option<&T> { self.get(pos) }
    unsafe fn get_unchecked(&self, pos : Vector<I,N>) -> &T { unsafe { self.get_unchecked(pos) } }

    fn subview<'a>(&'a self, rect : Rectangle<I, N>) -> GridView<'a,T,N,I> { GridView::new(self, rect) }
}

impl<T, const N : usize, I> IRectangle<I, N> for GridBase<T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>,
{
    fn size(&self) -> Vector<I, N> { self.size }
    fn begin(&self) -> Vector<I,N> { zero() }
}

impl<T, const N : usize, I> IGridViewMut<T,N,I> for GridBase<T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn get_mut(&mut self, pos : Vector<I,N>) -> Option<&mut T> { self.get_mut(pos) }
    unsafe fn get_unchecked_mut(&mut self, pos : Vector<I,N>) -> &mut T { unsafe { self.get_unchecked_mut(pos) } }

    fn subview_mut<'a>(&'a mut self, rect : Rectangle<I, N>) -> GridViewMut<'a,T,N,I> { GridViewMut::new(self, rect) }
    
    fn swap(&mut self, pos_a : Vector<I,N>, pos_b : Vector<I,N>) -> bool { self.swap(pos_a, pos_b) }
    fn replace(&mut self, val : T, pos : Vector<I,N>) ->  Option<T> { self.replace(val, pos) }
    fn set(&mut self, val : T, pos : Vector<I,N>) -> &mut Self { self.set(val, pos) }
}

impl<I, T, const N : usize> Index<usize> for GridBase<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Output=T;
    fn index(&self, index: usize) -> &Self::Output { self.get_index(index).unwrap() }
}
impl<I, T, const N : usize> IndexMut<usize> for GridBase<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_index_mut(index).unwrap() }
}

impl<I, T, const N : usize> Index<Vector<I,N>> for GridBase<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Output=T;
    fn index(&self, index: Vector<I,N>) -> &Self::Output { self.get(index).unwrap() }
}
impl<I, T, const N : usize> IndexMut<Vector<I,N>> for GridBase<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    fn index_mut(&mut self, index: Vector<I,N>) -> &mut Self::Output { self.get_mut(index).unwrap() }
}

impl<T, const N : usize, I> GridBase<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    pub fn iter(&self) -> Iter<'_,T,N,I> { self.into_iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_,T,N,I> { self.into_iter() }

    pub fn iter_x(&self) -> Range<I> where Vector<I,N> : HaveX<I> { I::ZERO..self.size.x() }
    pub fn iter_y(&self) -> Range<I> where Vector<I,N> : HaveY<I> { I::ZERO..self.size.y() }
    pub fn iter_z(&self) -> Range<I> where Vector<I,N> : HaveZ<I> { I::ZERO..self.size.z() }
    pub fn iter_w(&self) -> Range<I> where Vector<I,N> : HaveW<I> { I::ZERO..self.size.w() }

    #[inline] pub fn is_inside_x(&self, x : I) -> bool where Vector<I,N> : HaveX<I> { x >= I::ZERO && x < self.size.x() }
    #[inline] pub fn is_inside_y(&self, y : I) -> bool where Vector<I,N> : HaveY<I> { y >= I::ZERO && y < self.size.y() }
    #[inline] pub fn is_inside_z(&self, z : I) -> bool where Vector<I,N> : HaveZ<I> { z >= I::ZERO && z < self.size.z() }
    #[inline] pub fn is_inside_w(&self, w : I) -> bool where Vector<I,N> : HaveW<I> { w >= I::ZERO && w < self.size.w() }

    #[inline] pub fn is_outside_x(&self, x : I) -> bool where Vector<I,N> : HaveX<I> { !self.is_inside_x(x) }
    #[inline] pub fn is_outside_y(&self, y : I) -> bool where Vector<I,N> : HaveY<I> { !self.is_inside_y(y) }
    #[inline] pub fn is_outside_z(&self, z : I) -> bool where Vector<I,N> : HaveZ<I> { !self.is_inside_z(z) }
    #[inline] pub fn is_outside_w(&self, w : I) -> bool where Vector<I,N> : HaveW<I> { !self.is_inside_w(w) }
}

impl<T, const N : usize, I> GridBase<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    /// like a map_into
    pub fn transform<Z, F>(self, f : F) -> GridBase<Z, N, I> where F : FnMut(T) -> Z { GridBase { size: self.size, value: self.value.into_iter().map(f).collect() } }
}
/* 
impl<I, T, const N : usize> GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    /// map a function on each tile to create a new grid
    pub fn map<Z, F>(&self, f : F) -> GridOf<Z, N, I> where F : FnMut(&T) -> Z { GridOf { size: self.size, value: self.value.iter().map(f).collect() } }
    /// transform the current grid
    pub fn map_into<Z, F>(self, f : F) -> GridOf<Z, N, I> where F : FnMut(T) -> Z { GridOf { size: self.size, value: self.value.into_iter().map(f).collect() } }

    pub fn intersect_rect(&self, r : Rectangle<I,N>) -> Rectangle<I,N>  where Vector<I,N> : UnitArithmetic, I : PartialOrd { r.intersect_or_empty(self.rect()) }
}
*/

impl<T, const N : usize, I> have_len::HaveLen for GridBase<T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{ 
    fn len(&self) -> usize { self.len() }
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