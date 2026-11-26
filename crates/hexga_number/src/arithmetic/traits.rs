use super::*;

/// `Shl<Rhs,Output=Output> + Shr<Rhs,Output=Output>`
pub trait Sh<Rhs = Self, Output = Self>: Shl<Rhs, Output = Output> + Shr<Rhs, Output = Output> {}
impl<S, Rhs, Output> Sh<Rhs, Output> for S where S: Shl<Rhs, Output = Output> + Shr<Rhs, Output = Output> {}

/// `ShlAssign<Rhs> + ShrAssign<Rhs>`
pub trait ShAssign<Rhs = Self>: ShlAssign<Rhs> + ShrAssign<Rhs> {}
impl<S, Rhs> ShAssign<Rhs> for S where S: ShlAssign<Rhs> + ShrAssign<Rhs> {}

trait_marker!(
    /// Shift by every primitive integer width and `self`.
    ShAll: Sized + Sh<u8> + Sh<u16> + Sh<u32> + Sh<u64> + Sh<u128> + Sh<usize> + Sh<Self>
);

trait_marker!(
    /// Shift assign by every primitive integer width and `self`.
    ShAssignAll: Sized + ShAssign<u8> + ShAssign<u16> +  Sh<u32> + ShAssign<u64> + ShAssign<u128> + ShAssign<usize> + ShAssign<Self>
);

trait_marker!(
    /// Shift and shift assign by every primitive integer width and `self`.
    BitShift : ShAll + ShAssignAll
);

trait_marker!(
/// For every type that support bit based operation (and `&`, or `|`, xor `^`, not `!`, shift `<<` / `>>`...)
BitArithmetic:
        Sized + Copy +

        BitShift +

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

// TODO: harcoded float type
trait_marker!(
    /// +, -, *, /, %, 0, ==, min, max, clamp, mix, abs
    Numeric: Arithmetic + PartialEq + Debug + Min + Max + Clamp + Lerp<f32> + Lerp<f64> + Abs<Output=Self>
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
        NumericIdentity + PartialOrd + RangeDefault + PrimitiveType + CastPrimitive + CastRangePrimitive + Default + RemEuclid + Pow + Unit<Precision = Self>
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

pub trait BitManip: Sized /*
+ Shl<Self, Output = Self>
+ ShlAssign<Self>
+ Shr<Self, Output = Self>
+ ShrAssign<Self>
*/
{
    /// Returns the number of zeros in the binary representation of `self`.
    ///
    /// This is heavily dependent on the width of the type.
    ///
    /// # Examples
    /// ```
    /// let zero = 0u8;
    /// assert_eq!(zero.count_zeros(), 8);
    ///
    /// let max = u8::MAX;
    /// assert_eq!(max.count_zeros(), 0);
    /// ```
    fn count_zeros(self) -> u32;

    /// Returns the number of ones in the binary representation of `self`.
    ///
    /// # Examples
    /// ```
    /// let n = 0b01001100u8;
    /// assert_eq!(n.count_ones(), 3);
    ///
    /// let max = u8::MAX;
    /// assert_eq!(max.count_ones(), 8);
    /// ```
    fn count_ones(self) -> u32;

    /// Returns the index of the highest bit set to one in `self`, or `None`
    /// if `self` is 0.
    ///
    /// Note that this is equivalent to `checked_ilog2`.
    ///
    /// # Examples
    /// ```
    /// assert_eq!(0b0_u8.highest_one(), None);
    /// assert_eq!(0b1_u8.highest_one(), Some(0));
    /// assert_eq!(0b1_0000_u8.highest_one(), Some(4));
    /// assert_eq!(0b1_1111_u8.highest_one(), Some(4));
    /// ```
    fn highest_one(self) -> Option<u32>;

    /// Returns the index of the lowest bit set to one in `self`, or `None`
    /// if `self` is 0.
    ///
    /// # Examples
    /// ```
    /// assert_eq!(0b0_u8.lowest_one(), None);
    /// assert_eq!(0b1_u8.lowest_one(), Some(0));
    /// assert_eq!(0b1_0000_u8.lowest_one(), Some(4));
    /// assert_eq!(0b1_1111_u8.lowest_one(), Some(0));
    /// ```
    fn lowest_one(self) -> Option<u32>;

    /// Returns the minimum number of bits required to represent `self`.
    ///
    /// This method returns zero if `self` is zero.
    ///
    /// # Examples
    /// ```
    /// assert_eq!(0_u8.bit_width(), 0);
    /// assert_eq!(0b111_u8.bit_width(), 3);
    /// assert_eq!(0b1110_u8.bit_width(), 4);
    /// assert_eq!(u8::MAX.bit_width(), 8);
    /// ```
    fn bit_width(self) -> u32;

    /// Returns the number of trailing ones in the binary representation of `self`.
    ///
    /// # Examples
    /// ```
    /// let n = 0b1010111u8;
    /// assert_eq!(n.trailing_ones(), 3);
    ///
    /// let zero = 0u8;
    /// assert_eq!(zero.trailing_ones(), 0);
    ///
    /// let max = u8::MAX;
    /// assert_eq!(max.trailing_ones(), 8);
    /// ```
    fn trailing_ones(self) -> u32;

    /// Returns the number of leading ones in the binary representation of `self`.
    ///
    /// # Examples
    /// ```
    /// let n = !(u8::MAX >> 2);
    /// assert_eq!(n.leading_ones(), 2);
    ///
    /// let zero = 0u8;
    /// assert_eq!(zero.leading_ones(), 0);
    ///
    /// let max = u8::MAX;
    /// assert_eq!(max.leading_ones(), 8);
    /// ```
    fn leading_ones(self) -> u32;

    /// Returns the number of trailing zeros in the binary representation of `self`.
    ///
    /// # Examples
    /// ```
    /// let n = 0b0101000u8;
    /// assert_eq!(n.trailing_zeros(), 3);
    ///
    /// let zero = 0u8;
    /// assert_eq!(zero.trailing_zeros(), 8);
    ///
    /// let max = u8::MAX;
    /// assert_eq!(max.trailing_zeros(), 0);
    /// ```
    fn trailing_zeros(self) -> u32;

    /// Returns the number of leading zeros in the binary representation of `self`.
    ///
    /// Depending on what you're doing with the value, you might also be
    /// interested in `ilog2`, which returns a consistent number, even if the
    /// type widens.
    ///
    /// # Examples
    /// ```
    /// let n = u8::MAX >> 2;
    /// assert_eq!(n.leading_zeros(), 2);
    ///
    /// let zero = 0u8;
    /// assert_eq!(zero.leading_zeros(), 8);
    ///
    /// let max = u8::MAX;
    /// assert_eq!(max.leading_zeros(), 0);
    /// ```
    fn leading_zeros(self) -> u32;

    /// Shifts the bits to the left by a specified amount, `n`, wrapping the
    /// truncated bits to the end of the resulting integer.
    ///
    /// `rotate_left(n)` is equivalent to applying `rotate_left(1)` a total of
    /// `n` times. In particular, a rotation by the number of bits in `self`
    /// returns the input value unchanged.
    ///
    /// Please note this isn't the same operation as the `<<` shifting operator!
    ///
    /// # Examples
    /// ```
    /// let n = 0x82u8;
    /// let m = 0x0a;
    ///
    /// assert_eq!(n.rotate_left(2), m);
    /// assert_eq!(n.rotate_left(1024), n);
    /// ```
    fn rotate_left(self, n: u32) -> Self;

    /// Shifts the bits to the right by a specified amount, `n`, wrapping the
    /// truncated bits to the beginning of the resulting integer.
    ///
    /// `rotate_right(n)` is equivalent to applying `rotate_right(1)` a total of
    /// `n` times. In particular, a rotation by the number of bits in `self`
    /// returns the input value unchanged.
    ///
    /// Please note this isn't the same operation as the `>>` shifting operator!
    ///
    /// # Examples
    /// ```
    /// let n = 0x0au8;
    /// let m = 0x82;
    ///
    /// assert_eq!(n.rotate_right(2), m);
    /// assert_eq!(n.rotate_right(1024), n);
    /// ```
    fn rotate_right(self, n: u32) -> Self;
}

map_on_integer!(
    ($primitive_name: ty) =>
    {
        impl BitManip for $primitive_name
        {
            fn count_zeros(self) -> u32 { <$primitive_name>::count_zeros(self) }
            fn count_ones(self) -> u32 { <$primitive_name>::count_ones(self) }

            fn highest_one(self) -> Option<u32> { <$primitive_name>::highest_one(self) }
            fn lowest_one(self) -> Option<u32> { <$primitive_name>::lowest_one(self) }

            fn bit_width(self) -> u32 { <$primitive_name>::bit_width(self) }

            fn trailing_ones(self) -> u32 { <$primitive_name>::trailing_ones(self) }
            fn leading_ones(self) -> u32 { <$primitive_name>::leading_ones(self) }

            fn trailing_zeros(self) -> u32 { <$primitive_name>::trailing_zeros(self) }
            fn leading_zeros(self) -> u32 { <$primitive_name>::leading_zeros(self) }

            fn rotate_left(self, n: u32) -> Self { <$primitive_name>::rotate_left(self, n) }
            fn rotate_right(self, n: u32) -> Self { <$primitive_name>::rotate_right(self, n) }
        }
    };
);
impl<T> BitManip for Wrapping<T>
where
    T: BitManip,
{
    fn count_zeros(self) -> u32 {
        self.0.count_zeros()
    }
    fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    fn highest_one(self) -> Option<u32> {
        self.0.highest_one()
    }
    fn lowest_one(self) -> Option<u32> {
        self.0.lowest_one()
    }

    fn bit_width(self) -> u32 {
        self.0.bit_width()
    }

    fn trailing_ones(self) -> u32 {
        self.0.trailing_ones()
    }
    fn leading_ones(self) -> u32 {
        self.0.leading_ones()
    }

    fn trailing_zeros(self) -> u32 {
        self.0.trailing_zeros()
    }
    fn leading_zeros(self) -> u32 {
        self.0.leading_zeros()
    }

    fn rotate_left(self, n: u32) -> Self {
        Wrapping(self.0.rotate_left(n))
    }
    fn rotate_right(self, n: u32) -> Self {
        Wrapping(self.0.rotate_right(n))
    }
}

impl<T> BitManip for Saturating<T>
where
    T: BitManip,
{
    fn count_zeros(self) -> u32 {
        self.0.count_zeros()
    }
    fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    fn highest_one(self) -> Option<u32> {
        self.0.highest_one()
    }
    fn lowest_one(self) -> Option<u32> {
        self.0.lowest_one()
    }

    fn bit_width(self) -> u32 {
        self.0.bit_width()
    }

    fn trailing_ones(self) -> u32 {
        self.0.trailing_ones()
    }
    fn leading_ones(self) -> u32 {
        self.0.leading_ones()
    }

    fn trailing_zeros(self) -> u32 {
        self.0.trailing_zeros()
    }
    fn leading_zeros(self) -> u32 {
        self.0.leading_zeros()
    }

    fn rotate_left(self, n: u32) -> Self {
        Saturating(self.0.rotate_left(n))
    }
    fn rotate_right(self, n: u32) -> Self {
        Saturating(self.0.rotate_right(n))
    }
}

/*

Todo: trait for op like overflowing_add, wrapping_add, saturaring_add...
*/
