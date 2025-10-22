use super::*;

/*
pub type DeltaTime = Rel<Time>;
pub type FixedTime = Fix<Time>;
*/

/// Represents time which can be used as an instant or a duration.
///
/// Provides conversion to other time units (seconds, minutes, days...).
///
///
/// Uses `float` as its underlying representation.
///
/// See [`TimeOf`] to use your own precision.
pub type Time = TimeOf<float>;

/// Represents time which can be used as an instant or a duration.
///
/// Provides conversion to other time units (seconds, minutes, days...).
///
///
/// Uses `float` as its underlying representation.
///
/// See [`DeltaTimeOf`] to use your own precision.
pub type DeltaTime = Time;

/// Represents time which can be used as an instant or a duration.
///
/// Provides conversion to other time units (seconds, minutes, days...).
pub type DeltaTimeOf<T> = TimeOf<T>;

new_unit!(
    /// Represents time which can be used as an instant or a duration.
    ///
    /// Provides conversion to other time units (seconds, minutes, days...).
    TimeOf
);

pub trait ToTime<T> : ToTimeComposite<Output = TimeOf<T>> where T: CastIntoFloat {}
impl<S,T> ToTime<T> for S where S : ToTimeComposite<Output = TimeOf<T>>, T : CastIntoFloat {}

pub trait ToTimeComposite
{
    type Output;
    fn ms  (self) -> Self::Output;
    fn s   (self) -> Self::Output;
    /// With an `s` to avoid the confusion with the min function...
    fn mins(self) -> Self::Output;
    fn hour(self) -> Self::Output;
    fn day (self) -> Self::Output;
}

map_on_number!(
    ($name:ident) =>
    {
        impl ToTimeComposite for $name
        {
            type Output = Time;
            fn ms  (self) -> Self::Output { Time::from_ms(self.to_float()) }
            fn s   (self) -> Self::Output { Time::from_s(self.to_float()) }
            /// With an `s` to avoid the confusion with the min function...
            fn mins(self) -> Self::Output { Time::from_mins(self.to_float()) }
            fn hour(self) -> Self::Output { Time::from_hour(self.to_float()) }
            fn day (self) -> Self::Output { Time::from_day(self.to_float()) }
        }
    }
);

impl<T> ToTimeComposite for T where T: Map, T::Item : ToTimeComposite
{
    type Output = T::WithType<<T::Item as ToTimeComposite>::Output>;

    fn ms  (self) -> Self::Output { self.map(ToTimeComposite::ms  ) }
    fn s   (self) -> Self::Output { self.map(ToTimeComposite::s   ) }
    fn mins(self) -> Self::Output { self.map(ToTimeComposite::mins) }
    fn hour(self) -> Self::Output { self.map(ToTimeComposite::hour) }
    fn day (self) -> Self::Output { self.map(ToTimeComposite::day ) }
}

impl<T:Float> Debug for TimeOf<T> { fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "{}", self) } }

impl<T:Float> TimeOf<T>
{
    /// don't display the value if zero
    fn display_non_zero_unit(f: &mut Formatter<'_>, val : i32, unit : &str) -> FmtResult
    { if val != 0 {  Self::display_unit(f, val, unit)?; write!(f, " ") } else { Ok(())} }

    fn display_unit(f: &mut Formatter<'_>, val : i32, unit : &str) -> FmtResult
    { write!(f, "{}{}", val, unit) }
}

impl<T:Float> Display for TimeOf<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        if self.is_zero() { return write!(f, "0s"); }

        if self.is_strictly_negative() { write!(f, "-")?; }

        Self::display_non_zero_unit(f, self.timer_day(), "d")?;
        Self::display_non_zero_unit(f, self.timer_hour(), "h")?;
        Self::display_non_zero_unit(f, self.timer_mins(), "min")?;
        Self::display_non_zero_unit(f, self.timer_s(), "s")?;
        Self::display_non_zero_unit(f, self.timer_ms(), "ms")?;
        Ok(())
    }
}

impl<T:Float> TimeOf<T>
{
    /// milliseconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1000.ms(), 1.s());
    /// ```
    pub fn from_ms (ms : T) -> Self  { Self::from_s(ms / 1000.cast_into()) }

    /// milliseconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1f32.s().ms(), 1000.);
    /// ```
    pub fn ms(self) -> T { self.0 * 1000.cast_into() }


    /// whole milliseconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5f32.ms().whole_ms(), 0);
    /// debug_assert_eq!(1.0f32.ms().whole_ms(), 1);
    /// debug_assert_eq!(1.5f32.ms().whole_ms(), 1);
    /// debug_assert_eq!(1.9f32.ms().whole_ms(), 1);
    /// debug_assert_eq!(2.0f32.ms().whole_ms(), 2);
    ///
    /// debug_assert_eq!(-0.5f32.ms().whole_ms(),  0);
    /// debug_assert_eq!(-2.0f32.ms().whole_ms(), -2);
    /// debug_assert_eq!(-1.9f32.ms().whole_ms(), -1);
    /// ```
    pub fn whole_ms(self) -> i32 { self.ms().round_toward_zero().to_i32() }

    /// Can be used to display milliseconds in a timer
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5f32.ms().timer_ms(), 10);
    ///
    /// debug_assert_eq!(999f32.ms().timer_ms(), 999);
    /// debug_assert_eq!(1000f32.ms().timer_ms(), 0);
    /// debug_assert_eq!(1001f32.ms().timer_ms(), 1);
    /// debug_assert_eq!(2005f32.ms().timer_ms(), 5);
    /// ```
    pub fn timer_ms(self) -> i32 { self.ms().abs().floor().to_i32() % 1000 }

    /// seconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(60.s(), 1.mins());
    /// ```
    pub const fn from_s(second : T) -> Self { Self(second) }

    /// total seconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1f32.mins().s(), 60.);
    /// ```
    pub fn s(self) -> T { self.0 }

    /// whole seconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5f32.s().whole_s(), 0);
    /// debug_assert_eq!(1.0f32.s().whole_s(), 1);
    /// debug_assert_eq!(1.5f32.s().whole_s(), 1);
    /// debug_assert_eq!(1.9f32.s().whole_s(), 1);
    /// debug_assert_eq!(2.0f32.s().whole_s(), 2);
    /// ```
    pub fn whole_s(self) -> i32 { self.s().round_toward_zero().to_i32() }

    /// Can be used to display seconds in a timer
    ///
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5f32.s().timer_s(), 10);
    ///
    /// debug_assert_eq!(59f32.s().timer_s(), 59);
    /// debug_assert_eq!(60f32.s().timer_s(), 0);
    /// debug_assert_eq!(61f32.s().timer_s(), 1);
    /// debug_assert_eq!(125f32.s().timer_s(), 5);
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
    /// debug_assert_eq!(1f32.hour().mins(), 60.);
    /// ```
    pub fn mins(self) -> T { self.0 / T::SIXTY }

    /// whole minutes
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5f32.mins().whole_mins(), 0);
    /// debug_assert_eq!(1.0f32.mins().whole_mins(), 1);
    /// debug_assert_eq!(1.5f32.mins().whole_mins(), 1);
    /// debug_assert_eq!(1.9f32.mins().whole_mins(), 1);
    /// debug_assert_eq!(2.0f32.mins().whole_mins(), 2);
    ///
    /// debug_assert_eq!(-0.5f32.mins().whole_mins(),  0);
    /// debug_assert_eq!(-2.0f32.mins().whole_mins(), -2);
    /// debug_assert_eq!(-1.9f32.mins().whole_mins(), -1);
    /// ```
    pub fn whole_mins(self) -> i32 { self.mins().round_toward_zero().to_i32() }


    /// Can be used to display mins in a timer
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5f32.mins().timer_mins(), 10);
    ///
    /// debug_assert_eq!(59f32.mins().timer_mins(), 59);
    /// debug_assert_eq!(60f32.mins().timer_mins(), 0);
    /// debug_assert_eq!(61f32.mins().timer_mins(), 1);
    /// debug_assert_eq!(125f32.mins().timer_mins(), 5);
    /// ```
    pub fn timer_mins(self) -> i32 { self.mins().abs().floor().to_i32() % 60 }

    /// hours
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(24f32.hour(), 1.day());
    /// ```
    pub fn from_hour(hours : T) -> Self { Self::from_s(hours * (T::SIXTY * T::SIXTY)) }
    /// hours
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1f32.day().hour(), 24.);
    /// ```
    pub fn hour(self) -> T { self.0 / (T::SIXTY * T::SIXTY) }


    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5f32.hour().whole_hour(), 0);
    /// debug_assert_eq!(1.0f32.hour().whole_hour(), 1);
    /// debug_assert_eq!(1.5f32.hour().whole_hour(), 1);
    /// debug_assert_eq!(1.9f32.hour().whole_hour(), 1);
    /// debug_assert_eq!(2.0f32.hour().whole_hour(), 2);
    ///
    /// debug_assert_eq!(-0.5f32.hour().whole_hour(),  0);
    /// debug_assert_eq!(-2.0f32.hour().whole_hour(), -2);
    /// debug_assert_eq!(-1.9f32.hour().whole_hour(), -1);
    /// ```
    pub fn whole_hour(self) -> i32 { self.hour().round_toward_zero().to_i32() }


    /// Can be used to display hours in a timer
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5f32.hour().timer_hour(), 10);
    ///
    /// debug_assert_eq!(23f32.hour().timer_hour(), 23);
    /// debug_assert_eq!(24f32.hour().timer_hour(), 0);
    /// debug_assert_eq!(25f32.hour().timer_hour(), 1);
    /// debug_assert_eq!((48f32+5.).hour().timer_hour(), 5);
    /// ```
    pub fn timer_hour(self) -> i32 { self.hour().abs().floor().to_i32() % 24 }

    /// days
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1f32.day(), (3600*24).s());
    /// ```
    pub fn from_day(day : T) -> Self { Self::from_s(day * (T::SIXTY * T::SIXTY * T::TWENTY_FOUR)) }
    /// days
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(24f32.hour().day(), 1.);
    /// ```
    pub fn day(self) -> T { self.0 / (T::SIXTY * T::SIXTY * T::TWENTY_FOUR) }
    /// Whole days
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5f32.day().whole_day(), 0);
    /// debug_assert_eq!(1.0f32.day().whole_day(), 1);
    /// debug_assert_eq!(1.5f32.day().whole_day(), 1);
    /// debug_assert_eq!(1.9f32.day().whole_day(), 1);
    /// debug_assert_eq!(2.0f32.day().whole_day(), 2);
    ///
    /// debug_assert_eq!(-0.5f32.day().whole_day(),  0);
    /// debug_assert_eq!(-2.0f32.day().whole_day(), -2);
    /// debug_assert_eq!(-1.9f32.day().whole_day(), -1);
    /// ```
    pub fn whole_day(self) -> i32 { self.day().round_toward_zero().to_i32() }


    /// Can be used to display days in a timer
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5f32.day().timer_day(), 10);
    ///
    /// debug_assert_eq!(364f32.day().timer_day(), 364);
    /// debug_assert_eq!(365f32.day().timer_day(), 365);
    /// debug_assert_eq!(366f32.day().timer_day(), 366);
    /// debug_assert_eq!(900f32.day().timer_day(), 900);
    /// ```
    pub fn timer_day(self) -> i32 { self.day().abs().floor().to_i32() }
}


impl<T: Float> RangeDefault for TimeOf<T> where T: RangeDefault
{
    const RANGE_MIN  : Self = Self(T::RANGE_MIN);
    const RANGE_HALF : Self = Self(T::RANGE_HALF);
    const RANGE_MAX  : Self = Self(T::RANGE_MAX);
    const RANGE      : Self = Self(T::RANGE);
}

#[cfg(feature = "hexga_io")]
impl<T> Load for TimeOf<T> where T: Float + for<'de> Deserialize<'de> {}
#[cfg(feature = "hexga_io")]
impl<T> Save for TimeOf<T> where T: Float + Serialize {}


#[cfg(feature = "serde")]
impl<T> Serialize for TimeOf<T> where T: Float + Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    { self.s().serialize(serializer) }
}


#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for TimeOf<T> where T: Float + Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        //if deserializer.is_human_readeable
        Ok(Self::from_s(T::deserialize(deserializer)?))
        // Can't use it because of non descriptive serializer like bincode or postcard

        /*
        #[cfg_attr(feature = "serde", derive(Deserialize), serde(untagged))]
        enum TimeInput<T> where T: Float
        {
            Second (T),
            Prefix { ms : Option<T>, s : Option<T>, min : Option<T>, h: Option<T>, d: Option<T> },
            // Postfix (String) // To support "90s", "90s" "3.14min" ?
        }

        match TimeInput::deserialize(deserializer)
        {
            Ok(v) => match v
            {
                TimeInput::Second(s) => Ok(Self::from_s(s)),
                TimeInput::Prefix { ms, s, min, h, d } =>
                {
                    if ms.is_none() && s.is_none() && min.is_none() && h.is_none() && d.is_none()
                    {
                        Err(serde::de::Error::custom("Missing `s`, `ms`, `min`, `h` or `d`"))
                    }else
                    {
                        Ok
                        (
                            Self::from_ms(ms.unwrap_or_zero()) +
                            Self::from_s(s.unwrap_or_zero()) +
                            Self::from_mins(min.unwrap_or_zero()) +
                            Self::from_hour(h.unwrap_or_zero()) +
                            Self::from_day(d.unwrap_or_zero())
                        )
                    }
                }
            }
            Err(_) =>  Err(serde::de::Error::custom("Expected a Time in second ex: `3`. Available combinable unit: `s`, `ms`, `min`, `h` or `d`. ex: `(min:1,s:30)`.")),
        }
        */
    }
}


pub trait TimeNow : Additive
{
    /// Provides access to a notion of "current time" relative to some starting point.
    ///
    /// The launch point may be program start, the first measurement,
    /// or another fixed reference chosen by the implementation.
    ///
    /// Useful for generic code that needs a monotonically increasing
    /// notion of time without depending on a specific clock.
    fn since_launch() -> Self;
}

impl<F> TimeNow for TimeOf<F> where F:Float + CastFrom<f64>
{
    fn since_launch() -> Self
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::{time::Instant, sync::Once};
            static mut START_TIME: Option<Instant> = None;
            static INIT: Once = Once::new();

            INIT.call_once(|| {
                unsafe {
                    START_TIME = Some(Instant::now());
                }
            });
            let elapsed = unsafe { Instant::now() - START_TIME.unwrap() };
            return TimeOf::from_s(F::cast_from(elapsed.as_secs_f64()));
        }
        #[cfg(target_arch = "wasm32")]
        {
            use web_time::Instant;
            use std::sync::Once;

            static mut START_TIME: Option<Instant> = None;
            static INIT: Once = Once::new();

            INIT.call_once(|| unsafe {
                START_TIME = Some(Instant::now());
            });

            let elapsed = unsafe { Instant::now() - START_TIME.unwrap() };
            return TimeOf::from_s(F::cast_from(elapsed.as_secs_f64()));
        }
    }
}