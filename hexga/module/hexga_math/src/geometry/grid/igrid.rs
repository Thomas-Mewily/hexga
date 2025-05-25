use crate::*;

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

pub trait IGrid<T, Idx, const N : usize> : 
    Get<Idx,Output = T> + GetMut<Idx,Output = T> + GetManyMut<Idx,Output=T>
    + Index<Vector<Idx,N>, Output = T>
    + IndexMut<Vector<Idx,N>, Output = T>
    + IRectangle<Idx, N>
    + AsRef<[T]>
    + AsMut<[T]>
    + Sized
    where Idx : Integer
{
    fn values(&self) -> &[T];
    fn values_mut(&mut self) -> &mut [T];
    fn into_values(self) -> Vec<T> { self.into_size_and_values().1 }
    fn into_size_and_values(self) -> (Vector<Idx,N>, Vec<T>);

    unsafe fn position_to_index_unchecked(&self, pos : Vector<Idx,N>) -> usize { unsafe { Vector::<Idx,N>::to_index_unchecked(pos, self.size()) } }
    unsafe fn index_to_position_unchecked(&self, index : usize) -> Vector<Idx,N> { unsafe { Vector::<Idx,N>::from_index_unchecked(index, self.size()) } }

    fn from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Option<Self> { Self::try_from_vec(size, value).ok() }
    fn try_from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Result<Self, GridBaseError<Idx,N>>
    {
        if *size.min_element() <= Idx::ZERO { return Err(GridBaseError::NegativeSize(size)); }
        let area_size = size.area_usize();
        if area_size != value.len() { return Err(GridBaseError::WrongDimension(size, area_size)); }
        Ok(unsafe { Self::unchecked_from_vec(size, value) })
    }

    unsafe fn unchecked_from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Self;

    /// Create a grid from a function
    fn from_fn<F>(size : Vector::<Idx,N>, mut f : F) -> Self where F : FnMut(Vector::<Idx,N>) -> T 
    {
        let area = size.area_usize();
        let mut value = Vec::with_capacity(area);
        for idx in size.iter_index()
        {
            value.push(f(idx));
        }
        unsafe { Self::unchecked_from_vec(size, value) }
    }

    /// Fill the grid with the [Default] value
    fn new(size : Vector::<Idx,N>) -> Self where T : Default { Self::try_from_vec(size, (0..size.area_usize()).map(|_| ___()).collect()).unwrap() }

    /// Fill the grid by cloning the value
    fn new_uniform(size : Vector::<Idx,N>, value : T) -> Self where T : Clone { Self::try_from_vec(size, vec![value; size.area_usize()]).unwrap() }
}


pub trait IGridAccess<T, Idx, const N : usize> where Idx : Integer
{
    fn iter(&self) -> Iter<'_, Self, T, Idx, N> { Iter::new(self) }
    fn iter_mut(&mut self) -> IterMut<'_, Self, T, Idx, N> { IterMut::new(self) }
}


pub struct Iter<'a, G, T, Idx, const N : usize> where G : IGrid<T,Idx,N>, Idx : Integer, T:'a
{
    grid : &'a G,
    idx : RectangleIter<Idx,N>,
    phantom : std::marker::PhantomData<T>,
}

impl<'a, G, T, Idx, const N : usize> Iter<'a, G, T, Idx, N> where G : IGrid<T,Idx,N>, Idx : Integer, T:'a
{
    pub fn new(grid : &'a G) -> Self {
        Self {
            grid,
            idx : RectangleIter::new(zero(), grid.size().iter_index()),
            phantom : std::marker::PhantomData,
        }
    }

    pub fn from_rect(grid : &'a G, rect : Rectangle<Idx,N>) -> Option<Self>
    {
        if !grid.rect().is_rect_inside(rect) {
            return None;
        }else
        {
            Some(Self {
                grid,
                idx : rect,
                phantom : std::marker::PhantomData,
            })
        }
    }

    pub fn from_rect_intersect(grid : &'a G, rect : Rectangle<Idx,N>) -> Self 
    {
        Self {
            grid,
            idx : grid.rect().intersect_or_empty(rect),
            phantom : std::marker::PhantomData,
        }
    }
}

impl<'a, G, T, Idx, const N : usize> Clone for Iter<'a, G, T, Idx, N> where G : IGrid<T, Idx, N>, Idx : Integer
{
    fn clone(&self) -> Self {
        Self {
            grid: self.grid,
            idx: self.idx.clone(),
            phantom: std::marker::PhantomData,
        }
    }
}
impl<'a, G, T, Idx, const N : usize> Iterator for Iter<'a, G, T, Idx, N> where G : IGrid<T, Idx, N>, Idx : Integer
{
    type Item = (Vector<Idx,N>, &'a T);

    fn next(&mut self) -> Option<Self::Item> 
    {
        let idx = self.idx.next()?;
        Some((idx, unsafe { self.grid.get_unchecked(idx) }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.idx.size_hint()
    }
}





pub struct IterMut<'a, G, T, Idx, const N : usize> where G : IGrid<T,Idx,N>, Idx : Integer, T:'a
{
    grid : &'a mut G,
    idx : RectangleIter<Idx,N>,
    phantom : std::marker::PhantomData<T>,
}

impl<'a, G, T, Idx, const N : usize> IterMut<'a, G, T, Idx, N> where G : IGrid<T,Idx,N>, Idx : Integer, T:'a
{
    pub fn new(grid : &'a mut G) -> Self {
        Self {
            grid,
            idx : RectangleIter::new(zero(), grid.size().iter_index()),
            phantom : std::marker::PhantomData,
        }
    }

    pub fn from_rect(grid : &'a mut G, rect : Rectangle<Idx,N>) -> Option<Self>
    {
        if !grid.rect().is_rect_inside(rect) {
            return None;
        }else
        {
            Some(Self {
                grid,
                idx : rect,
                phantom : std::marker::PhantomData,
            })
        }
    }

    pub fn from_rect_intersect(grid : &'a mut G, rect : Rectangle<Idx,N>) -> Self 
    {
        Self {
            grid,
            idx : grid.rect().intersect_or_empty(rect),
            phantom : std::marker::PhantomData,
        }
    }
}

impl<'a, G, T, Idx, const N : usize> Iterator for IterMut<'a, G, T, Idx, N> where G : IGrid<T, Idx, N>, Idx : Integer
{
    type Item = (Vector<Idx,N>, &'a T);

    fn next(&mut self) -> Option<Self::Item> 
    {
        let idx = self.idx.next()?;
        Some((idx, unsafe { self.grid.get_unchecked(idx) }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.idx.size_hint()
    }
}