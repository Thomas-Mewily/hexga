use crate::*;

pub trait Abs 
{
    type Output;
    fn abs(self) -> Self::Output;
}
map_on_integer_unsigned!(
    ($primitive_name: ty) => 
    { 
        impl Abs for $primitive_name 
        { 
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self::Output { self }
        } 
    }
);
map_on_integer_signed!(
    ($primitive_name: ty) => 
    { 
        impl Abs for $primitive_name 
        { 
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self::Output { self.abs() }
        } 
    }
);
map_on_float!(
    ($primitive_name: ty) => 
    { 
        impl Abs for $primitive_name 
        { 
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self::Output { self.abs() }
        } 
    }
);
impl<T,T2, const N : usize> Abs for [T;N] where T : Abs<Output = T2> 
{
    type Output=[T2;N];
    fn abs(self) -> Self::Output { self.map(|v| v.abs()) }
}


/// The `+1` operation
pub trait Increase : One + Add<Self, Output=Self> + AddAssign<Self> + Copy + Sized
{
    /// The increment `x++` operator
    #[inline(always)] fn increase(&mut self) { *self += Self::ONE; }
    /// The increment `x++` operator
    #[inline(always)] fn increase_checked(&mut self) -> Result<(), ()> where Self : NumberAttibute + MaxValue + PartialEq
    { if self.can_increase() { self.increase(); Ok(()) } else { Err(()) } }

    /// Do the current value have a successor. 
    /// 
    /// True if not Self::MAX, except for wrapping type, they always have a successor because they wrap
    #[inline(always)] fn can_increase(&self) -> bool where Self : NumberAttibute + MaxValue + PartialEq { Self::OVERFLOW_BEHAVIOR.is_wrapping() || self.is_non_max_value() }

    /// Return the successor `self + 1`
    #[inline(always)] fn successor(self) -> Self { self + Self::ONE }
    /// Return the successor `self + 1`
    #[inline(always)] fn successor_checked(&mut self) -> Result<Self, ()> where Self : NumberAttibute + MaxValue + PartialEq
    { if self.have_successor() { Ok(self.successor()) } else { Err(()) } }

    /// Do the current value have a successor. 
    /// 
    /// True if not Self::MAX, except for wrapping type, they always have a successor because they wrap
    #[inline(always)] fn have_successor(self) -> bool where Self : NumberAttibute + MaxValue + PartialEq { self.can_increase() }
}
impl<T> Increase for T where T : One + Add<T, Output=T> + AddAssign<Self> + Copy {}

/// The `-1` operation
pub trait Decrease : One + Sub<Self, Output=Self> + SubAssign<Self> + Copy + Sized
{
    /// The decrement `x--` operator
    #[inline(always)] fn decrease(&mut self) { *self -= Self::ONE; }
    /// The increment `x--` operator
    #[inline(always)] fn decrease_checked(&mut self) -> Result<(), ()> where Self : NumberAttibute + MinValue + PartialEq
    { if self.can_decrease() { self.decrease(); Ok(()) } else { Err(()) } }

    /// Do the current value have a predecessor. 
    /// 
    /// True if not Self::MIN, except for wrapping type, they always have a predecessor because they wrap
    #[inline(always)] fn can_decrease(&self) -> bool where Self : NumberAttibute + MinValue + PartialEq { Self::OVERFLOW_BEHAVIOR.is_wrapping() || self.is_not_min_value() }

    /// Return the predecessor `self - 1`
    #[inline(always)] fn predecessor(self) -> Self { self - Self::ONE }
    /// Return the predecessor `self - 1`
    #[inline(always)] fn predecessor_checked(&mut self) -> Result<Self, ()> where Self : NumberAttibute + MinValue + PartialEq
    { if self.have_predecessor() { Ok(self.predecessor()) } else { Err(()) } }

    /// Do the current value have a predecessor. 
    /// 
    /// True if not Self::MIN, except for wrapping type, they always have a predecessor because they wrap
    #[inline(always)] fn have_predecessor(self) -> bool where Self : NumberAttibute + MinValue + PartialEq { self.can_decrease() }
}
impl<T> Decrease for T where T : One + Sub<Self, Output=Self> + SubAssign<Self> + Copy + Sized {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum OverflowBehavior { None, Wrapping, Saturating }

impl OverflowBehavior 
{
    #[inline(always)] pub const fn is_none(self) -> bool { matches!(self, Self::None) }
    #[inline(always)] pub const fn is_wrapping(self) -> bool { matches!(self, Self::Wrapping) }
    #[inline(always)] pub const fn is_saturating(self) -> bool { matches!(self, Self::Saturating) }
}

pub trait NumberAttibute
{
    const OVERFLOW_BEHAVIOR : OverflowBehavior = OverflowBehavior::None; 
}

macro_rules! impl_number_attribute 
{
    ($primitive_name: ty) => 
    {
        impl NumberAttibute for $primitive_name {}
    };
}
map_on_integer!(impl_number_attribute);

impl<T> NumberAttibute for Wrapping<T>   { const OVERFLOW_BEHAVIOR : OverflowBehavior = OverflowBehavior::Wrapping;   }
impl<T> NumberAttibute for Saturating<T> { const OVERFLOW_BEHAVIOR : OverflowBehavior = OverflowBehavior::Saturating; }