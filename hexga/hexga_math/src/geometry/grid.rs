use crate::*;

// Todo : remettre le trait IterArea pour la grille les + Vecteur, + faire les itérateur sur la grille pour itérer dans un sous rectangle 

/* 
Todo : impl AsMut<&[T]> AsRef<&[T]>
+ method as_mut_slice(&self) as_slice(&mut self)
*/

pub type Grid1<T> = Grid<T, 1>;
pub type Grid2<T> = Grid<T, 2>;
pub type Grid3<T> = Grid<T, 3>;
pub type Grid4<T> = Grid<T, 4>;

pub type Grid<T,const N : usize> = GridOf<T,N,int>;

/// A N dimensional grid
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GridOf<T, const N : usize, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    size  : Vector<I,N>,
    value : Vec<T>,
}

#[derive(Debug)]
pub struct IterMut<'a, T,const N : usize,I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    value : std::iter::Enumerate<std::slice::IterMut<'a, T>>,
    size  : Vector<I,N>,
}
impl<'a, T, const N : usize, I> IterMut<'a,T,N,I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    pub fn new(grid : &'a mut GridOf<T,N,I>) -> Self { Self { value: grid.value.iter_mut().enumerate(), size: grid.size }}
}
impl<'a, T, const N : usize, I> Iterator for IterMut<'a,T,N,I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Item=(Vector<I,N>, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        self.value.next().map(|(idx, v)| (unsafe { Vector::<I,N>::from_index_unchecked(idx, self.size) }, v))
    }
}

impl<'a, I, const N : usize, T> IntoIterator for &'a mut GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
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
    pub fn new(grid : &'a  GridOf<T,N,I>) -> Self { Self { value: grid.value.iter().enumerate(), size: grid.size }}
}
impl<'a, T, const N : usize, I> Iterator for Iter<'a,T,N,I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Item=(Vector<I,N>, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.value.next().map(|(idx, v)| (unsafe { Vector::<I,N>::from_index_unchecked(idx, self.size) }, v))
    }
}

impl<'a, I, T, const N : usize> IntoIterator for &'a GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
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
    pub fn new(grid : GridOf<T,N,I>) -> Self { Self { value: grid.value.into_iter().enumerate(), size: grid.size }}
}
impl<T, const N : usize, I> Iterator for IntoIter<T,N,I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Item=(Vector<I,N>, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.value.next().map(|(idx, v)| (unsafe { Vector::<I,N>::from_index_unchecked(idx, self.size) }, v))
    }
}

impl<I, T, const N : usize> IntoIterator for GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Item = (Vector<I,N>, T);
    type IntoIter = IntoIter::<T,N,I>;
    fn into_iter(self) -> Self::IntoIter { IntoIter::new(self) }
}

 

#[derive(Clone)]
pub enum GridError<I, const N : usize> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    NegativeSize(Vector::<I,N>),
    /// (dim, got)
    WrongDimension(Vector<I,N>, usize),
}

impl<I, const N : usize> Debug for GridError<I,N> where I : Debug, I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult {
        match self {
            Self::NegativeSize(size) => write!(f, "grid have a negative size {:?}", size),
            Self::WrongDimension(size, got) => write!(f, "grid vector have a wrong dimension : expected {:?} elements for a {:?} grid but only got {:?} elements", size.area(), size, got),
        }
    }
}

impl<I, T, const N : usize> GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    pub fn from_vec(size : Vector::<I,N>, value : Vec<T>) -> Option<Self> { Self::try_from_vec(size, value).ok() }
    pub fn try_from_vec(size : Vector::<I,N>, value : Vec<T>) -> Result<Self, GridError<I,N>>
    {
        if *size.min_element() <= I::ZERO { return Err(GridError::NegativeSize(size)); }
        if size.area().to_usize() != value.len() { return Err(GridError::NegativeSize(size)); }
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

    #[inline] pub fn size  (&self) -> Vector<I,N> { self.size }
    #[inline] pub fn size_x(&self) -> I where Vector<I,N> : HaveX<I> { self.size.x() }
    #[inline] pub fn size_y(&self) -> I where Vector<I,N> : HaveY<I> { self.size.y() }
    #[inline] pub fn size_z(&self) -> I where Vector<I,N> : HaveZ<I> { self.size.z() }
    #[inline] pub fn size_w(&self) -> I where Vector<I,N> : HaveW<I> { self.size.w() }

    pub fn rect(&self) -> Rectangle<I, N> { Rectangle::new(zero(), self.size()) }
    #[inline] pub fn area (&self) -> I { self.size.area() }

    /// Same as `.size_x()`
    #[inline] pub fn width (&self) -> I where Vector<I,N> : HaveX<I> { self.size_x() }
    /// Same as `.size_y()`
    #[inline] pub fn height(&self) -> I where Vector<I,N> : HaveY<I> { self.size_y() }
    /// Same as `.size_z()`
    #[inline] pub fn depth (&self) -> I where Vector<I,N> : HaveZ<I> { self.size_z() }
}


impl<I, T, const N : usize> GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    pub fn iter(&self) -> Iter<'_,T,N,I> { self.into_iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_,T,N,I> { self.into_iter() }
}

impl<I, T, const N : usize> GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
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

/*
pub trait IGrid<I, T, const N : usize> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    fn values(&self) -> &[T];
    fn values_mut(&mut self) -> &mut [T];
    fn into_values(self) -> Vec<T>;

    
    #[inline] fn is_inside(&self, pos : Vector<I,N>) -> bool { pos.is_inside(self.size) }
    #[inline] fn is_outside(&self, pos : Vector<I,N>) -> bool { !self.is_inside(pos) }

    #[inline] fn is_index_inside(&self, index : usize) -> bool { index < self.area().to_usize()  }
    #[inline] fn is_index_outside(&self, index : usize) -> bool { !self.is_index_inside(index) }

    fn position_to_index(&self, pos : Vector<I,N>) -> Option<usize> { Vector::<I,N>::to_index(pos, self.size) }
    fn index_to_position(&self, index : usize) -> Option<Vector<I,N>> { Vector::<I,N>::from_index(index, self.size) }

    fn get_index(&self, index : usize) -> Option<&T> { self.values().get(index) }
    fn get_index_mut(&mut self, index : usize) -> Option<&mut T> { self.values().get_mut(index) }

    fn get(&self, pos : Vector<I,N>) -> Option<&T> { self.position_to_index(pos).and_then(|i| self.get_index(i)) }
    fn get_mut(&mut self, pos : Vector<I,N>) -> Option<&mut T> { self.position_to_index(pos).and_then(|i| self.get_index_mut(i)) }

    fn swap(&mut self, pos_a : Vector<I,N>, pos_b : Vector<I,N>) -> bool
    {
        match (self.position_to_index(pos_a), self.position_to_index(pos_b))
        {
            (Some(a), Some(b)) => { self.value.swap(a, b); true }
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
    fn replace(&mut self, val : T, pos : Vector<I,N>) ->  Option<T> { self.get_mut(pos).map(|v| std::mem::replace(v, val)) }
    
    /// Do nothings if the index is outside the range
    fn set_index(&mut self, val : T, idx : usize) -> &mut Self { self.get_index_mut(idx).map(|v| *v = val); self }
    /// Do nothings if the index is outside the range
    fn set(&mut self, val : T, pos : Vector<I,N>) -> &mut Self { self.get_mut(pos).map(|v| *v = val); self }

    /// map a function on each tile to create a new grid
    fn map<Z, F>(&self, f : F) -> Grid<Z, I, N> where F : FnMut(&T) -> Z { Grid { size: self.size, value: self.value.iter().map(f).collect() } }
    /// transform the current grid
    fn map_into<Z, F>(self, f : F) -> Grid<Z, I, N> where F : FnMut(T) -> Z { Grid { size: self.size, value: self.value.into_iter().map(f).collect() } }

    fn intersect_rect(&self, r : Rectangle<I,N>) -> Rectangle<I,N>  where Vector<I,N> : UnitArithmetic, I : PartialOrd { r.intersect_or_empty(self.rect()) }

    fn sub_grid(&self, r : Rectangle<I, N>) -> Self where T : Clone
    {
        let r = self.intersect_rect(r);
        Self::from_fn(r.size(), |p| self[p + r.pos].clone())
    }

    fn len(&self) -> usize { self.value.len() }
}
*/

impl<I, T, const N : usize> GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    pub fn values(&self) -> &[T] { &self.value }
    pub fn values_mut(&mut self) -> &mut [T] { &mut self.value }
    pub fn into_values(self) -> Vec<T> { self.value }

    #[inline] pub fn is_inside(&self, pos : Vector<I,N>) -> bool { pos.is_inside(self.size) }
    #[inline] pub fn is_outside(&self, pos : Vector<I,N>) -> bool { !self.is_inside(pos) }

    #[inline] pub fn is_index_inside(&self, index : usize) -> bool { index < self.area().to_usize()  }
    #[inline] pub fn is_index_outside(&self, index : usize) -> bool { !self.is_index_inside(index) }

    pub fn position_to_index(&self, pos : Vector<I,N>) -> Option<usize> { Vector::<I,N>::to_index(pos, self.size) }
    pub fn index_to_position(&self, index : usize) -> Option<Vector<I,N>> { Vector::<I,N>::from_index(index, self.size) }

    pub fn get_index(&self, index : usize) -> Option<&T> { self.value.get(index) }
    pub fn get_index_mut(&mut self, index : usize) -> Option<&mut T> { self.value.get_mut(index) }

    pub fn get(&self, pos : Vector<I,N>) -> Option<&T> { self.position_to_index(pos).and_then(|i| self.get_index(i)) }
    pub fn get_mut(&mut self, pos : Vector<I,N>) -> Option<&mut T> { self.position_to_index(pos).and_then(|i| self.get_index_mut(i)) }

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
    pub fn map<Z, F>(&self, f : F) -> GridOf<Z, N, I> where F : FnMut(&T) -> Z { GridOf { size: self.size, value: self.value.iter().map(f).collect() } }
    /// transform the current grid
    pub fn map_into<Z, F>(self, f : F) -> GridOf<Z, N, I> where F : FnMut(T) -> Z { GridOf { size: self.size, value: self.value.into_iter().map(f).collect() } }

    pub fn intersect_rect(&self, r : Rectangle<I,N>) -> Rectangle<I,N>  where Vector<I,N> : UnitArithmetic, I : PartialOrd { r.intersect_or_empty(self.rect()) }

    pub fn sub_grid(&self, r : Rectangle<I, N>) -> Self where T : Clone
    {
        let r = self.intersect_rect(r);
        Self::from_fn(r.size(), |p| self[p + r.pos].clone())
    }

    pub fn len(&self) -> usize { self.value.len() }
}

impl<I, T, const N : usize> have_len::HaveLen for GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{ 
    fn len(&self) -> usize { self.values().len() }
}


impl<I, T, const N : usize> Index<usize> for GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Output=T;
    fn index(&self, index: usize) -> &Self::Output { self.get_index(index).unwrap() }
}
impl<I, T, const N : usize> IndexMut<usize> for GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_index_mut(index).unwrap() }
}

impl<I, T, const N : usize> Index<Vector<I,N>> for GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    type Output=T;
    fn index(&self, index: Vector<I,N>) -> &Self::Output { self.get(index).unwrap() }
}
impl<I, T, const N : usize> IndexMut<Vector<I,N>> for GridOf<T, N, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    fn index_mut(&mut self, index: Vector<I,N>) -> &mut Self::Output { self.get_mut(index).unwrap() }
}