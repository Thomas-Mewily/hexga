use crate::*;

pub trait ToAngle
{
    type Output;
    fn degree(self) -> Self::Output;
    fn radian(self) -> Self::Output;
    fn turn  (self) -> Self::Output;
}
impl_composite_output_with_methods!(ToAngle, degree, radian, turn);

impl<T> ToAngle for T where T : CastTo<float>
{
    type Output = Angle;
    fn degree(self) -> Angle { Angle::from_degree(self.cast_to()) }
    fn radian(self) -> Angle { Angle::from_radian(self.cast_to()) }
    fn turn  (self) -> Angle { Angle::from_turn  (self.cast_to()) }
}

pub type Angle = AngleOf<float>;

/// 2D Angle, support degree, radian, turn...
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct AngleOf<T>
{
    _radian : T,
}

#[cfg(feature = "serde")]
impl<T: FloatingNumber + Serialize> Serialize for AngleOf<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    { self.degree().serialize(serializer) }
}

#[cfg(feature = "serde")]
impl<'de, T: FloatingNumber + Deserialize<'de>> Deserialize<'de> for AngleOf<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>,
    {
        let degree = T::deserialize(deserializer)?;
        Ok(AngleOf::from_degree(degree))
    }
}


impl<T:FloatingNumber> Zero for AngleOf<T> { const ZERO : Self = AngleOf { _radian : T::ZERO }; }
impl<T:FloatingNumber> AngleOf<T>
{
    /// `360°`
    pub const FULL : Self = AngleOf { _radian : T::TWO_PI };
    /// `180°`
    pub const HALF : Self = AngleOf { _radian : T::PI };
    /// `180°`
    pub const FLAT : Self = AngleOf { _radian : T::PI };
    /// `90°`
    pub const RIGHT : Self = AngleOf { _radian : T::HALF_PI };

    pub fn from_radian(rad    : T) -> Self { Self { _radian: rad }}
    pub fn from_degree(degree : T) -> Self { Self { _radian: degree * (T::ANGLE_FULL_RADIAN / T::ANGLE_FULL_DEGREE)  }}
    pub fn from_turn  (coef   : T) -> Self { Self { _radian: coef * T::ANGLE_FULL_RADIAN  }}

    const fn from_internal(rad : T) -> Self { Self { _radian: rad } }

    pub fn radian(self) -> T { self._radian }
    pub fn degree(self) -> T { self._radian * (T::ANGLE_FULL_DEGREE / T::ANGLE_FULL_RADIAN) }
    pub fn turn  (self) -> T { self._radian  / T::ANGLE_FULL_RADIAN }

    // Todo : check for a better way to do it
    /// `[0, 2PI[`
    pub fn normalized_positive(self) -> Self { Self::from_radian((self._radian % T::ANGLE_FULL_RADIAN + T::ANGLE_FULL_RADIAN) % T::ANGLE_FULL_RADIAN)  }
    
    // Todo : check for a better way to do it
    /// `]PI; PI]`
    pub fn normalized(self) -> Self 
    {
        let tmp = self.normalized_positive();
        if tmp < Self::HALF { tmp } else { tmp - Self::FULL }
    }

    #[inline] pub fn cos_sin(self) -> (T, T) { (self.cos(), self.sin()) }
    #[inline] pub fn sin_cos(self) -> (T, T) { (self.sin(), self.cos()) }
    #[inline] pub fn cos_cos(self) -> (T, T) { (self.cos(), self.cos()) }
    #[inline] pub fn sin_sin(self) -> (T, T) { (self.sin(), self.sin()) }

    #[inline] pub fn cos(self) -> T { self._radian.cos() }
    #[inline] pub fn cosh(self) -> T { self._radian.cosh() }

    #[inline] pub fn sin(self) -> T { self._radian.sin() }
    #[inline] pub fn sinh(self) -> T { self._radian.sinh() }

    #[inline] pub fn tan(self) -> T { self._radian.tan() }
    #[inline] pub fn tanh(self) -> T { self._radian.tanh() }

    /// Return a normalized (length = 1) vector with the same angle
    #[inline] pub fn to_vec2_normalized(self) -> Vec2 where T : Into<float> { Angle::from_internal(self._radian.into()).to_vector2_normalized() }
    #[inline] pub fn to_vec2(self, length : T) -> Vec2 where T : Into<float> { Angle::from_internal(self._radian.into()).to_vector2(length.into()) }
    
    /// Return a normalized (length = 1) vector with the same angle
    #[inline] pub fn to_vector2_normalized(self) -> Vector2<T> { self.to_vector2(T::ONE) }
    #[inline] pub fn to_vector2(self, length : T) -> Vector2<T> { Vector2::new(self.cos() * length, self.sin()* length) }

    pub fn inside_range(self, begin : Self, end : Self) -> bool
    {
        let self_normalized = self.normalized_positive();
        let begin_normalized = begin.normalized_positive();
        let end_normalized   = end.normalized_positive();

        if begin_normalized._radian <= end_normalized._radian
        {
            self_normalized._radian >= begin_normalized._radian && self_normalized._radian <= end_normalized._radian
        }
        else
        {
            self_normalized._radian >= begin_normalized._radian || self_normalized._radian <= end_normalized._radian
        }
    }
}

impl<T:FloatingNumber> Debug for AngleOf<T> where T: Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult { write!(f, "{:?}°", self.degree()) }
}
impl<T:FloatingNumber> Display for AngleOf<T> where T: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult { write!(f, "{:}°", self.degree()) }
}

/* 
impl<T:FloatingNumber> AngleOf<T>
{
    pub fn fmt_degree_with_optional_precision(self, precision : Option<T>) -> DisplayAngleDegree { DisplayAngleDegree { angle: self, precision }}
    pub fn fmt_degree_with_precision(self, precision : float) -> DisplayAngleDegree { self.fmt_degree_with_optional_precision(Some(precision)) }
    pub fn fmt_degree(self) -> DisplayAngleDegree { self.fmt_degree_with_optional_precision(None) }
}
impl<T:FloatingNumber> Display for AngleOf<T> where T:FloatingNumber { fn fmt(&self, f: &mut Formatter<'_>) -> DResult { self.fmt_degree_with_precision(360.).fmt(f) }}

#[derive(Clone, Copy)]
pub struct DisplayAngleDegree{ angle : AngleOf, precision : Option<T> }
impl<T:FloatingNumber> Display for DisplayAngleDegree {
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult 
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


impl<T:FloatingNumber> Mul<T> for AngleOf<T> { type Output=AngleOf<T>; fn mul(self, rhs: T) -> Self::Output { Self::from_internal(self._radian * rhs) }}
impl<T:FloatingNumber> Mul<T> for &AngleOf<T> { type Output=AngleOf<T>; fn mul(self, rhs: T) -> Self::Output { *self / rhs }}
impl<T:FloatingNumber> Mul<&T> for AngleOf<T> where T : Copy { type Output=AngleOf<T>; fn mul(self, rhs: &T) -> Self::Output { self / *rhs }}
impl<T:FloatingNumber> Mul<&T> for &AngleOf<T> where T : Copy { type Output=AngleOf<T>; fn mul(self, rhs: &T) -> Self::Output { *self / *rhs }}
impl<T:FloatingNumber> MulAssign<T> for AngleOf<T> { fn mul_assign(&mut self, rhs: T) { self._radian.mul_assign(rhs); }}
impl<T:FloatingNumber> MulAssign<&T> for AngleOf<T> { fn mul_assign(&mut self, rhs: &T) { *self *= *rhs; }}

impl<T:FloatingNumber> Div<T> for AngleOf<T> { type Output=AngleOf<T>; fn div(self, rhs: T) -> Self::Output { Self::from_internal(self._radian/ rhs) }}
impl<T:FloatingNumber> DivAssign<T> for AngleOf<T> { fn div_assign(&mut self, rhs: T) { self._radian.div_assign(rhs); }}

impl<T:FloatingNumber> Neg for AngleOf<T> { type Output=AngleOf<T>; fn neg(self) -> Self::Output { Self::from_internal(self._radian.neg()) }}

impl<T:FloatingNumber> Add<AngleOf<T>> for AngleOf<T> { type Output=AngleOf<T>; fn add(self, rhs: AngleOf<T>) -> Self::Output { Self::from_internal(self._radian.add(rhs._radian)) }}
impl<T:FloatingNumber> AddAssign<AngleOf<T>> for AngleOf<T> { fn add_assign(&mut self, rhs: AngleOf<T>) { self._radian.add_assign(rhs._radian); }}

impl<T:FloatingNumber> Sub<AngleOf<T>> for AngleOf<T> { type Output=AngleOf<T>; fn sub(self, rhs: AngleOf<T>) -> Self::Output { Self::from_internal(self._radian.sub(rhs._radian)) }}
impl<T:FloatingNumber> SubAssign<AngleOf<T>> for AngleOf<T> { fn sub_assign(&mut self, rhs: AngleOf<T>) { self._radian.sub_assign(rhs._radian); }}

impl<T:FloatingNumber> Div<AngleOf<T>> for AngleOf<T> { type Output=AngleOf<T>; fn div(self, rhs: AngleOf<T>) -> Self::Output { Self::from_internal(self._radian.div(rhs._radian)) }}
impl<T:FloatingNumber> DivAssign<AngleOf<T>> for AngleOf<T> { fn div_assign(&mut self, rhs: AngleOf<T>) { self._radian.div_assign(rhs._radian); }}

impl<T:FloatingNumber> Rem<AngleOf<T>> for AngleOf<T> { type Output=AngleOf<T>; fn rem(self, rhs: AngleOf<T>) -> Self::Output { Self::from_internal(self._radian.rem(rhs._radian)) }}
impl<T:FloatingNumber> RemAssign<AngleOf<T>> for AngleOf<T> { fn rem_assign(&mut self, rhs: AngleOf<T>) { self._radian.rem_assign(rhs._radian); }}

impl<T:FloatingNumber> Sum for AngleOf<T>
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<T:FloatingNumber> MinValue for AngleOf<T> where T : MinValue
{
    const MIN : Self = Self::from_internal(T::MIN);
}

impl<T:FloatingNumber> MaxValue for AngleOf<T> where T : MaxValue
{
    const MAX : Self = Self::from_internal(T::MAX);
}