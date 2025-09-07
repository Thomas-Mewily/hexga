//! # Geometry Rectangle Module
//!
//! This module provides generic, N-dimensional rectangles and related utilities for geometric computations.
//! It supports rectangles in arbitrary dimensions, with methods for construction, manipulation, cropping, intersection, and iteration.
//!
//! ## Features
//!
//! - Generic `Rectangle<T, N>` type for N-dimensional rectangles
//! - Construction from position and size, or from two corner points
//! - Querying rectangle corners, edges, and center points
//! - Cropping and margin operations
//! - Intersection checks
//! - Iteration over rectangle indices (for integer types)
//! - Traits for rectangle-like types (`IRectangle`, `Crop`)
//!
//! ## Usage
//!
//! ```rust
//! use hexga_math::prelude::*;
//!
//! let rect = rect2p(0, 0, 10, 10);
//! assert!(rect.is_inside(point2(5, 5)));
//!
//! let cropped = rect.crop_margin_intersect(point2(2, 2), point2(2, 2));
//! assert_eq!(cropped, rect2p(2, 2, 6, 6));
//! ```
//!
//! ## See Also
//!
//! - [`Vector`] for N-dimensional vector operations
//! - [`IRectangle`] trait for rectangle-like abstractions
//! - [`Crop`] trait for cropping operations
use crate::*;

pub mod prelude;

mod iter;
pub use iter::*;

pub type Rectangle<T, const N : usize> = RectangleOf<Vector<T, N>>;

/// A `N` dimension rectangle
///
#[repr(C)]
pub struct RectangleOf<T>
{
    pub pos  : T,
    pub size : T,
}
impl_fixed_array_with_op!(RectangleOf, 2);

impl<T> Default for RectangleOf<T> where T: Number
{
    fn default() -> Self { Self::SIZED_ONE }
}

impl<T> RectangleOf<T> where T: Number
{
    pub const SIZED_ONE : Self = Self::new_sized(T::ONE);
}

impl<T> From<(T,T)> for RectangleOf<T>
{
    fn from(value: (T,T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> From<RectangleOf<T>> for (T,T)
{
    fn from(value: RectangleOf<T>) -> Self {
        (value.pos, value.size)
    }
}

// 2D :
impl<T> Rectangle<T,2> where T: Number
{
    pub fn down_left (&self) -> Vector<T,2> { self.pos }
    pub fn down_right(&self) -> Vector<T,2> where T: One + Mul<T,Output = T> + Add<T, Output=T> { self.pos + self.size * Vector::<T,2>::X }

    /// Same as down_left()
    pub fn bottom_left (&self) -> Vector<T,2> { self.down_left() }
    /// Same as down_right()
    pub fn bottom_right(&self) -> Vector<T,2> where T: One + Mul<T,Output = T> + Add<T, Output=T> { self.down_right() }

    pub fn up_right  (&self) -> Vector<T,2> where T: Add<T, Output=T> { self.pos + self.size }
    pub fn up_left   (&self) -> Vector<T,2> where T: One, T : Mul<T,Output = T> + Add<T, Output=T> { self.pos + self.size * Vector::<T,2>::Y }

    /// Same as up_right()
    pub fn top_right  (&self) -> Vector<T,2> where T: Add<T, Output=T> { self.up_right() }
    /// Same as up_left()
    pub fn top_left   (&self) -> Vector<T,2> where T: One, T : Mul<T,Output = T> + Add<T, Output=T> { self.up_left() }
}

impl<T,const N : usize> Rectangle<T,N> where Vector<T,N> : Number, T : Number
{
    pub fn new_centered(pos_middle : Vector<T,N>, size : Vector<T,N>) -> Self { Self::new(pos_middle-size/ (Vector::<T,N>::two()), size) }
    pub fn new_with_center(bottom_left : Vector<T,N>, size : Vector<T,N>, center_coef : Vector<T,N>) -> Self { Self::new(bottom_left - center_coef * size, size) }

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

impl<T,const N : usize> SetRectangle<T,N> for Rectangle<T,N> where T: Number
{
    fn set_size(&mut self, size : Vector<T, N>) -> &mut Self { self.size = size; self }
    fn set_size_x(&mut self, x : T) -> &mut Self where Vector<T, N> : HaveX<T> { self.size.set_x(x); self }
    fn set_size_y(&mut self, y : T) -> &mut Self where Vector<T, N> : HaveY<T> { self.size.set_y(y); self }
    fn set_size_z(&mut self, z : T) -> &mut Self where Vector<T, N> : HaveZ<T> { self.size.set_z(z); self }
    fn set_size_w(&mut self, w : T) -> &mut Self where Vector<T, N> : HaveW<T> { self.size.set_w(w); self }

    fn with_size(mut self, size : Vector<T, N>) -> Self { self.size = size; self }
    fn with_size_x(mut self, x : T) -> Self where Vector<T, N> : HaveX<T> { self.size.set_x(x); self }
    fn with_size_y(mut self, y : T) -> Self where Vector<T, N> : HaveY<T> { self.size.set_y(y); self }
    fn with_size_z(mut self, z : T) -> Self where Vector<T, N> : HaveZ<T> { self.size.set_z(z); self }
    fn with_size_w(mut self, w : T) -> Self where Vector<T, N> : HaveW<T> { self.size.set_w(w); self }
}

impl<T,const N : usize> Rectangle<T,N> where T: Number
{
    /// Check if a point inside the rectangle.
    /// Also work for indexing
    ///
    /// ```
    /// use hexga_math::prelude::*;
    ///
    /// // inside :
    /// assert!(rect2p(0, 0, 2, 2).is_inside(point2(1, 1)));
    /// assert!(rect2p(0, 0, 2, 2).is_inside(point2(0, 0)));
    ///
    ///
    /// // not inside :
    /// assert!(!rect2p(0, 0, 2, 2).is_inside(point2(2, 0)));
    /// assert!(!rect2p(0, 0, 2, 2).is_inside(point2(0, 2)));
    /// assert!(!rect2p(0, 0, 2, 2).is_inside(point2(2, 2)));
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
        self.min().all_with(&point, |axis, other_axis| other_axis >= axis) &&
        self.max().all_with(&point, |axis, other_axis| other_axis <  axis)
    }

    /// Check if a point inside the rectangle.
    /// Don't work for indexing
    pub fn is_inside_inclusif(&self, point : Vector<T, N>) -> bool
    {
        self.min().all_with(&point, |axis, other_axis| other_axis >= axis) &&
        self.max().all_with(&point, |axis, other_axis| other_axis <= axis)
    }

    pub fn is_rect_inside(&self, rect : Rectangle<T, N>) -> bool
    {
        self.is_inside(rect.min()) && self.is_inside_inclusif(rect.max())
    }

    pub fn is_empty(&self) -> bool { self.size.is_zero() }

    pub fn clamp_vector(&self, vector : Vector<T, N>) -> Vector<T, N>
    { vector.max(self.min()).min(self.max()) }

    pub fn intersect_or_empty(self, other : Self) -> Self
    {
        Self::from_pos_to_pos(self.min().max(other.pos), self.max().min(other.max()))
    }
    /// None if any rectangle is empty
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

impl<T> RectangleOf<T>
{
    /// assume the size is valid
    pub const fn new(pos: T, size: T) -> Self { Self { pos, size } }
    /// assume the size is valid
    pub const fn new_sized(size : T) -> Self where T: Zero { Self::new(zero(), size) }
}

/// Crop a selection to a sub selection, where the sub selection is contained in the selection.
pub trait Crop<T, const N : usize> : GetRectangle<T, N> + Sized where T: Number
{
    /// Crop the current rectangle to the given sub rectangle.
    ///
    /// The sub rectangle will always be inside the current rectangle.
    fn crop(self, subrect : Rectangle<T, N>) -> Option<Self>;

    /// Crop the current rectangle to the given sub rectangle.
    ///
    /// The sub rectangle will always be inside the current rectangle.
    unsafe fn crop_unchecked(self, subrect : Rectangle<T, N>) -> Self { self.crop(subrect).expect("invalid subrect") }

    /// Crop the current rectangle to the given sub rectangle.
    ///
    /// The sub rectangle will always be inside the current rectangle.
    #[track_caller]
    fn crop_or_panic(self, subrect : Rectangle<T, N>) -> Self { self.crop(subrect).expect("invalid subrect") }


    /// Crop the current rectangle to the given sub rectangle.
    ///
    /// The sub rectangle will always be inside the current rectangle.
    /// If the sub rectangle is outside or partially outside the current view, it will be intersected with the current rectangle.
    fn crop_intersect(self, subrect : Rectangle<T, N>) -> Self
    {
        let rect = self.rect().intersect_or_empty(subrect);
        unsafe { self.crop_unchecked(rect) }
    }

    fn crop_margin(self, margin_start : Vector<T,N>, margin_end : Vector<T,N>) -> Option<Self>
    {
        if margin_start.any(|a| a < &zero()) || margin_end.any(|a| a < &zero()) { return None; }

        let mut subrect = self.rect();

        let removed_size = margin_start + margin_end;
        if removed_size.any_with(subrect.size(), |a,b| a > b) { return None; }

        subrect.pos  += margin_start;
        subrect.size -= removed_size;

        Some(self.crop_or_panic(subrect))
    }

    unsafe fn crop_margin_unchecked(self, margin_start : Vector<T,N>, margin_end : Vector<T,N>) -> Self
    {
        let mut subrect = self.rect();
        subrect.pos  += margin_start;
        subrect.size -= margin_start + margin_end;
        unsafe { self.crop_unchecked(subrect) }
    }

    /// Crop self by adding a margin to the start and the end of the current rectangle size.
    ///
    /// The sub self will always be inside the self.
    fn crop_margin_intersect(self, margin_start : Vector<T,N>, margin_end : Vector<T,N>) -> Self
    {
        let size = self.size() - margin_start - margin_end;
        self.crop_intersect(Rectangle::new(margin_start, size))
    }
}

impl<T,const N : usize> Crop<T,N> for Rectangle<T,N> where T: Number
{
    // The Behavior is not well defined with zero sized subrect
    // Currently it will return None

    fn crop(self, mut subrect : Rectangle<T, N>) -> Option<Self>
    {
        subrect.move_by(self.pos);
        if self.is_rect_inside(subrect) { Some(subrect) } else { None }
    }
    unsafe fn crop_unchecked(self, subrect : Rectangle<T, N>) -> Self { subrect.moved_by(self.pos) }

    fn crop_intersect(self, subrect : Rectangle<T, N>) -> Self { self.intersect_or_empty(subrect.moved_by(self.pos)) }
    unsafe fn crop_margin_unchecked(self, margin_start : Vector<T,N>, margin_end : Vector<T,N>) -> Self { Self::new(self.pos + margin_start, self.size - margin_start - margin_end) }
}

#[cfg(test)]
mod rect_crop_test
{
    use crate::prelude::*;

    #[test]
    fn crop_margin_unchecked()
    {
        unsafe
        {
            assert_eq!(rect2p(5,5,10,10).crop_margin_unchecked(2.splat2(), 2.splat2()), rect2p(7,7,6,6));
            assert_eq!(rect2p(5,5,10,10).crop_margin_unchecked(-1.splat2(), zero()), rect2p(4,4,11,11));
        }
    }

    #[test]
    fn crop_margin_intersect()
    {
        assert_eq!(rect2p(5,5,10,10).crop_margin_intersect(2.splat2(), 2.splat2()), rect2p(7,7,6,6));
        assert_eq!(rect2p(5,5,10,10).crop_margin_intersect(-1.splat2(), zero()), rect2p(5,5,10,10));
    }
}


impl<T,const N : usize> Rectangle<T,N> where T: Number
{
    pub fn from_pos_to_pos(start_pos : Vector<T,N>, end_pos : Vector<T,N>) -> Self { Self::new(start_pos, (end_pos - start_pos).map_with(zero(), |a,b| a.max_partial(b))) }
}

impl<T, const N : usize> GetPosition<T, N> for Rectangle<T, N> where T: Number
{
    #[inline(always)]
    fn pos(&self) -> Vector<T,N> { self.pos }
}
impl<T, const N : usize> SetPosition<T, N> for Rectangle<T, N> where T: Number
{
    #[inline(always)]
    fn set_pos(&mut self, pos : Vector<T,N>) -> &mut Self {
        self.pos = pos;
        self
    }
}
impl<T, const N : usize> GetRectangle<T, N> for Rectangle<T, N> where T: Number
{
    #[inline(always)]
    fn size(&self) -> Vector<T,N> { self.size }
}


pub trait GetRectangle<T, const N : usize> : GetPosition<T,N> where T: Number
{
    fn size(&self) -> Vector<T,N>;

    #[inline(always)] fn size_x(&self) -> T where Vector<T,N> : HaveX<T> { self.size().x() }
    #[inline(always)] fn size_y(&self) -> T where Vector<T,N> : HaveY<T> { self.size().y() }
    #[inline(always)] fn size_z(&self) -> T where Vector<T,N> : HaveZ<T> { self.size().z() }
    #[inline(always)] fn size_w(&self) -> T where Vector<T,N> : HaveW<T> { self.size().w() }

    /// Same as `.size_x()`
    #[inline(always)]
    fn width (&self) -> T where Vector<T,N> : HaveX<T> { self.size_x() }
    /// Same as `.size_y()`
    #[inline(always)]
    fn height(&self) -> T where Vector<T,N> : HaveY<T> { self.size_y() }
    /// Same as `.size_z()`
    #[inline(always)]
    fn depth (&self) -> T where Vector<T,N> : HaveZ<T> { self.size_z() }

    #[inline(always)]
    fn area(&self) -> T { self.size().area() }
    #[inline(always)]
    fn area_usize(&self) -> usize where T: Integer { self.size().area_usize() }

    #[inline(always)]
    fn is_inside(&self, pos : Vector<T,N>) -> bool { pos.is_inside(self.size()) }
    #[inline(always)]
    fn is_outside(&self, pos : Vector<T,N>) -> bool { !self.is_inside(pos) }

    #[inline(always)]
    fn begin(&self) -> Vector<T,N> { self.pos()}
    #[inline(always)]
    fn end(&self) -> Vector<T,N> { self.pos() + self.size() }

    /// same as `.begin()`
    #[inline(always)]
    fn min(&self) -> Vector<T,N> { self.begin() }
    /// same as `.end()`
    #[inline(always)]
    fn max(&self) -> Vector<T,N> { self.end() }

    #[inline(always)]
    fn rect(&self) -> Rectangle<T, N> { Rectangle::new(self.begin(), self.size()) }

    fn iter_x(&self) -> Range<T> where Vector<T,N> : HaveX<T>, Range<T> : IntoIterator { self.min().x()..self.max().x() }
    fn iter_y(&self) -> Range<T> where Vector<T,N> : HaveY<T>, Range<T> : IntoIterator { self.min().y()..self.max().y() }
    fn iter_z(&self) -> Range<T> where Vector<T,N> : HaveZ<T>, Range<T> : IntoIterator { self.min().z()..self.max().z() }
    fn iter_w(&self) -> Range<T> where Vector<T,N> : HaveW<T>, Range<T> : IntoIterator { self.min().w()..self.max().w() }

    #[inline(always)] fn is_inside_x(&self, x : T) -> bool where Vector<T,N> : HaveX<T> { x >= self.min().x() && x < self.max().x() }
    #[inline(always)] fn is_inside_y(&self, y : T) -> bool where Vector<T,N> : HaveY<T> { y >= self.min().y() && y < self.max().y() }
    #[inline(always)] fn is_inside_z(&self, z : T) -> bool where Vector<T,N> : HaveZ<T> { z >= self.min().z() && z < self.max().z() }
    #[inline(always)] fn is_inside_w(&self, w : T) -> bool where Vector<T,N> : HaveW<T> { w >= self.min().w() && w < self.max().w() }

    #[inline(always)] fn is_outside_x(&self, x : T) -> bool where Vector<T,N> : HaveX<T> { !self.is_inside_x(x) }
    #[inline(always)] fn is_outside_y(&self, y : T) -> bool where Vector<T,N> : HaveY<T> { !self.is_inside_y(y) }
    #[inline(always)] fn is_outside_z(&self, z : T) -> bool where Vector<T,N> : HaveZ<T> { !self.is_inside_z(z) }
    #[inline(always)] fn is_outside_w(&self, w : T) -> bool where Vector<T,N> : HaveW<T> { !self.is_inside_w(w) }
}

pub trait SetRectangle<T, const N : usize> : GetRectangle<T,N> + SetPosition<T,N> where T: Number
{
    fn set_size(&mut self, size : Vector<T, N>) -> &mut Self;
    fn set_size_x(&mut self, x : T) -> &mut Self where Vector<T, N> : HaveX<T> { let mut p = self.pos(); p.set_x(x); self.set_pos(p); self }
    fn set_size_y(&mut self, y : T) -> &mut Self where Vector<T, N> : HaveY<T> { let mut p = self.pos(); p.set_y(y); self.set_pos(p); self }
    fn set_size_z(&mut self, z : T) -> &mut Self where Vector<T, N> : HaveZ<T> { let mut p = self.pos(); p.set_z(z); self.set_pos(p); self }
    fn set_size_w(&mut self, w : T) -> &mut Self where Vector<T, N> : HaveW<T> { let mut p = self.pos(); p.set_w(w); self.set_pos(p); self }

    fn with_size(mut self, size : Vector<T, N>) -> Self where Self : Sized { self.set_size(size); self }
    fn with_size_x(mut self, x : T) -> Self where Vector<T, N> : HaveX<T>, Self : Sized { self.set_size_x(x); self }
    fn with_size_y(mut self, y : T) -> Self where Vector<T, N> : HaveY<T>, Self : Sized { self.set_size_y(y); self }
    fn with_size_z(mut self, z : T) -> Self where Vector<T, N> : HaveZ<T>, Self : Sized { self.set_size_z(z); self }
    fn with_size_w(mut self, w : T) -> Self where Vector<T, N> : HaveW<T>, Self : Sized { self.set_size_w(w); self }
}


impl<T,const N : usize> Lerpable for Rectangle<T,N> where T: Number, Vector<T,N> : Lerpable
{
    fn lerp_unchecked(self, dest : Self, coef : float) -> Self { Self::new(self.pos.lerp(dest.pos, coef), self.size.lerp(dest.size, coef)) }
}


/*

impl<T,const N : usize> Rectangle<T,N>
    where Vector<T,N> : Number,
    T : Number ,
    usize : CastInto<T>
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
impl<T,const N : usize> Rectangle<T,N> where T: ArrayLike<N> + Copy
{
    fn map_pos(&mut self, )
    fn into_map_pos(mut self, )
}
*/
*/

impl<Idx,const N : usize> IterIndex<Idx,N> for Rectangle<Idx,N> where Idx : Integer
{
    type IterIndex = RectangleIter<Idx,N>;
    fn iter_index(&self) -> Self::IterIndex { RectangleIter::from_vec_iter(self.pos, self.size.iter_index()) }
}


#[cfg(test)]
mod rect_test
{
    use super::*;


    #[test]
    fn crop_margin_unchecked()
    {
        unsafe
        {
            assert_eq!(rect2p(5,5,10,10).crop_margin_unchecked(2.splat2(), 2.splat2()), rect2p(7,7,6,6));

            assert_eq!(rect2p(5,5,10,10).crop_margin_unchecked(2.splat2(), zero()), rect2p(7,7,8,8));
            assert_eq!(rect2p(5,5,10,10).crop_margin_unchecked(zero(), 2.splat2()), rect2p(5,5,8,8));

            assert_eq!(rect2p(5,5,10,10).crop_margin_unchecked(-1.splat2(), zero()), rect2p(4,4,11,11));
            assert_eq!(rect2p(5,5,10,10).crop_margin_unchecked(zero(), -1.splat2()), rect2p(5,5,11,11));
        }
    }


    #[test]
    fn crop_margin()
    {
        assert_eq!(rect2p(5,5,10,10).crop_margin_intersect(2.splat2(), 2.splat2()), rect2p(7,7,6,6));

        assert_eq!(rect2p(5,5,10,10).crop_margin_intersect(2.splat2(), zero()), rect2p(7,7,8,8));
        assert_eq!(rect2p(5,5,10,10).crop_margin_intersect(zero(), 2.splat2()), rect2p(5,5,8,8));

        assert_eq!(rect2p(5,5,10,10).crop_margin_intersect(-1.splat2(), zero()), rect2p(5,5,10,10));
        assert_eq!(rect2p(5,5,10,10).crop_margin_intersect(zero(), -1.splat2()), rect2p(5,5,10,10));
    }

    #[test]
    fn crop_normal()
    {
        let rect = rect2p(0, 0, 10, 10);
        let cropped = rect.crop_margin_intersect(point2(1, 1), point2(2, 2));
        assert_eq!(cropped, rect2p(1, 1, 7, 7));
    }

    #[test]
    fn crop_to_much()
    {
        let rect = rect2p(0, 0, 10, 10);
        let cropped = rect.crop_margin_intersect(point2(0, 0), point2(20, 20));
        assert_eq!(cropped, zero());
    }

    #[test]
    fn crop_to_much_2()
    {
        let rect = rect2p(0, 0, 10, 10);
        let cropped = rect.crop_margin_intersect(point2(20, 20), point2(0, 0));
        assert_eq!(cropped, rect2p(20, 20, 0, 0));
    }

    #[test]
    fn crop_to_much_3()
    {
        let rect = rect2p(0, 0, 10, 10);
        let cropped = rect.crop_margin_intersect(point2(20, 20), point2(20, 20));
        assert_eq!(cropped, rect2p(20, 20, 0, 0));
    }

    #[test]
    fn crop_2()
    {
        let r = rect2p(0, 0, 2, 3);
        assert_eq!(r.crop(rect2p(0, 0, 0, 0)), Some(rect2p(0, 0, 0, 0)));
        assert_eq!(r.crop(r), Some(r));
        assert_eq!(r.crop(rect2p(0, 0, 2, 4)), None);

        for idx in r.iter_index()
        {
            assert_eq!(r.is_inside(idx), true);
        }
    }
}