use std::ops::*;
use std::hash::Hash;
use crate::*;
/// For every type that support bit based operation (and `&`, or `|`, xor `^`, not `!`, shift `<<` / `>>`...)
pub trait BitArithmetic : 
    Sized + Copy +
    Shl   <Output=Self> + ShlAssign    +
    Shr   <Output=Self> + ShlAssign    +
    BitOr <Output=Self> + BitOrAssign  +
    BitAnd<Output=Self> + BitAndAssign + 
    BitXor<Output=Self> + BitXorAssign +
    Not   <Output = Self>
{}
impl<T> BitArithmetic for T
    where T :
    Sized + Copy +
    Shl   <Output=Self> + ShlAssign    +
    Shr   <Output=Self> + ShlAssign    +
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
pub trait Number             : NumberArithmetic + One + PartialEq + PartialOrd + MinValue + MaxValue {}
impl<T> Number for T where T : NumberArithmetic + One + PartialEq + PartialOrd + MinValue + MaxValue {}

/// fX or iX
pub trait NumberNegative             : Number + ArithmeticNegative + MinusOne {}
impl<T> NumberNegative for T where T : Number + ArithmeticNegative + MinusOne {}

/// fX
pub trait Floating : NumberNegative + Half + NaNValue {}
impl<T> Floating for T where T : NumberNegative + Half + NaNValue {}

/// uX or iX
pub trait Integer             : Number + Eq + Hash + Ord + BitArithmetic + Increase + NumberAttibute {}
impl<T> Integer for T where T : Number + Eq + Hash + Ord + BitArithmetic + Increase + NumberAttibute {}

/// uX
pub trait IntegerUnsigned             : Integer {}
impl<T> IntegerUnsigned for T where T : Integer {}

/// iX
pub trait IntegerSigned             : Integer + NumberNegative {}
impl<T> IntegerSigned for T where T : Integer + NumberNegative {}
