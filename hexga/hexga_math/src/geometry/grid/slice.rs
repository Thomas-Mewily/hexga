use crate::*;

/// A slice view to a [Grid]
pub trait ISlice<T, const N : usize, I> : Index<Vector<I,N>> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    /* 
    // Can't be easily optimized (they will call `self.get(pos)`), so it is better to omit them
    fn get_index(&self, index : usize) -> Option<&T>;
    fn is_index_inside(&self, index : usize) -> bool { index < self.area().to_usize()  }
    fn is_index_outside(&self, index : usize) -> bool { !self.is_index_inside(index) }
    */

    /// The Zero is located at the `position()`/`begin()` 
    fn get(&self, pos : Vector<I,N>) -> Option<&T>;

    fn is_inside(&self, pos : Vector<I,N>) -> bool { pos.is_inside(self.size()) }
    fn is_outside(&self, pos : Vector<I,N>) -> bool { !self.is_inside(pos) }

    fn area(&self) -> I { self.size().area() }

    // same as `.begin()`
    fn position(&self) -> Vector<I,N> { self.begin() }
    fn begin(&self) -> Vector<I,N>;
    fn end(&self) -> Vector<I,N> { self.begin() + self.size() }

    fn rect(&self) -> Rectangle<I, N> { Rectangle::new(self.begin(), self.size()) }

    fn size(&self) -> Vector<I,N>;
    #[inline] fn size_x(&self) -> I where Vector<I,N> : HaveX<I> { self.size().x() }
    #[inline] fn size_y(&self) -> I where Vector<I,N> : HaveY<I> { self.size().y() }
    #[inline] fn size_z(&self) -> I where Vector<I,N> : HaveZ<I> { self.size().z() }
    #[inline] fn size_w(&self) -> I where Vector<I,N> : HaveW<I> { self.size().w() }

    /// Same as `.size_x()`
    #[inline] fn width (&self) -> I where Vector<I,N> : HaveX<I> { self.size_x() }
    /// Same as `.size_y()`
    #[inline] fn height(&self) -> I where Vector<I,N> : HaveY<I> { self.size_y() }
    /// Same as `.size_z()`
    #[inline] fn depth (&self) -> I where Vector<I,N> : HaveZ<I> { self.size_z() }
}



/// A slice inside a [Grid]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Slice<'a, T, const N : usize, I> where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I>
{
    grid : &'a GridBase<T,N,I>,
    view : Rectangle<I,N>,
}

impl<'a, T, const N : usize, I> Slice<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    pub fn from_grid(grid : &'a GridBase<T,N,I>) -> Self 
    {
        Self { grid, view: grid.rect() }
    }
    pub fn new(grid : &'a GridBase<T,N,I>, view : Rectangle<I,N>) -> Self 
    {
        let view = grid.rect().intersect_or_empty(view);
        Self { grid, view }
    }
}


impl<'a, T, const N : usize, I> ISlice<T,N,I> for Slice<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    fn get(&self, pos : Vector<I,N>) -> Option<&T> { self.grid.get(self.view.pos + pos) }
    fn size(&self) -> Vector<I,N> { self.view.size() }
    fn begin(&self) -> Vector<I,N> { self.view.pos }
}

impl<'a, T, const N : usize, I> Index<Vector<I,N>> for Slice<'a, T, N, I> 
    where I : IntegerIndex, usize : CastTo<I>, isize : CastTo<I> 
{
    type Output=T;
    fn index(&self, index: Vector<I,N>) -> &Self::Output { self.get(index).unwrap() }
}


