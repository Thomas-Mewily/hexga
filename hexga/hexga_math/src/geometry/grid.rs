use crate::*;

// Todo : remettre le trait IterArea pour la grille les + Vecteur, + faire les itérateur sur la grille pour itérer dans un sous rectangle 

/*
Todo : impl AsMut<&[T]> AsRef<&[T]>
+ method as_mut_slice(&self) as_slice(&mut self)



pub type Grid1<T> = Grid<T, Point1, 1>;
pub type Grid2<T> = Grid<T, Point2, 2>;
pub type Grid3<T> = Grid<T, Point3, 3>;
pub type Grid4<T> = Grid<T, Point4, 4>;

 

/// A N dimensional grid
/// 
/// - pos refer to a N dimensional index
/// - idx refer to usize
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Grid<T, V : Integer, const N : usize>
{
    size   : V,
    values : Vec<T>,
}


impl<'a, V : Integer, T, const N : usize> IntoIterator for &'a mut Grid<T, V, N> 
{
    type Item = <&'a mut Vec<T> as IntoIterator>::Item;
    type IntoIter = <&'a mut Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.values).into_iter()
    }
}

impl<'a, V : Integer, T, const N : usize> IntoIterator for &'a Grid<T, V, N> 
{
    type Item = <&'a Vec<T> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.values).into_iter()
    }
}

impl<V : Integer, T, const N : usize> IntoIterator for Grid<T, V, N> 
{
    type Item = <Vec<T> as IntoIterator>::Item;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

#[derive(Clone)]
pub enum GridError<V : Integer, const N : usize>
{
    NegativeSize(V),
    /// (dim, got)
    WrongDimension(V,usize),
}

impl<V : Integer, const N : usize> Debug for GridError<V,N> where V : Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult {
        match self {
            Self::NegativeSize(size) => write!(f, "grid have a negative size {:?}", size),
            Self::WrongDimension(size, got) => write!(f, "grid vector have a wrong dimension : expected {:?} elements for a {:?} grid but only got {:?} elements", size.area(), size, got),
        }
    }
}

impl<V : IVectorIndex<N>, T, const N : usize> Grid<T, V, N> 
{
    pub fn new_from_vec(size : V, values : Vec<T>) -> Option<Self> { Self::try_new_from_vec(size, values).ok() }
    pub fn try_new_from_vec(size : V, values : Vec<T>) -> Result<Self, GridError<V,N>>
    {
        if size.min_element() <= V::Value::ZERO { return Err(GridError::NegativeSize(size)); }
        if size.area().to_usize() as usize != values.len() { return Err(GridError::NegativeSize(size)); }
        Ok(Self { size, values })
    }

    pub fn new_map<F>(size : V, mut f : F) -> Self where F : FnMut(V) -> T 
    {
        let s = size.area().to_usize();
        let mut values = Vec::with_capacity(s);
        for idx in 0.. s
        {
            values.push(f(unsafe { V::from_idx_unchecked(idx, size) }));
        }
        Self { size, values }
    }

    /// Fill the grid with the [Default] element
    pub fn new(size : V) -> Self where T : Default { Self::new_map(size, |_| ___())}
    pub fn new_with(size : V, value : T) -> Self where T : Clone { Self::new_map(size, |_idx| value.clone()) }

    #[inline] pub fn size  (&self) -> V { self.size }
    #[inline] pub fn size_x(&self) -> V::Value where V : VectorHaveX<N> { self.size.x() }
    #[inline] pub fn size_y(&self) -> V::Value where V : VectorHaveY<N> { self.size.y() }
    #[inline] pub fn size_z(&self) -> V::Value where V : VectorHaveZ<N> { self.size.z() }
    #[inline] pub fn size_w(&self) -> V::Value where V : VectorHaveW<N> { self.size.w() }

    pub fn rect(&self) -> Rectangle<V, N> { Rectangle::new(zero(), self.size()) }
    #[inline] pub fn area (&self) -> V::Value { unsafe { self.size.unchecked_area() } }

    /// size_x
    #[inline] pub fn width (&self) -> V::Value where V : VectorHaveX<N> { self.size_x() }
    /// size_y
    #[inline] pub fn height(&self) -> V::Value where V : VectorHaveY<N> { self.size_y() }
    /// size_z
    #[inline] pub fn depth (&self) -> V::Value where V : VectorHaveZ<N> { self.size_z() }
}

impl<V : IVectorIndex<N>, T, const N : usize> Grid<T, V, N> 
{
    pub fn iter(&self) -> impl Iterator<Item = &T> + DoubleEndedIterator + Clone { self.into_iter() }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + DoubleEndedIterator { self.into_iter() }

    pub fn iter_with_idx(&self) -> impl Iterator<Item=(usize, &T)> + Clone { self.iter().enumerate() }
    pub fn iter_with_idx_mut(&mut self) -> impl Iterator<Item=(usize, &mut T)> { self.iter_mut().enumerate() }

    pub fn iter_with_pos(&self) -> impl Iterator<Item=(V, &T)> { self.iter().enumerate().map(|(idx, v)| (unsafe { V::from_idx_unchecked(idx, self.size) }, v)) }
    pub fn iter_with_pos_mut(&mut self) -> impl Iterator<Item=(V, &mut T)> { let s = self.size; self.iter_mut().enumerate().map(move |(idx, v)| (unsafe { V::from_idx_unchecked(idx, s) }, v)) }

    pub fn iter_pos(&self) -> <V as IterArea<N>>::IntoIterArea { self.size.into_iter_area() }
    
    pub fn intersect_rect(&self, r : Rectangle<V,N>) -> Rectangle<V,N> { r.intersect(self.rect()) }


    //pub fn iter_pos_in_rect(&self, r : Rectangle<V,N>) -> impl Iterator<Item=V> { self.intersect_rect(r).into  }
}
impl<V : IVectorIndex<N>, T, const N : usize> Grid<T, V, N> 
{
    pub fn iter_x(&self) -> Range<V::Value> where V : VectorHaveX<N> { V::Value::ZERO..self.size.x() }
    pub fn iter_y(&self) -> Range<V::Value> where V : VectorHaveY<N> { V::Value::ZERO..self.size.y() }
    pub fn iter_z(&self) -> Range<V::Value> where V : VectorHaveZ<N> { V::Value::ZERO..self.size.z() }
    pub fn iter_w(&self) -> Range<V::Value> where V : VectorHaveW<N> { V::Value::ZERO..self.size.w() }

    #[inline] pub fn is_inside_x(&self, x : V::Value) -> bool where V : VectorHaveX<N> { x >= V::Value::ZERO && x < self.size.x() }
    #[inline] pub fn is_inside_y(&self, y : V::Value) -> bool where V : VectorHaveY<N> { y >= V::Value::ZERO && y < self.size.y() }
    #[inline] pub fn is_inside_z(&self, z : V::Value) -> bool where V : VectorHaveZ<N> { z >= V::Value::ZERO && z < self.size.z() }
    #[inline] pub fn is_inside_w(&self, w : V::Value) -> bool where V : VectorHaveW<N> { w >= V::Value::ZERO && w < self.size.w() }

    #[inline] pub fn is_outside_x(&self, x : V::Value) -> bool where V : VectorHaveX<N> { !self.is_inside_x(x) }
    #[inline] pub fn is_outside_y(&self, y : V::Value) -> bool where V : VectorHaveY<N> { !self.is_inside_y(y) }
    #[inline] pub fn is_outside_z(&self, z : V::Value) -> bool where V : VectorHaveZ<N> { !self.is_inside_z(z) }
    #[inline] pub fn is_outside_w(&self, w : V::Value) -> bool where V : VectorHaveW<N> { !self.is_inside_w(w) }
}

impl<V : IVectorIndex<N>, T, const N : usize> Grid<T, V, N> 
{
    #[inline] pub fn is_inside(&self, idx : V) -> bool { idx.is_inside(self.size) }
    #[inline] pub fn is_outside(&self, idx : V) -> bool { !self.is_inside(idx) }

    #[inline] pub fn is_idx_inside(&self, idx : usize) -> bool { idx < self.area().to_usize()  }
    #[inline] pub fn is_idx_outside(&self, idx : usize) -> bool { !self.is_idx_inside(idx) }

    pub fn point_to_idx(&self, pos : V) -> Option<usize> { V::to_idx(pos, self.size) }
    pub fn idx_to_point(&self, idx : usize) -> Option<V> { V::from_idx(idx, self.size) }

    pub fn get_idx(&self, idx : usize) -> Option<&T> { self.values.get(idx) }
    pub fn get_idx_mut(&mut self, idx : usize) -> Option<&mut T> { self.values.get_mut(idx) }

    pub fn get(&self, pos : V) -> Option<&T> { self.point_to_idx(pos).and_then(|i| self.get_idx(i)) }
    pub fn get_mut(&mut self, pos : V) -> Option<&mut T> { self.point_to_idx(pos).and_then(|i| self.get_idx_mut(i)) }

    pub fn swap(&mut self, pos_a : V, pos_b : V) -> bool
    {
        match (self.point_to_idx(pos_a), self.point_to_idx(pos_b))
        {
            (Some(a), Some(b)) => { self.values.swap(a, b); true }
            _ => false
        }
    }

    pub fn swap_idx(&mut self, idx_a : usize, idx_b : usize) -> bool
    {
        if self.is_idx_inside(idx_a) && self.is_idx_inside(idx_b)
        {
            self.values.swap(idx_a, idx_b);
            true
        }else { false }
    }

    pub fn replace_idx(&mut self, val : T, idx : usize) -> Option<T> { self.get_idx_mut(idx).map(|v| std::mem::replace(v, val)) }
    pub fn replace(&mut self, val : T, pos : V) ->  Option<T> { self.get_mut(pos).map(|v| std::mem::replace(v, val)) }
    
    /// Do nothings if the index is outside the range
    pub fn set_idx(&mut self, val : T, idx : usize) -> &mut Self { self.get_idx_mut(idx).map(|v| *v = val); self }
    /// Do nothings if the index is outside the range
    pub fn set(&mut self, val : T, pos : V) -> &mut Self { self.get_mut(pos).map(|v| *v = val); self }

    /// map a function on each tile to create a new grid
    pub fn map<Z, F>(&self, f : F) -> Grid<Z, V, N> where F : FnMut(&T) -> Z { Grid { size: self.size, values: self.values.iter().map(f).collect() } }
    /// transform the current grid
    pub fn map_into<Z, F>(self, f : F) -> Grid<Z, V, N> where F : FnMut(T) -> Z { Grid { size: self.size, values: self.values.into_iter().map(f).collect() } }

    pub fn sub_grid(&self, r : Rectangle<V, N>) -> Self where T : Clone
    {
        let r = self.intersect_rect(r);
        Self::new_map(r.size(), |p| self[p + r.pos].clone())
    }
}

impl<V : IVectorIndex<N>, T, const N : usize> HaveLen for Grid<T, V, N>
{ 
    fn len(&self) -> usize { self.values.len() }
}
impl<V : IVectorIndex<N>, T, const N : usize> Index<usize> for Grid<T, V, N>
{
    type Output=T;
    fn index(&self, index: usize) -> &Self::Output { self.get_idx(index).unwrap() }
}
impl<V : IVectorIndex<N>, T, const N : usize> IndexMut<usize> for Grid<T, V, N>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_idx_mut(index).unwrap() }
}

impl<V : IVectorIndex<N>, T, const N : usize> Index<V> for Grid<T, V, N>
{
    type Output=T;
    fn index(&self, index: V) -> &Self::Output { self.get(index).unwrap() }
}
impl<V : IVectorIndex<N>, T, const N : usize> IndexMut<V> for Grid<T, V, N>
{
    fn index_mut(&mut self, index: V) -> &mut Self::Output { self.get_mut(index).unwrap() }
}
    */