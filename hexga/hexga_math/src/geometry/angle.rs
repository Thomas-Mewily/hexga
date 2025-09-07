/// This module provides a generic 2D angle type `AngleOf<T>` that supports units such as degrees, radians, and turns.
///
/// # Features
/// - Conversion between degrees, radians, and turns.
/// - Normalization of angles to standard ranges.
/// - Trigonometric functions (sin, cos, tan, etc.).
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// let a = 90.0f32.degree();
/// println!("Angle in radians: {}", a.radian());
/// println!("Angle in degrees: {}", a.degree());
/// ```
use super::*;


pub mod prelude
{
    pub use super::{Angle,ToAngleComposite};
}

new_unit!(
    /// 2D Angle, support degree, radian, turn...
    ///
    /// Provides conversion to other angle units
    AngleOf
);


/// 2D Angle, support degree, radian, turn...
///
/// Provides conversion to other angle units
///
///
/// Uses `float` as its underlying representation.
///
/// See [`AngleOf`] to use your own precision.
pub type Angle = AngleOf<float>;

pub trait ToAngle<T> : ToAngleComposite<Output = AngleOf<T>> where T: CastIntoFloat {}
impl<S,T> ToAngle<T> for S where S : ToAngleComposite<Output = AngleOf<T>>, T : CastIntoFloat {}

pub trait ToAngleComposite
{
    type Output;
    fn degree(self) -> Self::Output;
    fn radian(self) -> Self::Output;
    fn turn  (self) -> Self::Output;
}

impl<T> ToAngleComposite for T where T: ToFloat<Output = float>
{
    type Output = Angle;
    fn degree(self) -> Angle { Angle::from_degree(self.to_float()) }
    fn radian(self) -> Angle { Angle::from_radian(self.to_float()) }
    fn turn  (self) -> Angle { Angle::from_turn  (self.to_float()) }
}


#[cfg(feature = "hexga_io")]
impl<T> IoLoad for AngleOf<T> where T: Float + for<'de> Deserialize<'de> {}
#[cfg(feature = "hexga_io")]
impl<T> IoSave for AngleOf<T> where T: Float + Serialize {}

#[cfg(feature = "serde")]
impl<T> Serialize for AngleOf<T> where T: Float + Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    { self.degree().serialize(serializer) }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for AngleOf<T> where T: Float + Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[cfg_attr(feature = "serde", derive(Deserialize), serde(untagged))]
        enum AngleInput<T> where T: Float
        {
            Degree (T),
            Prefix { deg: Option<T>, rad: Option<T>, turn: Option<T> },
            // Postfix (String) // To support "90°", "90deg" "3.14rad" ?
        }

        match AngleInput::deserialize(deserializer)
        {
            Ok(v) => match v
            {
                AngleInput::Degree(d) => Ok(Self::from_degree(d)),
                AngleInput::Prefix { deg, rad, turn } =>
                {
                    match deg.is_some() as u8 + rad.is_some() as u8 + turn.is_some() as u8
                    {
                        0 => Err(serde::de::Error::custom("Missing `deg`, `rad` or `turn`")),
                        1 =>
                        {
                            if let Some(d) = deg  { return Ok(Self::from_degree(d)) }
                            if let Some(r) = rad  { return Ok(unsafe { Self::from_inner_value(r) }) }
                            if let Some(t) = turn { return Ok(Self::from_turn(t)) }
                            unreachable!()
                        }
                        _ => Err(serde::de::Error::custom("Only one of `deg`, `rad` or `turn` is expected")),
                    }
                },
            }
            Err(_) => Err(serde::de::Error::custom("Expected an Angle in degree, ex `90`. Available unit: `deg`, `rad` or `turn`. ex: `(deg:45)`.")),
        }
    }
}

impl<T:Float> AngleOf<T>
{
    /// `360°`
    pub const FULL : Self = AngleOf(T::TWO_PI);
    /// `180°`
    pub const HALF : Self = AngleOf(T::PI);
    /// `180°`
    pub const FLAT : Self = AngleOf(T::PI);
    /// `90°`
    pub const RIGHT : Self = AngleOf(T::HALF_PI);

    pub fn from_radian(rad    : T) -> Self { Self(rad) }
    pub fn from_degree(degree : T) -> Self { Self(degree * (T::ANGLE_FULL_RADIAN / T::ANGLE_FULL_DEGREE)) }
    pub fn from_turn  (coef   : T) -> Self { Self(coef * T::ANGLE_FULL_RADIAN) }

    pub fn radian(self) -> T { self.0 }
    pub fn degree(self) -> T { self.0 * (T::ANGLE_FULL_DEGREE / T::ANGLE_FULL_RADIAN) }
    pub fn turn  (self) -> T { self.0  / T::ANGLE_FULL_RADIAN }

    // Todo : check for a better way to do it
    /// `[0, 2PI[`
    pub fn normalized_positive(self) -> Self { Self::from_radian((self.0 % T::ANGLE_FULL_RADIAN + T::ANGLE_FULL_RADIAN) % T::ANGLE_FULL_RADIAN)  }

    // Todo : check for a better way to do it
    /// `]PI; PI]`
    pub fn normalized(self) -> Self
    {
        let tmp = self.normalized_positive();
        if tmp < Self::HALF { tmp } else { tmp - Self::FULL }
    }

    // TODO: make a Trigonometric trait for float and angle ?

    /// `(cos(self), sin(self))`
    #[inline(always)] pub fn cos_sin(self) -> (T, T) { let (s, c) = self.0.sin_cos(); (c, s) }
    /// `(sin(self), cos(self))`
    #[inline(always)] pub fn sin_cos(self) -> (T, T) { self.0.sin_cos() }
    /// `(cos(self), cos(self))`
    #[inline(always)] pub fn cos_cos(self) -> (T, T) { let c = self.cos(); (c, c) }
    /// `(sin(self), sin(self))`
    #[inline(always)] pub fn sin_sin(self) -> (T, T) { let s = self.sin(); (s, s) }

    /// Cosine of the angle.
    #[inline(always)] pub fn cos(self) -> T { self.0.cos() }
    /// Hyperbolic cosine.
    #[inline(always)] pub fn cosh(self) -> T { self.0.cosh() }

    /// Sine of the angle.
    #[inline(always)] pub fn sin(self) -> T { self.0.sin() }
    /// Hyperbolic sine.
    #[inline(always)] pub fn sinh(self) -> T { self.0.sinh() }

    /// Cotangent (`1 / tan(self)`).
    #[inline(always)] pub fn cot(self) -> T { self.0.cot() }

    /// Tangent of the angle.
    #[inline(always)] pub fn tan(self) -> T { self.0.tan() }
    /// Hyperbolic tangent.
    #[inline(always)] pub fn tanh(self) -> T { self.0.tanh() }

    /// Return a normalized (length = 1) vector with the same angle
    #[inline(always)] pub fn to_vec2_normalized(self) -> Vec2 where T: Into<float> { unsafe { Angle::from_inner_value(self.0.into()).to_vector2_normalized() } }
    #[inline(always)] pub fn to_vec2(self, length : T) -> Vec2 where T: Into<float> { unsafe { Angle::from_inner_value(self.0.into()).to_vector2(length.into()) } }

    /// Return a normalized (length = 1) vector with the same angle
    #[inline(always)] pub fn to_vector2_normalized(self) -> Vector2<T> { self.to_vector2(T::ONE) }
    #[inline(always)] pub fn to_vector2(self, length : T) -> Vector2<T> { Vector2::new(self.cos() * length, self.sin()* length) }

    pub fn inside_range(self, begin : Self, end : Self) -> bool
    {
        let self_normalized = self.normalized_positive();
        let begin_normalized = begin.normalized_positive();
        let end_normalized   = end.normalized_positive();

        if begin_normalized.0 <= end_normalized.0
        {
            self_normalized.0 >= begin_normalized.0 && self_normalized.0 <= end_normalized.0
        }
        else
        {
            self_normalized.0 >= begin_normalized.0 || self_normalized.0 <= end_normalized.0
        }
    }
}

impl<T:Float> Debug for AngleOf<T> where T: Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "{:?}°", self.degree()) }
}
impl<T:Float> Display for AngleOf<T> where T: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "{:}°", self.degree()) }
}

impl<T: Float> RangeDefault for AngleOf<T>
{
    const RANGE_MIN  : Self = Self::ZERO;
    const RANGE_HALF : Self = Self::FLAT;
    const RANGE_MAX  : Self = Self::FULL;
    const RANGE      : Self = Self::FULL;
}

/*
impl<T:Float> AngleOf<T>
{
    pub fn fmt_degree_with_optional_precision(self, precision : Option<T>) -> DisplayAngleDegree { DisplayAngleDegree { angle: self, precision }}
    pub fn fmt_degree_with_precision(self, precision : float) -> DisplayAngleDegree { self.fmt_degree_with_optional_precision(Some(precision)) }
    pub fn fmt_degree(self) -> DisplayAngleDegree { self.fmt_degree_with_optional_precision(None) }
}
impl<T:Float> Display for AngleOf<T> where T:Float { fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { self.fmt_degree_with_precision(360.).fmt(f) }}

#[derive(Clone, Copy)]
pub struct DisplayAngleDegree{ angle : AngleOf, precision : Option<T> }
impl<T:Float> Display for DisplayAngleDegree {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        write!(f, "{}°",
        {
            match self.precision
            {
                Some(p) => (self.angle.degree() / p) as i32 as float * p,
                None => self.angle.degree(),
            }
        })
    }
}
*/
