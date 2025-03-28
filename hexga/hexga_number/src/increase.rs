use crate::*;

/// The `+1` operation
pub trait Increase : One + Add<Self, Output=Self> + AddAssign<Self> + Copy + Sized
{
    /// The increment `x++` operator
    #[inline] fn inc (&mut self) { *self += Self::ONE; }
    /// Return the successor `self + 1`
    #[inline] fn succ(self) -> Self { self + Self::ONE }


    /// Do the current value have a successor. 
    /// 
    /// True if not Self::MAX, except for wrapping type, they always have a succesor because they wrap
    #[inline] fn have_succ(self) -> bool where Self : NumberAttibute + MaxValue + PartialEq { Self::OVERFLOW_BEHAVIOR.is_wrapping() || self.is_not_max_value() }
}
impl<T> Increase for T where T : One + Add<T, Output=T> + AddAssign<Self> + Copy {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum OverflowBehavior { None, Wrapping, Saturating }

impl OverflowBehavior 
{
    pub fn is_none(self) -> bool { matches!(self, Self::None) }
    pub fn is_wrapping(self) -> bool { matches!(self, Self::Wrapping) }
    pub fn is_saturating(self) -> bool { matches!(self, Self::Saturating) }
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