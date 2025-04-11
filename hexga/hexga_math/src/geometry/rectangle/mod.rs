use crate::*;

pub mod prelude;

pub type Rectangle<T, const N : usize> = RectangleBase<Vector<T, N>>;

/// A `N` dimension rectangle
#[repr(C)]
pub struct RectangleBase<T>
{
    pub pos  : T,
    pub size : T,
}
impl_fixed_array_like_with_op!(RectangleBase, 2);

impl<T> Default for RectangleBase<T> where T : Number
{
    fn default() -> Self { Self::SIZED_ONE }
}

impl<T> RectangleBase<T> where T : Number
{
    pub const SIZED_ONE : Self = Self::new_sized(T::ONE);
}

// 2D :
impl<T> Rectangle<T,2> where T : Number
{
    pub fn down_left (&self) -> Vector<T,2> { self.pos }
    pub fn down_right(&self) -> Vector<T,2> where T : One + Mul<T,Output = T> + Add<T, Output=T> { self.pos + self.size * Vector::<T,2>::X }
    
    /// Same as down_left()
    pub fn bottom_left (&self) -> Vector<T,2> { self.down_left() }
    /// Same as down_right()
    pub fn bottom_right(&self) -> Vector<T,2> where T : One + Mul<T,Output = T> + Add<T, Output=T> { self.down_right() }
    
    pub fn up_right  (&self) -> Vector<T,2> where T : Add<T, Output=T> { self.pos + self.size }
    pub fn up_left   (&self) -> Vector<T,2> where T : One, T : Mul<T,Output = T> + Add<T, Output=T> { self.pos + self.size * Vector::<T,2>::Y } 

    /// Same as up_right()
    pub fn top_right  (&self) -> Vector<T,2> where T : Add<T, Output=T> { self.up_right() }
    /// Same as up_left()
    pub fn top_left   (&self) -> Vector<T,2> where T : One, T : Mul<T,Output = T> + Add<T, Output=T> { self.up_left() } 
}

impl<T,const N : usize> Rectangle<T,N> where Vector<T,N> : Number, T : Number
{
    pub fn get_coef(&self, coef : Vector<T,N>) -> Vector<T,N> where Vector<T,N> : UnitArithmetic { self.pos + self.size * coef }

    /// The center of the coordinate, whatever the dimension
    pub fn middle      (&self) -> Vector<T,N> { self.pos + self.size.take_half() }
    pub fn center      (&self) -> Vector<T,N> { self.pos + self.size.take_half() }
    
    pub fn middle_right(&self) -> Vector<T,N> where Vector<T,N> : HaveX<T> { self.middle().with_x(self.right_value()) }
    pub fn middle_left (&self) -> Vector<T,N> where Vector<T,N> : HaveX<T> { self.middle().with_x(self.left_value ()) }

    pub fn middle_up   (&self) -> Vector<T,N> where Vector<T,N> : HaveY<T> { self.middle().with_y(self.up_value  ()) }
    pub fn middle_down (&self) -> Vector<T,N> where Vector<T,N> : HaveY<T> { self.middle().with_y(self.down_value()) }

    pub fn middle_forward  (&self) -> Vector<T,N> where Vector<T,N> : HaveZ<T> { self.middle().with_z(self.forward_value ()) }
    pub fn middle_backward (&self) -> Vector<T,N> where Vector<T,N> : HaveZ<T> { self.middle().with_z(self.backward_value()) }

    pub fn middle_ana  (&self) -> Vector<T,N> where Vector<T,N> : HaveW<T> { self.middle().with_w(self.ana_value ()) }
    pub fn middle_kata (&self) -> Vector<T,N> where Vector<T,N> : HaveW<T> { self.middle().with_w(self.kata_value()) }

    pub fn right_value (&self) -> T where Vector<T,N> : HaveX<T> { self.pos.x() + self.size.x() }
    pub fn left_value  (&self) -> T where Vector<T,N> : HaveX<T> { self.pos.x() }
    
    pub fn up_value    (&self) -> T where Vector<T,N> : HaveY<T> { self.pos.y() + self.size.y() }
    pub fn down_value  (&self) -> T where Vector<T,N> : HaveY<T> { self.pos.y() }
    
    /// Same as up_value()
    pub fn top_value  (&self) -> T where Vector<T,N> : HaveY<T> { self.up_value() }
    /// Same as down_value()
    pub fn bottom_value  (&self) -> T where Vector<T,N> : HaveY<T> { self.down_value() }
    
    pub fn forward_value (&self) -> T where Vector<T,N> : HaveZ<T> { self.pos.z() + self.size.z() }
    pub fn backward_value(&self) -> T where Vector<T,N> : HaveZ<T> { self.pos.z() }

    pub fn ana_value (&self) -> T where Vector<T,N> : HaveW<T> { self.pos.w() + self.size.w() }
    pub fn kata_value(&self) -> T where Vector<T,N> : HaveW<T> { self.pos.w() }




}

impl<T,const N : usize> Rectangle<T,N> where T : Copy
{
    pub fn set_size(&mut self, size : Vector<T, N>) -> &mut Self { self.size = size; self }
    pub fn set_size_x(&mut self, x : T) -> &mut Self where Vector<T, N> : HaveX<T> { self.size.set_x(x); self }
    pub fn set_size_y(&mut self, y : T) -> &mut Self where Vector<T, N> : HaveY<T> { self.size.set_y(y); self }
    pub fn set_size_z(&mut self, z : T) -> &mut Self where Vector<T, N> : HaveZ<T> { self.size.set_z(z); self }
    pub fn set_size_w(&mut self, w : T) -> &mut Self where Vector<T, N> : HaveW<T> { self.size.set_w(w); self }

    pub fn with_size(mut self, size : Vector<T, N>) -> Self { self.size = size; self }
    pub fn with_size_x(mut self, x : T) -> Self where Vector<T, N> : HaveX<T> { self.size.set_x(x); self }
    pub fn with_size_y(mut self, y : T) -> Self where Vector<T, N> : HaveY<T> { self.size.set_y(y); self }
    pub fn with_size_z(mut self, z : T) -> Self where Vector<T, N> : HaveZ<T> { self.size.set_z(z); self }
    pub fn with_size_w(mut self, w : T) -> Self where Vector<T, N> : HaveW<T> { self.size.set_w(w); self }
}

impl<T,const N : usize> Rectangle<T,N> where T : Number
{
    /// Check if a point inside the rectangle.
    /// 
    /// ```
    /// use hexga_math::prelude::*;
    /// 
    /// // inside :
    /// assert!(rect2p(0, 0, 2, 2).is_inside(point2(1, 1)));
    /// assert!(rect2p(0, 0, 2, 2).is_inside(point2(0, 0)));
    /// 
    /// assert!(rect2p(0, 0, 2, 2).is_inside(point2(2, 0)));
    /// assert!(rect2p(0, 0, 2, 2).is_inside(point2(0, 2)));
    /// assert!(rect2p(0, 0, 2, 2).is_inside(point2(2, 2)));
    /// 
    /// // not inside :
    /// 
    /// assert!(!rect2p(0, 0, 2, 2).is_inside(point2( 3,   3)));
    /// assert!(!rect2p(0, 0, 2, 2).is_inside(point2(-1,  -1)));
    /// assert!(!rect2p(0, 0, 2, 2).is_inside(point2(-1,  -1)));
    /// assert!(!rect2p(0, 0, 2, 2).is_inside(point2( 1,  10)));
    /// assert!(!rect2p(0, 0, 2, 2).is_inside(point2( 1, -10)));
    /// assert!(!rect2p(0, 0, 2, 2).is_inside(point2(-10,  1)));
    /// assert!(!rect2p(0, 0, 2, 2).is_inside(point2( 10,  1)));
    /// 
    /// ```
    pub fn is_inside(&self, point : Vector<T, N>) -> bool
    {
        self.pos().all_with(&point, |a, c| c >= a) &&
        (self.pos + self.size).all_with(&point, |a, c| c <= a)
    }
    
    pub fn clamp_vector(&self, vector : Vector<T, N>) -> Vector<T, N>
    { vector.max(self.min()).min(self.max()) }
    
    pub fn intersect_or_empty(self, other : Self) -> Self
    {
        Self::from_pos_to_pos(self.min().max(other.pos), self.max().min(other.max()))
    }
    pub fn intersect(self, other : Self) -> Option<Self>
    {
        let intersect = self.intersect_or_empty(other);
        if intersect.size.is_zero()
        {
            Some(intersect)
        }
        else
        {
            None
        }
    }
}

impl<T> RectangleBase<T> 
{
    /// assume the size is valid
    pub const fn new(pos: T, size: T) -> Self { Self { pos, size } }
    /// assume the size is valid
    pub const fn new_sized(size : T) -> Self where T : Zero { Self::new(zero(), size) }
}

impl<T,const N : usize> Rectangle<T,N> where T : Number
{
    pub fn from_pos_to_pos(start_pos : Vector<T,N>, end_pos : Vector<T,N>) -> Self { Self::new(start_pos, (end_pos - start_pos).map_with(zero(), |a,b| a.max_partial(b))) }

    /// Can access the outside rectangle
    /// 
    /// ```rust
    /// use hexga_math::prelude::*;
    /// 
    /// assert_eq!(rect2p(5,5,10,10).crop_margin_uncheck(2.splat2(), 2.splat2()), rect2p(7,7,6,6));
    /// 
    /// // Can go outside
    /// assert_eq!(rect2p(5,5,10,10).crop_margin_uncheck(-1.splat2(), zero()), rect2p(4,4,11,11));
    /// ```
    pub fn crop_margin_uncheck(&self, margin_start : Vector<T,N>, margin_end : Vector<T,N>) -> Self 
    {
        Self::new(self.pos + margin_start, self.size - margin_start - margin_end)
    }

    /// Can't access the outside rectangle
    /// Return an empty rectangle if the cropped part is too big
    /// 
    /// ```rust
    /// use hexga_math::prelude::*;
    /// 
    /// assert_eq!(rect2p(5,5,10,10).crop_margin(2.splat2(), 2.splat2()), rect2p(7,7,6,6));
    /// 
    /// // Can't go outside
    /// assert_eq!(rect2p(5,5,10,10).crop_margin(-1.splat2(), zero()), rect2p(5,5,10,10));
    /// ```
    pub fn crop_margin(&self, margin_start : Vector<T,N>, margin_end : Vector<T,N>) -> Self 
    {
        self.crop_margin_uncheck(margin_start, margin_end).intersect_or_empty(*self)
    }

    /// Can access the outside rectangle
    pub fn crop_uncheck(&self, subrect_relative : Self) -> Self { subrect_relative.moved_by(self.pos) }

    /// Can't access the outside rectangle
    pub fn crop(&self, subrect_relative : Self) -> Self { self.intersect_or_empty(self.crop_uncheck(subrect_relative)) }
}

impl<T, const N : usize> IRectangle<T, N> for Rectangle<T, N> where T : Number
{
    fn begin(&self) -> Vector<T,N> { self.pos  }
    fn size (&self) -> Vector<T,N> { self.size }
}
 
pub trait IRectangle<T, const N : usize> where T : Number
{
    fn size(&self) -> Vector<T,N>;

    #[inline] fn size_x(&self) -> T where Vector<T,N> : HaveX<T> { self.size().x() }
    #[inline] fn size_y(&self) -> T where Vector<T,N> : HaveY<T> { self.size().y() }
    #[inline] fn size_z(&self) -> T where Vector<T,N> : HaveZ<T> { self.size().z() }
    #[inline] fn size_w(&self) -> T where Vector<T,N> : HaveW<T> { self.size().w() }

    /// Same as `.size_x()`
    fn width (&self) -> T where Vector<T,N> : HaveX<T> { self.size_x() }
    /// Same as `.size_y()`
    fn height(&self) -> T where Vector<T,N> : HaveY<T> { self.size_y() }
    /// Same as `.size_z()`
    fn depth (&self) -> T where Vector<T,N> : HaveZ<T> { self.size_z() }

    fn area(&self) -> T { self.size().area() }

    fn is_inside(&self, pos : Vector<T,N>) -> bool { pos.is_inside(self.size()) }
    fn is_outside(&self, pos : Vector<T,N>) -> bool { !self.is_inside(pos) }

    /// same as `.begin()`
    fn position(&self) -> Vector<T,N> { self.begin() }

    fn begin(&self) -> Vector<T,N>;
    fn end(&self) -> Vector<T,N> { self.begin() + self.size() }

    /// same as `.begin()`
    fn min(&self) -> Vector<T,N> { self.begin() }
    /// same as `.end()`
    fn max(&self) -> Vector<T,N> { self.end() }

    fn rect(&self) -> Rectangle<T, N> { Rectangle::new(self.begin(), self.size()) }

    fn iter_x(&self) -> Range<T> where Vector<T,N> : HaveX<T>, Range<T> : IntoIterator { self.min().x()..self.max().x() }
    fn iter_y(&self) -> Range<T> where Vector<T,N> : HaveY<T>, Range<T> : IntoIterator { self.min().y()..self.max().y() }
    fn iter_z(&self) -> Range<T> where Vector<T,N> : HaveZ<T>, Range<T> : IntoIterator { self.min().z()..self.max().z() }
    fn iter_w(&self) -> Range<T> where Vector<T,N> : HaveW<T>, Range<T> : IntoIterator { self.min().w()..self.max().w() }

    #[inline] fn is_inside_x(&self, x : T) -> bool where Vector<T,N> : HaveX<T> { x >= self.min().x() && x < self.max().x() }
    #[inline] fn is_inside_y(&self, y : T) -> bool where Vector<T,N> : HaveY<T> { y >= self.min().y() && y < self.max().y() }
    #[inline] fn is_inside_z(&self, z : T) -> bool where Vector<T,N> : HaveZ<T> { z >= self.min().z() && z < self.max().z() }
    #[inline] fn is_inside_w(&self, w : T) -> bool where Vector<T,N> : HaveW<T> { w >= self.min().w() && w < self.max().w() }

    #[inline] fn is_outside_x(&self, x : T) -> bool where Vector<T,N> : HaveX<T> { !self.is_inside_x(x) }
    #[inline] fn is_outside_y(&self, y : T) -> bool where Vector<T,N> : HaveY<T> { !self.is_inside_y(y) }
    #[inline] fn is_outside_z(&self, z : T) -> bool where Vector<T,N> : HaveZ<T> { !self.is_inside_z(z) }
    #[inline] fn is_outside_w(&self, w : T) -> bool where Vector<T,N> : HaveW<T> { !self.is_inside_w(w) }
}

/// Iter over all idx inside a rectangle
pub struct RectIter<T, const N : usize> where T : Number
{
    pub offset : Vector<T,N>,
    pub iter : VectorIter<T, N>,
}
impl<T, const N : usize> RectIter<T,N> where T : Number
{
    pub const fn new(offset : Vector<T,N>, iter : VectorIter<T, N>) -> Self { Self { offset, iter } }
}

impl<T, const N : usize> Iterator for RectIter<T,N> where T : Number
{
    type Item = <VectorIter<T, N> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| v + self.offset)
    }
}


impl<T, const N : usize> Position<T, N> for Rectangle<T,N> where T : Copy
{
    fn pos(&self) -> Vector<T,N> {
        self.pos
    }

    fn set_pos(&mut self, pos : Vector<T,N>) -> &mut Self {
        self.pos = pos; self
    }
}

impl<T,const N : usize> Lerpable for Rectangle<T,N> where T : Number, Vector<T,N> : Lerpable
{
    fn lerp_unchecked(self, dest : Self, coef : float) -> Self { Self::new(self.pos.lerp(dest.pos, coef), self.size.lerp(dest.size, coef)) }
}


/*

impl<T,const N : usize> Rectangle<T,N> 
    where Vector<T,N> : Number,
    T : Number ,
    usize : CastTo<T>
{
    pub fn split_axis(&self, axis : Vector<T,N>, nb : usize) -> impl Iterator<Item = Rectangle<T,N>> + '_
    {
        let coef = (Vector::splat(nb.cast_to()) * axis).max(Vector::ONE);
        let size = self.size / coef;
        let offset = size * axis;
        (0..nb).map(move |i| Rectangle::new(self.pos + offset * Vector::splat(i.cast_to()), size))
    }

    
    pub fn split_x(&self, nb : usize) -> impl Iterator<Item = Rectangle<T,N>> + '_ where Vector<T,N> : HaveXAndOne<T> { self.split_axis(Vector::<T,N>::X, nb) }
    pub fn split_y(&self, nb : usize) -> impl Iterator<Item = Rectangle<T,N>> + '_ where Vector<T,N> : HaveYAndOne<T> { self.split_axis(Vector::<T,N>::Y, nb) }
    pub fn split_z(&self, nb : usize) -> impl Iterator<Item = Rectangle<T,N>> + '_ where Vector<T,N> : HaveZAndOne<T> { self.split_axis(Vector::<T,N>::Z, nb) }
    pub fn split_w(&self, nb : usize) -> impl Iterator<Item = Rectangle<T,N>> + '_ where Vector<T,N> : HaveWAndOne<T> { self.split_axis(Vector::<T,N>::W, nb) }

    pub fn split_min(&self, nb : usize) -> impl Iterator<Item = Rectangle<T,N>> + '_ where 
    {
        self.split_axis(Vector::ZERO.with(self.size.min_element_idx(), T::ONE), nb)
    }

    pub fn split_max(&self, nb : usize) -> impl Iterator<Item = Rectangle<T,N>> + '_ where 
    {
        self.split_axis(Vector::ZERO.with(self.size.max_element_idx(), T::ONE), nb)
    }

    pub fn split_on(&self, nb : usize, split_on : SplitOn) -> impl Iterator<Item = Rectangle<T,N>> + '_ where 
    {
        let v = match split_on
        {
            SplitOn::Dim(x) => if x < N { Vector::ZERO.with(x, T::ONE) } else { Vector::ZERO },
            SplitOn::Min => Vector::ZERO.with(self.size.min_element_idx(), T::ONE),
            SplitOn::Max => Vector::ZERO.with(self.size.max_element_idx(), T::ONE),
        };
        self.split_axis(v, nb)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SplitOn
{
    Dim(usize),
    Min,
    Max,
}

/* 
impl<T,const N : usize> Rectangle<T,N> where T : ArrayLike<N> + Copy
{
    fn map_pos(&mut self, )
    fn into_map_pos(mut self, )
}
*/
*/

impl<T,const N : usize> IterIdx<T,N> for Rectangle<T,N> where T : Integer
{
    type IterIdx = RectIter<T,N>;
    fn iter_idx(&self) -> Self::IterIdx { RectIter::new(self.pos, self.size.iter_idx()) }
}


#[cfg(test)]
mod rect_test
{
    use super::*;


    #[test]
    fn crop_margin_uncheck() 
    {
        assert_eq!(rect2p(5,5,10,10).crop_margin_uncheck(2.splat2(), 2.splat2()), rect2p(7,7,6,6));

        assert_eq!(rect2p(5,5,10,10).crop_margin_uncheck(2.splat2(), zero()), rect2p(7,7,8,8));
        assert_eq!(rect2p(5,5,10,10).crop_margin_uncheck(zero(), 2.splat2()), rect2p(5,5,8,8));

        assert_eq!(rect2p(5,5,10,10).crop_margin_uncheck(-1.splat2(), zero()), rect2p(4,4,11,11));
        assert_eq!(rect2p(5,5,10,10).crop_margin_uncheck(zero(), -1.splat2()), rect2p(5,5,11,11));
    }

    
    #[test]
    fn crop_margin() 
    {
        assert_eq!(rect2p(5,5,10,10).crop_margin(2.splat2(), 2.splat2()), rect2p(7,7,6,6));

        assert_eq!(rect2p(5,5,10,10).crop_margin(2.splat2(), zero()), rect2p(7,7,8,8));
        assert_eq!(rect2p(5,5,10,10).crop_margin(zero(), 2.splat2()), rect2p(5,5,8,8));

        assert_eq!(rect2p(5,5,10,10).crop_margin(-1.splat2(), zero()), rect2p(5,5,10,10));
        assert_eq!(rect2p(5,5,10,10).crop_margin(zero(), -1.splat2()), rect2p(5,5,10,10));
    }

    #[test]
    fn crop_normal() 
    {
        let rect = rect2p(0, 0, 10, 10);
        let cropped = rect.crop_margin(point2(1, 1), point2(2, 2));
        assert_eq!(cropped, rect2p(1, 1, 7, 7));
    }

    #[test]
    fn crop_to_much() 
    {
        let rect = rect2p(0, 0, 10, 10);
        let cropped = rect.crop_margin(point2(0, 0), point2(20, 20));
        assert_eq!(cropped, zero());
    }

    #[test]
    fn crop_to_much_2() 
    {
        let rect = rect2p(0, 0, 10, 10);
        let cropped = rect.crop_margin(point2(20, 20), point2(0, 0));
        assert_eq!(cropped, rect2p(20, 20, 0, 0));
    }

    #[test]
    fn crop_to_much_3() 
    {
        let rect = rect2p(0, 0, 10, 10);
        let cropped = rect.crop_margin(point2(20, 20), point2(20, 20));
        assert_eq!(cropped, rect2p(20, 20, 0, 0));
    }
}