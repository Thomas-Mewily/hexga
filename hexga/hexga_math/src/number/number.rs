use super::*;


trait_marker!(
/// For every type that support bit based operation (and `&`, or `|`, xor `^`, not `!`, shift `<<` / `>>`...)
BitArithmetic:
        Sized + Copy +
        Shl   <Output = Self> + ShlAssign    +
        Shr   <Output = Self> + ShrAssign    +
        BitOr <Output = Self> + BitOrAssign  +
        BitAnd<Output = Self> + BitAndAssign +
        BitXor<Output = Self> + BitXorAssign +
        Not   <Output = Self>
);


trait_marker!(
/// +, -, 0
Additive:
    Sized + Copy +
    Add<Self,Output = Self> + AddAssign<Self> + Sum +
    Sub<Self,Output = Self> + SubAssign<Self> +
    Zero
    //CfgSerde
);

trait_marker!(
/// +, -, *, /, %, 0
Arithmetic:
    Additive +
    Mul<Self,Output = Self> + MulAssign<Self> + Product +
    Div<Self,Output = Self> + DivAssign<Self> +
    Rem<Self,Output = Self> + RemAssign<Self>
);

trait_marker!(
ArithmeticSigned:
    Arithmetic + Neg<Output = Self>
);


trait_marker!(
    /// +, -, *, /, %, 0, ==, min, max, clamp, mix, abs
    Numeric: Arithmetic + PartialEq + Debug + Min + Max + Clamp + Mix + Abs<Output=Self>
);

trait_marker!(
    NumericSigned: Numeric + ArithmeticSigned
);

trait_marker!(
    /// 1, +, -, *, /, %, 0, ==, min, max, clamp, mix, abs
    NumericIdentity: Numeric + One
);

trait_marker!(
    NumericSignedIdentity: NumericSigned + One
);


trait_marker!(
    /// 1, +, -, *, /, %, 0, ==, >=, min, max, clamp, mix, abs
    Number: NumericIdentity + PartialOrd
);

trait_marker!(
    NumberSigned: Number + NumericSignedIdentity
);

trait_marker!(
    // uX, iX, fX
    Primitive:
        NumericIdentity + PartialOrd + RangeDefault + PrimitiveType + CastPrimitive + CastRangePrimitive + Default
);

trait_marker!(
    // iX, fX
    PrimitiveSigned: Primitive + NumericSignedIdentity
);


trait_marker!(
    // uX, iX
    Integer: Primitive + Eq + Hash + Ord + BitArithmetic + BitManip + Increment + OverflowBehavior + fmt::Binary + fmt::Octal + fmt::LowerHex + fmt::UpperHex
);

trait_marker!(
    // uX
    IntegerUnsigned: Primitive
);

trait_marker!(
    // iX
    IntegerSigned: PrimitiveSigned + Integer
);



pub trait BitManip
{
    fn count_bit_zeros(self) -> u32;
    fn count_bit_ones(self) -> u32;
}

map_on_integer!(
    ($primitive_name: ty) =>
    {
        impl BitManip for $primitive_name
        {
            fn count_bit_zeros(self) -> u32 { self.count_zeros() }
            fn count_bit_ones(self) -> u32 { self.count_ones() }
        }
    };
);
impl<T> BitManip for Wrapping<T> where T: BitManip
{
    fn count_bit_zeros(self) -> u32 { self.0.count_bit_zeros() }
    fn count_bit_ones(self) -> u32 { self.0.count_bit_ones() }
}
impl<T> BitManip for Saturating<T> where T: BitManip
{
    fn count_bit_zeros(self) -> u32 { self.0.count_bit_zeros() }
    fn count_bit_ones(self) -> u32 { self.0.count_bit_ones() }
}
