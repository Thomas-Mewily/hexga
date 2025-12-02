use super::*;


/// Generalized function and constant for floating point like `f32`, `f64`...
///
/// The func impl and documentation are copied from the Rust std because those are the same function, generalized in this trait
pub trait Float : PrimitiveSigned + CastPrimitive + Half + NaNValue
{
    /// 2.
    const TWO : Self;
    /// 3.
    const THREE : Self;
    /// 6.
    const SIX : Self;
    /// 60.
    const SIXTY : Self;
    /// 24.
    const TWENTY_FOUR : Self;


    /// Archimedes' constant (π)
    const PI : Self;

    /// `2.0 * PI`
    const TWO_PI : Self;
    /// `0.5 * PI`
    const HALF_PI : Self;

    const ANGLE_ZERO_RADIAN  : Self = Self::ZERO;
    const ANGLE_FULL_RADIAN  : Self = Self::TWO_PI;
    const ANGLE_HALF_RADIAN  : Self = Self::PI;
    const ANGLE_FLAT_RADIAN  : Self = Self::PI;
    const ANGLE_RIGHT_RADIAN : Self = Self::ANGLE_HALF_RADIAN;

    const ANGLE_ZERO_DEGREE : Self = Self::ZERO;
    const ANGLE_FULL_DEGREE : Self;
    const ANGLE_HALF_DEGREE : Self;
    const ANGLE_FLAT_DEGREE : Self = Self::ANGLE_HALF_DEGREE;
    const ANGLE_RIGHT_DEGREE : Self;

    const ANGLE_ZERO_TURN : Self = Self::ZERO;
    const ANGLE_FULL_TURN : Self;
    const ANGLE_HALF_TURN : Self;
    const ANGLE_FLAT_TURN : Self = Self::ANGLE_HALF_TURN;
    const ANGLE_RIGHT_TURN : Self;



    const COLOR_30_DIV_360  : Self;
    const COLOR_60_DIV_360  : Self;
    const COLOR_90_DIV_360  : Self;
    const COLOR_120_DIV_360 : Self;
    const COLOR_150_DIV_360 : Self;
    const COLOR_180_DIV_360 : Self;
    const COLOR_210_DIV_360 : Self;
    const COLOR_240_DIV_360 : Self;
    const COLOR_270_DIV_360 : Self;
    const COLOR_300_DIV_360 : Self;
    const COLOR_330_DIV_360 : Self;


    /// Returns the square root of a number.
    ///
    /// Returns NaN if `self` is a negative number other than `-0.0`.
    ///
    /// # Precision
    ///
    /// The result of this operation is guaranteed to be the rounded
    /// infinite-precision result. It is specified by IEEE 754 as `squareRoot`
    /// and guaranteed not to change.
    ///
    /// # Examples
    ///
    /// ```
    /// let positive = 4.0_f32;
    /// let negative = -4.0_f32;
    /// let negative_zero = -0.0_f32;
    ///
    /// assert_eq!(positive.sqrt(), 2.0);
    /// assert!(negative.sqrt().is_nan());
    /// assert!(negative_zero.sqrt() == negative_zero);
    /// ```
    #[must_use]
    fn sqrt(self) -> Self;

    /// Computes the absolute value of `self`.
    ///
    /// This function always returns the precise result.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 3.5_f32;
    /// let y = -3.5_f32;
    ///
    /// assert_eq!(x.abs(), x);
    /// assert_eq!(y.abs(), -y);
    ///
    /// assert!(f32::NAN.abs().is_nan());
    /// ```
    #[must_use]
    fn fract(self) -> Self;

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - NaN if the number is NaN
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.5_f32;
    ///
    /// assert_eq!(f.signum(), 1.0);
    /// assert_eq!(f32::NEG_INFINITY.signum(), -1.0);
    ///
    /// assert!(f32::NAN.signum().is_nan());
    /// ```
    #[must_use]
    fn signum(self) -> Self;


    /// Returns a number composed of the magnitude of `self` and the sign of
    /// `sign`.
    ///
    /// Equal to `self` if the sign of `self` and `sign` are the same, otherwise equal to `-self`.
    /// If `self` is a NaN, then a NaN with the same payload as `self` and the sign bit of `sign` is
    /// returned.
    ///
    /// If `sign` is a NaN, then this operation will still carry over its sign into the result. Note
    /// that IEEE 754 doesn't assign any meaning to the sign bit in case of a NaN, and as Rust
    /// doesn't guarantee that the bit pattern of NaNs are conserved over arithmetic operations, the
    /// result of `copysign` with `sign` being a NaN might produce an unexpected or non-portable
    /// result. See the [specification of NaN bit patterns](primitive@f32#nan-bit-patterns) for more
    /// info.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.5_f32;
    ///
    /// assert_eq!(f.copysign(0.42), 3.5_f32);
    /// assert_eq!(f.copysign(-0.42), -3.5_f32);
    /// assert_eq!((-f).copysign(0.42), 3.5_f32);
    /// assert_eq!((-f).copysign(-0.42), -3.5_f32);
    ///
    /// assert!(f32::NAN.copysign(1.0).is_nan());
    /// ```
    #[must_use]
    fn copysign(self, sign: Self) -> Self;

    /// Raises a number to a floating point power.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0_f32;
    /// let abs_difference = (x.powf(2.0) - (x * x)).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn pow(self, n: Self) -> Self;

    /// Returns `e^(self)`, (the exponential function).
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// let one = 1.0f32;
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn exp(self) -> Self;

    /// Returns `2^(self)`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 2.0f32;
    ///
    /// // 2^2 - 4 == 0
    /// let abs_difference = (f.exp2() - 4.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn exp2(self) -> Self;

    /// Returns the natural logarithm of the number.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// let one = 1.0f32;
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn ln(self) -> Self;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// The result might not be correctly rounded owing to implementation details;
    /// `self.log2()` can produce more accurate results for base 2, and
    /// `self.log10()` can produce more accurate results for base 10.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5.0f32;
    ///
    /// // log5(5) - 1 == 0
    /// let abs_difference = (five.log(5.0) - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn log(self, base: Self) -> Self;

    /// Returns the base 2 logarithm of the number.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// let two = 2.0f32;
    ///
    /// // log2(2) - 1 == 0
    /// let abs_difference = (two.log2() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn log2(self) -> Self;

    /// Returns the base 10 logarithm of the number.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// let ten = 10.0f32;
    ///
    /// // log10(10) - 1 == 0
    /// let abs_difference = (ten.log10() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn log10(self) -> Self;

    /// Computes the sine of a number (in radians).
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f32::consts::FRAC_PI_2;
    ///
    /// let abs_difference = (x.sin() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn sin(self) -> Self;

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    /// This function currently corresponds to the `asinf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = std::f32::consts::FRAC_PI_2;
    ///
    /// // asin(sin(pi/2))
    /// let abs_difference = (f.sin().asin() - std::f32::consts::FRAC_PI_2).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn asin(self) -> Self;

    /// Hyperbolic sine function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    /// This function currently corresponds to the `sinhf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f32::consts::E;
    /// let x = 1.0f32;
    ///
    /// let f = x.sinh();
    /// // Solving sinh() at 1 gives `(e^2-1)/(2e)`
    /// let g = ((e * e) - 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn sinh(self) -> Self;

    /// Computes the cosine of a number (in radians).
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0 * std::f32::consts::PI;
    ///
    /// let abs_difference = (x.cos() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn cos(self) -> Self;


    /// Computes the cotangent of a number (in radians).
    ///
    /// Defined as `1 / tan(x)` or equivalently `cos(x) / sin(x)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_math::number::*;
    ///
    /// let x = std::f32::consts::FRAC_PI_4; // 45°
    ///
    /// let abs_difference = (x.cot() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn cot(self) -> Self { Self::ONE / self.tan() }

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    /// This function currently corresponds to the `acosf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = std::f32::consts::FRAC_PI_4;
    ///
    /// // acos(cos(pi/4))
    /// let abs_difference = (f.cos().acos() - std::f32::consts::FRAC_PI_4).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn acos(self) -> Self;

    /// Hyperbolic cosine function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    /// This function currently corresponds to the `coshf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f32::consts::E;
    /// let x = 1.0f32;
    /// let f = x.cosh();
    /// // Solving cosh() at 1 gives this result
    /// let g = ((e * e) + 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// // Same result
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn cosh(self) -> Self;


    /// Computes the tangent of a number (in radians).
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    /// This function currently corresponds to the `tanf` from libc on Unix and
    /// Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f32::consts::FRAC_PI_4;
    /// let abs_difference = (x.tan() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn tan(self) -> Self;

    /// Computes the arctangent of a number. Return value is in radians in the
    /// range [-pi/2, pi/2];
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    /// This function currently corresponds to the `atanf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 1.0f32;
    ///
    /// // atan(tan(1))
    /// let abs_difference = (f.tan().atan() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn atan(self) -> Self;

    /// Hyperbolic tangent function.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    /// This function currently corresponds to the `tanhf` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f32::consts::E;
    /// let x = 1.0f32;
    ///
    /// let f = x.tanh();
    /// // Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
    /// let g = (1.0 - e.powi(-2)) / (1.0 + e.powi(-2));
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[must_use]
    fn tanh(self) -> Self;

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
    /// can even differ within the same execution from one invocation to the next.
    /// This function currently corresponds to the `atan2f` from libc on Unix
    /// and Windows. Note that this might change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// // Positive angles measured counter-clockwise
    /// // from positive x axis
    /// // -pi/4 radians (45 deg clockwise)
    /// let x1 = 3.0f32;
    /// let y1 = -3.0f32;
    ///
    /// // 3pi/4 radians (135 deg counter-clockwise)
    /// let x2 = -3.0f32;
    /// let y2 = 3.0f32;
    ///
    /// let abs_difference_1 = (y1.atan2(x1) - (-std::f32::consts::FRAC_PI_4)).abs();
    /// let abs_difference_2 = (y2.atan2(x2) - (3.0 * std::f32::consts::FRAC_PI_4)).abs();
    ///
    /// assert!(abs_difference_1 <= f32::EPSILON);
    /// assert!(abs_difference_2 <= f32::EPSILON);
    /// ```
    #[must_use]
    fn atan2(self, other : Self) -> Self;

    fn sin_cos(self) -> (Self, Self);

    /// between [0., 1.]
    fn normalize(self) -> Self where Self : One { self.min_partial(Self::ONE).max_partial(Self::ZERO) }

    fn floor(self) -> Self;
    fn round(self) -> Self;
    fn ceil (self) -> Self;

    fn trunc(self) -> Self;

    fn total_cmp(&self, other : &Self) -> ::std::cmp::Ordering;

    fn round_toward_zero(self) -> Self where Self : PositiveOrNegative { if self.is_positive_or_zero() { self.floor() } else { self.ceil() }}
    fn round_away_from_zero(self) -> Self where Self : PositiveOrNegative { if self.is_positive_or_zero() { self.ceil() } else { self.floor() }}
}

macro_rules! impl_floating_number {
    ($primitive_name: ident) =>
    {
        impl Float for $primitive_name
        {
            #[inline(always)] fn floor(self) -> Self { Self::floor(self) }
            #[inline(always)] fn ceil (self) -> Self { Self::ceil(self) }
            #[inline(always)] fn round(self) -> Self { Self::round(self) }
            #[inline(always)] fn trunc(self) -> Self { Self::trunc(self) }

            const TWO   : Self = 2.0;
            const THREE : Self = 3.0;
            const SIX   : Self = 6.0;
            const TWENTY_FOUR : Self = 24.;
            const SIXTY : Self = 60.;

            const PI : Self =  std::$primitive_name::consts::PI;
            const TWO_PI : Self  =  Self::PI * 2.;
            const HALF_PI : Self =  Self::PI * 0.5;



            const ANGLE_FULL_DEGREE  : Self = 360.;
            const ANGLE_HALF_DEGREE  : Self = 180.;
            const ANGLE_RIGHT_DEGREE : Self = 90.;
            const ANGLE_FULL_TURN    : Self = Self::ONE;
            const ANGLE_HALF_TURN    : Self = 0.5;
            const ANGLE_RIGHT_TURN   : Self = 0.25;

            const COLOR_30_DIV_360  : Self = 30.  / 360. ;
            const COLOR_60_DIV_360  : Self = 60.  / 360. ;
            const COLOR_90_DIV_360  : Self = 90.  / 360. ;
            const COLOR_120_DIV_360 : Self = 120. / 360. ;
            const COLOR_150_DIV_360 : Self = 150. / 360. ;
            const COLOR_180_DIV_360 : Self = 180. / 360. ;
            const COLOR_210_DIV_360 : Self = 210. / 360. ;
            const COLOR_240_DIV_360 : Self = 240. / 360. ;
            const COLOR_270_DIV_360 : Self = 270. / 360. ;
            const COLOR_300_DIV_360 : Self = 300. / 360. ;
            const COLOR_330_DIV_360 : Self = 330. / 360. ;



            fn sqrt(self) -> Self { self.sqrt() }

            fn cos (self) -> Self { self.cos () }
            fn acos(self) -> Self { self.acos() }
            fn cosh(self) -> Self { self.cosh() }

            fn sin (self) -> Self { self.sin () }
            fn asin(self) -> Self { self.asin() }
            fn sinh(self) -> Self { self.sinh() }

            fn tan (self) -> Self { self.tan () }
            fn atan(self) -> Self { self.atan() }
            fn tanh(self) -> Self { self.tanh() }

            fn atan2(self, other : Self) -> Self { self.atan2(other) }

            fn sin_cos(self) -> (Self, Self) { self.sin_cos() }


            fn fract   (self) -> Self { self.fract() }
            fn signum  (self) -> Self {self.signum() }
            fn copysign(self, sign: Self) -> Self { self.copysign(sign) }

            fn pow(self, n: Self) -> Self { self.powf(n) }

            fn exp(self) -> Self { self.exp() }
            fn exp2(self) -> Self { self.exp2() }

            fn ln(self) -> Self { self.ln() }
            fn log(self, base: Self) -> Self { self.log(base) }

            fn log2(self) -> Self { self.log2() }
            fn log10(self) -> Self { self.log10() }

            fn total_cmp(&self, other : &Self) -> ::std::cmp::Ordering { self.total_cmp(other) }
        }
    };
}
map_on_float!(impl_floating_number);
