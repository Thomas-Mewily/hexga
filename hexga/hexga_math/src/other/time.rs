use super::*;

/* 
pub type DeltaTime = Rel<Time>;
pub type FixedTime = Fix<Time>;
*/

pub type Time = TimeOf<float>;
pub type DeltaTime = Time;
pub type DeltaTimeOf<T> = TimeOf<T>;

pub trait ToTime
{
    type Output;
    fn ms  (self) -> Self::Output;
    fn s   (self) -> Self::Output;
    /// With an `s` to avoid the confusion with the min function
    fn mins(self) -> Self::Output;
    fn hour(self) -> Self::Output;
    fn day (self) -> Self::Output;
}
impl_composite_output_with_methods!(ToTime, ms, s, mins, hour, day);

impl<T> ToTime for T where T : ToFloat<Output = float>
{
    type Output = Time;

    fn ms  (self) -> Self::Output { Time::from_ms(self.to_float()) }
    fn s   (self) -> Self::Output { Time::from_s(self.to_float()) }
    fn mins(self) -> Self::Output { Time::from_mins(self.to_float()) }
    fn hour(self) -> Self::Output { Time::from_hour(self.to_float()) }
    fn day (self) -> Self::Output { Time::from_day(self.to_float()) }
}

/// Represents time using, which can be used as an instant or a duration.
/// 
/// Provides conversion to other time units (seconds, minutes, days...).
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct TimeOf<T:Float> { second : T }

impl<T:Float> Debug for TimeOf<T> { fn fmt(&self, f: &mut Formatter<'_>) -> DResult { write!(f, "{}", self) } }

impl<T:Float> TimeOf<T>
{
    /// don't display the value if zero
    fn display_non_zero_unit(f: &mut Formatter<'_>, val : i32, unit : &str) -> DResult 
    { if val != 0 {  Self::display_unit(f, val, unit)?; write!(f, " ") } else { Ok(())} }

    fn display_unit(f: &mut Formatter<'_>, val : i32, unit : &str) -> DResult 
    { write!(f, "{}{}", val, unit) }
}

impl<T:Float> Display for TimeOf<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult 
    {
        if self.is_zero() { return write!(f, "0s"); }

        if self.is_strictly_negative() { write!(f, "-")?; }

        Self::display_non_zero_unit(f, self.timer_day(), "d")?;
        Self::display_non_zero_unit(f, self.timer_hour(), "h")?;
        Self::display_non_zero_unit(f, self.timer_mins(), "m")?;
        Self::display_non_zero_unit(f, self.timer_s(), "s")?;
        Self::display_non_zero_unit(f, self.timer_ms(), "ms")?;
        Ok(())
    }
}


impl<T:Float> Zero for TimeOf<T> { const ZERO : Self = Self::from_s(T::ZERO); }

impl<T:Float> Absolute for TimeOf<T> where T : Absolute
{
    fn abs(self) -> Self { TimeOf::from_s(self.second.abs()) }
}

impl<T:Float> TimeOf<T>
{
    const fn from_internal_unit(internal_unit : T) -> Self { Self { second : internal_unit } }

    /// milliseconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1000.ms(), 1.s());
    /// ```
    pub fn from_ms (ms : T) -> Self  { Self::from_s(ms / T::THOUSAND) }

    /// milliseconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1.s().ms(), 1000.);
    /// ```
    pub fn ms(self) -> T { self.second * T::THOUSAND }


    /// whole milliseconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5.ms().whole_ms(), 0);
    /// debug_assert_eq!(1.0.ms().whole_ms(), 1);
    /// debug_assert_eq!(1.5.ms().whole_ms(), 1);
    /// debug_assert_eq!(1.9.ms().whole_ms(), 1);
    /// debug_assert_eq!(2.0.ms().whole_ms(), 2);
    /// 
    /// debug_assert_eq!(-0.5.ms().whole_ms(),  0);
    /// debug_assert_eq!(-2.0.ms().whole_ms(), -2);
    /// debug_assert_eq!(-1.9.ms().whole_ms(), -1);
    /// ```
    pub fn whole_ms(self) -> i32 { self.ms().round_toward_zero().to_i32() }

    /// Can be used to display milliseconds in a timer
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5.ms().timer_ms(), 10);
    /// 
    /// debug_assert_eq!(999.ms().timer_ms(), 999);
    /// debug_assert_eq!(1000.ms().timer_ms(), 0);
    /// debug_assert_eq!(1001.ms().timer_ms(), 1);
    /// debug_assert_eq!(2005.ms().timer_ms(), 5);
    /// ```
    pub fn timer_ms(self) -> i32 { self.ms().abs().floor().to_i32() % 1000 }

    /// seconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(60.s(), 1.mins());
    /// ```
    pub const fn from_s(second : T) -> Self { Self::from_internal_unit(second) }

    /// total seconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1.mins().s(), 60.);
    /// ```
    pub fn s(self) -> T { self.second }

    /// whole seconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5.s().whole_s(), 0);
    /// debug_assert_eq!(1.0.s().whole_s(), 1);
    /// debug_assert_eq!(1.5.s().whole_s(), 1);
    /// debug_assert_eq!(1.9.s().whole_s(), 1);
    /// debug_assert_eq!(2.0.s().whole_s(), 2);
    /// ```
    pub fn whole_s(self) -> i32 { self.s().round_toward_zero().to_i32() }

    /// Can be used to display seconds in a timer
    /// 
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5.s().timer_s(), 10);
    /// 
    /// debug_assert_eq!(59.s().timer_s(), 59);
    /// debug_assert_eq!(60.s().timer_s(), 0);
    /// debug_assert_eq!(61.s().timer_s(), 1);
    /// debug_assert_eq!(125.s().timer_s(), 5);
    /// ```
    pub fn timer_s(self) -> i32 { self.s().abs().floor().to_i32() % 60 }

    /// minutes
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(60.mins(), 1.hour());
    /// ```
    pub fn from_mins(min : T) -> Self { Self::from_s(min * T::SIXTY) }

    /// minutes
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1.hour().mins(), 60.);
    /// ```
    pub fn mins(self) -> T { self.second / T::SIXTY }

    /// whole minutes
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5.mins().whole_mins(), 0);
    /// debug_assert_eq!(1.0.mins().whole_mins(), 1);
    /// debug_assert_eq!(1.5.mins().whole_mins(), 1);
    /// debug_assert_eq!(1.9.mins().whole_mins(), 1);
    /// debug_assert_eq!(2.0.mins().whole_mins(), 2);
    /// 
    /// debug_assert_eq!(-0.5.mins().whole_mins(),  0);
    /// debug_assert_eq!(-2.0.mins().whole_mins(), -2);
    /// debug_assert_eq!(-1.9.mins().whole_mins(), -1);
    /// ```
    pub fn whole_mins(self) -> i32 { self.mins().round_toward_zero().to_i32() }


    /// Can be used to display mins in a timer
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5.mins().timer_mins(), 10);
    /// 
    /// debug_assert_eq!(59.mins().timer_mins(), 59);
    /// debug_assert_eq!(60.mins().timer_mins(), 0);
    /// debug_assert_eq!(61.mins().timer_mins(), 1);
    /// debug_assert_eq!(125.mins().timer_mins(), 5);
    /// ```
    pub fn timer_mins(self) -> i32 { self.mins().abs().floor().to_i32() % 60 }

    /// hours
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(24.hour(), 1.day());
    /// ```
    pub fn from_hour(hours : T) -> Self { Self::from_s(hours * (T::SIXTY * T::SIXTY)) }
    /// hours
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1.day().hour(), 24.);
    /// ```
    pub fn hour(self) -> T { self.second / (T::SIXTY * T::SIXTY) }


    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5.hour().whole_hour(), 0);
    /// debug_assert_eq!(1.0.hour().whole_hour(), 1);
    /// debug_assert_eq!(1.5.hour().whole_hour(), 1);
    /// debug_assert_eq!(1.9.hour().whole_hour(), 1);
    /// debug_assert_eq!(2.0.hour().whole_hour(), 2);
    /// 
    /// debug_assert_eq!(-0.5.hour().whole_hour(),  0);
    /// debug_assert_eq!(-2.0.hour().whole_hour(), -2);
    /// debug_assert_eq!(-1.9.hour().whole_hour(), -1);
    /// ```
    pub fn whole_hour(self) -> i32 { self.hour().round_toward_zero().to_i32() }


    /// Can be used to display hours in a timer
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5.hour().timer_hour(), 10);
    /// 
    /// debug_assert_eq!(23.hour().timer_hour(), 23);
    /// debug_assert_eq!(24.hour().timer_hour(), 0);
    /// debug_assert_eq!(25.hour().timer_hour(), 1);
    /// debug_assert_eq!((48+5).hour().timer_hour(), 5);
    /// ```
    pub fn timer_hour(self) -> i32 { self.hour().abs().floor().to_i32() % 24 }

    /// days
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1.day(), (3600*24).s());
    /// ```
    pub fn from_day(day : T) -> Self { Self::from_s(day * (T::SIXTY * T::SIXTY * T::TWENTY_FOUR)) }
    /// days
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(24.hour().day(), 1.);
    /// ```
    pub fn day(self) -> T { self.second / (T::SIXTY * T::SIXTY * T::TWENTY_FOUR) }
    /// Whole days
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5.day().whole_day(), 0);
    /// debug_assert_eq!(1.0.day().whole_day(), 1);
    /// debug_assert_eq!(1.5.day().whole_day(), 1);
    /// debug_assert_eq!(1.9.day().whole_day(), 1);
    /// debug_assert_eq!(2.0.day().whole_day(), 2);
    /// 
    /// debug_assert_eq!(-0.5.day().whole_day(),  0);
    /// debug_assert_eq!(-2.0.day().whole_day(), -2);
    /// debug_assert_eq!(-1.9.day().whole_day(), -1);
    /// ```
    pub fn whole_day(self) -> i32 { self.day().round_toward_zero().to_i32() }


    /// Can be used to display days in a timer
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5.day().timer_day(), 10);
    /// 
    /// debug_assert_eq!(364.day().timer_day(), 364);
    /// debug_assert_eq!(365.day().timer_day(), 365);
    /// debug_assert_eq!(366.day().timer_day(), 366);
    /// debug_assert_eq!(900.day().timer_day(), 900);
    /// ```
    pub fn timer_day(self) -> i32 { self.day().abs().floor().to_i32() }
}

impl<T:Float> Add<TimeOf<T>> for TimeOf<T> { type Output=Self; fn add(self, rhs: Self) -> Self::Output { Self::from_internal_unit(self.second.add(rhs.second)) }}
impl<T:Float> AddAssign<TimeOf<T>> for TimeOf<T> { fn add_assign(&mut self, rhs: Self) { self.second.add_assign(rhs.second); }}

impl<T:Float> Sub<TimeOf<T>> for TimeOf<T> { type Output=Self; fn sub(self, rhs: Self) -> Self::Output { Self::from_internal_unit(self.second.sub(rhs.second)) }}
impl<T:Float> SubAssign<TimeOf<T>> for TimeOf<T> { fn sub_assign(&mut self, rhs: Self) { self.second.sub_assign(rhs.second); }}

impl<T:Float> Div<TimeOf<T>> for TimeOf<T> { type Output=T; fn div(self, rhs: Self) -> Self::Output { self.second / rhs.second } }
impl<T:Float> DivAssign<T> for TimeOf<T> { fn div_assign(&mut self, rhs: T) { self.second.div_assign(rhs) }}

impl<T:Float> Mul<T> for TimeOf<T> { type Output=Self; fn mul(self, rhs: T) -> Self::Output { Self::from_internal_unit(self.second.mul(rhs)) }}
impl<T:Float> MulAssign<T> for TimeOf<T> { fn mul_assign(&mut self, rhs: T) { self.second.mul_assign(rhs); }}

impl<T:Float> Div<T> for TimeOf<T> { type Output=TimeOf<T>; fn div(self, rhs: T) -> Self::Output { Self::from_internal_unit(self.second.div(rhs)) } }
impl<T:Float> Rem<T> for TimeOf<T> { type Output=TimeOf<T>; fn rem(self, rhs: T) -> Self::Output { Self::from_internal_unit(self.second.rem(rhs)) } }
impl<T:Float> RemAssign<T> for TimeOf<T> { fn rem_assign(&mut self, rhs: T) { self.second.rem_assign(rhs); } }

impl<T:Float> Sum for TimeOf<T>
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<T:Float> MinValue for TimeOf<T> where T : MinValue
{
    const MIN : Self = Self::from_s(T::MIN);
}

impl<T:Float> MaxValue for TimeOf<T> where T : MaxValue
{
    const MAX : Self = Self::from_s(T::MAX);
}




#[cfg(feature = "serde")]
impl<T: Float> Serialize for TimeOf<T> where T : Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    { self.s().serialize(serializer) }
}

#[cfg(feature = "serde")]
impl<'de, T: Float> Deserialize<'de> for TimeOf<T> where T : Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>,
    {
        Ok(Self::from_s(T::deserialize(deserializer)?))
    }
}