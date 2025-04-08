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

impl<T> Default for RectangleBase<T> where T : Zero + One
{
    fn default() -> Self { Self::SIZED_ONE }
}

impl<T> RectangleBase<T>
{
    pub const fn new(pos: T, size: T) -> Self { Self { pos, size } }
    pub const fn new_sized(size : T) -> Self where T : Zero { Self::new(T::ZERO, size) }

    pub fn from_pos_to_pos(start_pos : T, end_pos : T) -> Self where T : Sub<T, Output = T> + Copy { Self::new(start_pos, end_pos - start_pos) }
}

impl<T> RectangleBase<T> where T : Zero + One
{
    pub const SIZED_ONE : Self = Self::new_sized(T::ONE);
}

// 2D :
impl<T> Rectangle<T,2> where T : UnitArithmetic
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

    /// When Dimension is >= 1
    pub fn width (&self) -> T where Vector<T,N> : HaveX<T> { self.size.x() }
    /// When Dimension is >= 2
    pub fn height(&self) -> T where Vector<T,N> : HaveY<T> { self.size.y() }
    /// When Dimension is >= 3
    pub fn depth (&self) -> T where Vector<T,N> : HaveZ<T> { self.size.z() }
}

impl<T,const N : usize> Rectangle<T,N> where T : Copy
{
    pub fn size(&self) -> Vector<T, N> { self.size }
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

impl<T,const N : usize> Rectangle<T,N> where Vector<T,N> : UnitArithmetic, T : UnitArithmetic
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
    pub fn is_inside(&self, point : Vector<T, N>) -> bool where T : PartialOrd
    {
        self.pos().all_with(&point, |a, c| c >= a) &&
        (self.pos + self.size).all_with(&point, |a, c| c <= a)
    }
    
    pub fn clamp_vector(&self, vector : Vector<T, N>) -> Vector<T, N> where T : PartialOrd
    { vector.max(self.min()).min(self.max()) }
    
    pub fn intersect_or_empty(self, other : Self) -> Self where T : PartialOrd
    {
        Self::from_pos_to_pos(self.clamp_vector(other.min()), self.clamp_vector(other.max()))
    }
    pub fn intersect(self, other : Self) -> Option<Self> where T : PartialOrd
    {
        let intersect = Self::from_pos_to_pos(self.clamp_vector(other.min()), self.clamp_vector(other.max()));
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



impl<T, const N : usize> Rectangle<T,N> where T : Copy
{
    pub fn area(&self) -> T where T : Number { self.size.area() }

    pub fn min(&self) -> Vector<T, N> { self.pos }
    pub fn max(&self) -> Vector<T, N> where Vector<T, N> : Add<Vector<T, N>,Output = Vector<T, N>> { self.pos + self.size }
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

impl<T,const N : usize> Lerpable for Rectangle<T,N> where Vector<T,N> : Lerpable
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