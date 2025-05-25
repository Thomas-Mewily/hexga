use std::fmt::Debug;
use std::ops::*;
use std::hash::Hash;
use crate::*;


/// For every type that support bit based operation (and `&`, or `|`, xor `^`, not `!`, shift `<<` / `>>`...)
pub trait BitArithmetic : 
    Sized + Copy +
    Shl   <Output=Self> + ShlAssign    +
    Shr   <Output=Self> + ShrAssign    +
    BitOr <Output=Self> + BitOrAssign  +
    BitAnd<Output=Self> + BitAndAssign + 
    BitXor<Output=Self> + BitXorAssign +
    Not   <Output = Self>
{
}
impl<T> BitArithmetic for T
    where T :
    Sized + Copy +
    Shl   <Output=Self> + ShlAssign    +
    Shr   <Output=Self> + ShrAssign    +
    BitOr <Output=Self> + BitOrAssign  +
    BitAnd<Output=Self> + BitAndAssign + 
    BitXor<Output=Self> + BitXorAssign +
    Not   <Output = Self>
{}

/// +, -, 0
pub trait UnitArithmetic : 
    Sized + Copy +
    Add<Self,Output = Self> + AddAssign<Self> + Sum +
    Sub<Self,Output = Self> + SubAssign<Self> +
    Zero
{}
impl<T> UnitArithmetic for T where T :
    Sized + Copy +
    Add<Self,Output = Self> + AddAssign<Self> + Sum +
    Sub<Self,Output = Self> + SubAssign<Self> +
    Zero
{}

/// +, -, *, /, %, 0
pub trait NumberArithmetic : 
    UnitArithmetic +
    Mul<Self,Output = Self> + MulAssign<Self> + Product +
    Div<Self,Output = Self> + DivAssign<Self> +
    Rem<Self,Output = Self> + RemAssign<Self> +
{}
impl<T> NumberArithmetic for T where T :
    UnitArithmetic +
    Mul<Self,Output = Self> + MulAssign<Self> + Product +
    Div<Self,Output = Self> + DivAssign<Self> +
    Rem<Self,Output = Self> + RemAssign<Self> +
{}

pub trait ArithmeticNegative             : Neg<Output = Self> {}
impl<T> ArithmeticNegative for T where T : Neg<Output = Self> {}

/// +, -, *, /, %, 0, 1, ==, >=, min val, max val
pub trait Number             : NumberArithmetic + One + PartialEq + PartialOrd + MinValue + MaxValue + Debug {}
impl<T> Number for T where T : NumberArithmetic + One + PartialEq + PartialOrd + MinValue + MaxValue + Debug {}

pub trait NumberNegative             : Number + ArithmeticNegative + MinusOne {}
impl<T> NumberNegative for T where T : Number + ArithmeticNegative + MinusOne {}

/// fX or uX or iX
pub trait NumberPrimitive : Number + PrimitiveNumberType {}
impl<T> NumberPrimitive for T where T : Number + PrimitiveNumberType {}

/// fX or or iX
pub trait NumberPrimitiveNegative : NumberPrimitive + NumberNegative {}
impl<T> NumberPrimitiveNegative for T where T : NumberPrimitive +  NumberNegative {}

/// fX
pub trait NumberFloat             : NumberPrimitiveNegative + Half + NaNValue {}
impl<T> NumberFloat for T where T : NumberPrimitiveNegative + Half + NaNValue {}

/// uX or iX
pub trait NumberInteger             : NumberPrimitive + Eq + Hash + Ord + BitManip + BitArithmetic + Increase + OverflowBehavior {}
impl<T> NumberInteger for T where T : NumberPrimitive + Eq + Hash + Ord + BitManip + BitArithmetic + Increase + OverflowBehavior {}

/// uX
pub trait NumberIntegerUnsigned             : NumberInteger {}
impl<T> NumberIntegerUnsigned for T where T : NumberInteger {}

/// iX
pub trait NumberIntegerSigned             : NumberInteger + NumberPrimitiveNegative {}
impl<T> NumberIntegerSigned for T where T : NumberInteger + NumberPrimitiveNegative {}

// Todo : impl it for vector / array ?
pub trait BitManip
{ 
    fn count_bit_zeros(self) -> u32;
    fn count_bit_ones(self) -> u32;
}
macro_rules! impl_bit { ($primitive_name: ty) => 
    { 
        impl BitManip for $primitive_name 
        { 
            fn count_bit_zeros(self) -> u32 { self.count_zeros() }
            fn count_bit_ones(self) -> u32 { self.count_ones() }
        } 
    }; 
}
map_on_integer!(impl_bit);

impl<T> BitManip for Wrapping<T> where T : BitManip  
{
    fn count_bit_zeros(self) -> u32 { self.0.count_bit_zeros() }
    fn count_bit_ones(self) -> u32 { self.0.count_bit_ones() }
}
impl<T> BitManip for Saturating<T> where T : BitManip  
{
    fn count_bit_zeros(self) -> u32 { self.0.count_bit_zeros() }
    fn count_bit_ones(self) -> u32 { self.0.count_bit_ones() }
}