use std::ops::*;

use crate::*;

pub mod prelude;

mod vector_iter;
pub use vector_iter::*;

///
/// A wrapper for an array that applies binary operators component-wise.
///
/// Also supports scalar operation (e.g., `Vector<T,N> op T`).
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(vec2(1.0, 2.0) + Vec2::ONE, vec2(2.0, 3.0));
///
/// assert_eq!(vector3(true, false, true) & Vector3::ZERO, vector3(false, false, false));
/// ```
pub struct Vector<T, const N : usize>
{
    pub array : [T; N]
}
impl<T, const N : usize> Default for Vector<T, N> where [T; N] : Default
{
    fn default() -> Self { Self { array: ___() } }
}
impl<T, const N : usize> Vector<T,N>
{
    pub fn from_fn<F>(f : F) -> Self where F : FnMut(usize) -> T { Self::from_array(std::array::from_fn(f)) }
    pub const fn from_array(array : [T; N]) -> Self { Self { array }}
    pub fn splat(val : T) -> Self where T : Clone { Self::from_fn(|_| val.clone()) }

    pub fn iter(&self) -> impl Iterator<Item = &T> { self.array.iter() }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> { self.array.iter_mut() }
    pub fn into_iter(self) -> impl Iterator<Item = T> { self.array.into_iter() }
}
impl_generic_array_like_with_op!(Vector);

impl<T, const N : usize> Position<T,N> for Vector<T, N> where Self : Copy, T : Copy
{
    fn pos(&self) -> Vector<T,N> {
        *self
    }

    fn set_pos(&mut self, pos : Vector<T,N>) -> &mut Self {
        *self = pos;
        self
    }
}


impl<T,const N : usize> Vector<T,N>
{
    // Vector related

    /// Will always be >= 0.
    ///
    /// If any component is negative, return 0
    pub fn area(self) -> T where T : Number
    {
        self.iter().fold(T::ONE,|a, b| a * (*b).max_partial(T::ZERO))
    }
    /// Will always be >= 0.
    ///
    /// If any component is negative, return 0
    pub fn area_usize(self) -> usize where T : Integer
    {
        if T::PRIMITIVE_NUMBER_TYPE.is_integer_unsigned()
        {
            self.iter().map(|v| v.to_usize()).product()
        }else
        {
            self.iter().fold(usize::ONE,|a, b| a * (*b).max_partial(T::ZERO).to_usize())
        }
    }

    /// Multiply each component
    /// The result can be negative
    pub unsafe fn area_signed(self) -> T where T : Number { self.into_iter().product() }

    /// True is area() > 0
    pub fn have_area(self) -> bool where T : Number { self.area().is_non_zero() }
    /// True is area_usize() > 0
    pub fn have_area_usize(self) -> bool where T : Integer { self.area_usize().is_non_zero() }

    pub fn all_zero(self) -> bool where T : Zero + PartialEq { self.all(|v| v.is_zero()) }
    pub fn all_non_zero(self) -> bool where T : Zero + PartialEq { self.all(|v| v.is_non_zero()) }

    pub fn any_zero(self) -> bool where T : Zero + PartialEq { self.any(|v| v.is_zero()) }
    pub fn any_non_zero(self) -> bool where T : Zero + PartialEq { self.any(|v| v.is_non_zero()) }

    pub fn all_positive_or_zero(self) -> bool where T : PositiveOrNegative { self.all(|v| v.is_positive_or_zero()) }
    pub fn any_positive_or_zero(self) -> bool where T : PositiveOrNegative { self.any(|v| v.is_positive_or_zero()) }

    pub fn all_strictly_positive(self) -> bool where T : PositiveOrNegative { self.all(|v| v.is_strictly_positive()) }
    pub fn any_strictly_negative(self) -> bool where T : PositiveOrNegative { self.any(|v| v.is_strictly_negative()) }

    // Index :
    #[inline(always)]
    pub fn is_inside(self, size : Self) -> bool where T : Number
    {
        self.all_with(&size, |a, m| *a >= T::ZERO && a < m)
    }
    #[inline(always)]
    pub fn is_outside(self, size : Self) -> bool where T : Number { !self.is_inside(size) }

    #[inline(always)]
    pub fn is_inside_rect(self, area : Rectangle<T,N>) -> bool where T : Number { area.is_inside(self) }


    /// `x + y + z ...`
    ///
    /// ```rust
    /// use hexga_math::prelude::*;
    /// assert_eq!(point2(3,4).sum_axis(), 7);
    /// ```
    pub fn sum_axis(self) -> T where T : NumberArithmetic { self.into_iter().sum() }

    /// [Manhattan distance](https://fr.wikipedia.org/wiki/Distance_de_Manhattan)
    pub fn length_manhattan(self) -> T where T : NumberArithmetic, Self : Abs<Output=Self> + Copy { self.abs().sum_axis() }
    pub fn length_squared(self) -> T where T : NumberArithmetic { self.into_iter().map(|v| v * v).sum() }
    pub fn have_length(self) -> bool where T : Zero + PartialEq { self.into_iter().any(|c| c.is_non_zero()) }

    /// The dot product : `x1 * x2 + y1 * y2 + z1 * z2 + ...`
    pub fn dot(self, other : Self) -> T where T : NumberArithmetic
    {
        self.array.into_iter().zip(other.array.into_iter()).fold(T::ZERO, |acc,(a, b)| acc + a * b)
    }

    pub fn vector_to(self, target : Self) -> Self where Self : Sub<Self,Output = Self> { target - self }

    /// Create a rectangle with the given size at position zero.
    pub fn to_rect(self) -> Rectangle<T,N> where Self : Zero
    {
        Rectangle::new_sized(self)
    }
}

/*
pub trait IntegerIndex : Integer + CastInto<isize> + CastFrom<isize> + CastInto<usize> + CastFrom<usize> + Debug {}
impl<T> IntegerIndex for T where T : Integer + CastInto<isize> + CastFrom<isize> + CastInto<usize> + CastFrom<usize> + Debug {}
*/

impl<Idx, const N : usize> Vector<Idx, N>
    where Idx : Integer
{
    #[inline(always)]
    pub fn to_index(self, size : Self) -> Option<usize> { self.is_inside(size).then(|| unsafe { self.to_index_unchecked(size) }) }

    #[inline(always)]
    /// # Safety
    /// This function assuming that :
    /// - The size is valid : (all axis size are >= 1)
    /// - The self is valid (inside the grid : all axis are >= 0 and < current axis size )
    ///
    /// ```rust
    /// use hexga_math::prelude::*;
    /// assert_eq!(unsafe{ point2(0,0).to_index_unchecked(point2(10, 20)) }, 0);
    /// assert_eq!(unsafe{ point2(3,0).to_index_unchecked(point2(10, 20)) }, 3);
    /// assert_eq!(unsafe{ point2(3,1).to_index_unchecked(point2(10, 20)) }, 3+1*10);
    /// assert_eq!(unsafe{ point2(3,5).to_index_unchecked(point2(10, 20)) }, 3+5*10);
    /// ```
    pub unsafe fn to_index_unchecked(self, size : Self) -> usize
    {
        debug_assert!(size.all(|v| *v >= Idx::ZERO));

        let mut index_1d : usize = 0;
        let mut area_cumulative : usize = 1;

        let mut i = 0;
        while i < N
        {
            let current_axis_len : usize = size[i].to_usize();
            let current_value    : usize = self[i].to_usize();

            index_1d        += current_value * area_cumulative;
            area_cumulative *= current_axis_len;
            i += 1;
        }
        index_1d
    }

    #[inline(always)]
    pub fn from_index(index : usize, size : Self) -> Option<Self>
    {
        (index < size.area_usize()).then(|| unsafe { Self::from_index_unchecked(index, size) })
    }

    /// # Safety
    /// This function assuming that :
    /// - The size is valid : (all axis are >= 1)
    /// - The index is valid (inside the grid : all axis are >= 0 and < current axis size )
    ///
    /// ```rust
    /// use hexga_math::prelude::*;
    /// unsafe
    /// {
    ///     let size = point2(10, 20);
    ///     for point in [point2(0,0), point2(3,0), point2(3,1), point2(0,5)]
    ///     {
    ///         let index = point.to_index_unchecked(size);
    ///         let point_back = Point2::from_index_unchecked(index, size);
    ///         assert_eq!(point, point_back);
    ///     }
    /// }
    /// ```
    #[inline(always)]
    pub unsafe fn from_index_unchecked(index : usize, size : Self) -> Self
    {
        debug_assert!(size.all(|v| *v >= Idx::ZERO));

        let mut result = [Idx::ZERO; N];
        let mut area_cumulative = usize::ONE;

        let mut i = 0;
        while i < N
        {
            let current_axis_len = size[i].to_usize();
            result[i] = Idx::cast_from((index / area_cumulative) % current_axis_len);
            area_cumulative *= current_axis_len;
            i += 1;
        }
        Self::from_array(result)
    }

    pub fn to_grid<F,U>(self, f : F) -> GridBase<U,Idx,N> where F : FnMut(Self) -> U { GridBase::from_fn(self, f) }
}



impl<T, const N : usize> Vector<T,N> where Self : NumberArithmetic, T : Float
{
    pub fn distance_to(self, target : Self) -> T { self.vector_to(target).length() }

    pub fn length(self) -> T { self.length_squared().sqrt() }

    pub fn normalize(&mut self) -> &mut Self { *self = self.normalized(); self }
    pub fn normalized(self) -> Self
    {
        if self.have_length()
        {
            self / Self::splat(self.length())
        }
        else
        {
            Self::ZERO
        }
    }

    /// set the angle of the `(x, y)` axis
    pub fn set_angle(&mut self, angle : AngleOf<T>) -> &mut Self where Self : HaveY<T>
    {
        let a = angle.to_vector2(self.length()); self.set_x(a.x).set_y(a.y);
        self
    }

    /// Using the `(x, y)` axis
    pub fn angle(self) -> AngleOf<T> where Self : HaveY<T> { AngleOf::from_radian(self.y().atan2(self.x())) }

    pub fn set_length(&mut self, length : T) { *self = self.normalized() * Self::splat(length);  }
    pub fn with_length(mut self, length : T) -> Self { self.set_length(length); self }



    pub fn vector_to_limited_by_speed(self, target : Self, speed : T) -> Self where T : PartialOrd
    {
        let dif= self.vector_to(target);
        if dif.is_zero() { return Self::ZERO; };

        let min_dist = speed.min_partial(dif.length());
        dif.normalized().with_length(min_dist)
    }

    /// return a bool to know if the point reached the destination
    pub fn move_to(&mut self, target : Self, speed : T) -> bool where T : PartialOrd, Self : AddAssign<Self>
    {
        if self.distance_to(target) <= speed
        {
            *self = target;
            return true;
        }
        let add = self.vector_to_limited_by_speed(target, speed);
        *self += add;
        add.is_zero()
    }
}