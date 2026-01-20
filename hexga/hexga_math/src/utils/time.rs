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
    /// Creates a new Time from the specified number of whole milliseconds.
    fn millis(self) -> Self::Output;
    /// Creates a new Time from the specified number of whole seconds.
    fn secs  (self) -> Self::Output;
    /// Creates a new Time from the specified number of whole minutes.
    fn mins  (self) -> Self::Output;
    /// Creates a new Time from the specified number of whole hours.
    fn hours (self) -> Self::Output;
    /// Creates a new Time from the specified number of whole days.
    fn days  (self) -> Self::Output;
}

map_on_number!(
    ($name:ident) =>
    {
        impl ToTimeComposite for $name
        {
            type Output = Time;
            fn millis(self) -> Self::Output { Time::from_millis(self.to_float()) }
            fn secs  (self) -> Self::Output { Time::from_secs  (self.to_float()) }
            fn mins  (self) -> Self::Output { Time::from_mins  (self.to_float()) }
            fn hours (self) -> Self::Output { Time::from_hours (self.to_float()) }
            fn days  (self) -> Self::Output { Time::from_days  (self.to_float()) }
        }
    }
);

impl<T> ToTimeComposite for T where T: Map, T::Item : ToTimeComposite
{
    type Output = T::WithType<<T::Item as ToTimeComposite>::Output>;

    fn millis(self) -> Self::Output { self.map(ToTimeComposite::millis) }
    fn secs  (self) -> Self::Output { self.map(ToTimeComposite::secs  ) }
    fn mins  (self) -> Self::Output { self.map(ToTimeComposite::mins  ) }
    fn hours (self) -> Self::Output { self.map(ToTimeComposite::hours ) }
    fn days  (self) -> Self::Output { self.map(ToTimeComposite::days  ) }
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

        Self::display_non_zero_unit(f, self.timer_days(), "d")?;
        Self::display_non_zero_unit(f, self.timer_hours(), "h")?;
        Self::display_non_zero_unit(f, self.timer_mins(), "min")?;
        Self::display_non_zero_unit(f, self.timer_secs(), "s")?;
        Self::display_non_zero_unit(f, self.timer_millis(), "ms")?;
        Ok(())
    }
}

impl<T:Float> TimeOf<T>
{
    /// milliseconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1000.millis(), 1.secs());
    /// ```
    pub fn from_millis (ms : T) -> Self  { Self::from_secs(ms / 1000.cast_into()) }

    /// milliseconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1f32.secs().millis(), 1000.);
    /// ```
    pub fn millis(self) -> T { self.0 * 1000.cast_into() }


    /// whole milliseconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5f32.millis().whole_millis(), 0);
    /// debug_assert_eq!(1.0f32.millis().whole_millis(), 1);
    /// debug_assert_eq!(1.5f32.millis().whole_millis(), 1);
    /// debug_assert_eq!(1.9f32.millis().whole_millis(), 1);
    /// debug_assert_eq!(2.0f32.millis().whole_millis(), 2);
    ///
    /// debug_assert_eq!(-0.5f32.millis().whole_millis(),  0);
    /// debug_assert_eq!(-2.0f32.millis().whole_millis(), -2);
    /// debug_assert_eq!(-1.9f32.millis().whole_millis(), -1);
    /// ```
    pub fn whole_millis(self) -> i32 { self.millis().round_toward_zero().to_i32() }

    /// Can be used to display milliseconds in a timer
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5f32.millis().timer_millis(), 10);
    ///
    /// debug_assert_eq!(999f32.millis().timer_millis(), 999);
    /// debug_assert_eq!(1000f32.millis().timer_millis(), 0);
    /// debug_assert_eq!(1001f32.millis().timer_millis(), 1);
    /// debug_assert_eq!(2005f32.millis().timer_millis(), 5);
    /// ```
    pub fn timer_millis(self) -> i32 { self.millis().abs().floor().to_i32() % 1000 }

    /// seconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(60.secs(), 1.mins());
    /// ```
    pub const fn from_secs(second : T) -> Self { Self(second) }

    /// total seconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1f32.mins().secs(), 60.);
    /// ```
    pub fn secs(self) -> T { self.0 }

    /// whole seconds
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5f32.secs().whole_secs(), 0);
    /// debug_assert_eq!(1.0f32.secs().whole_secs(), 1);
    /// debug_assert_eq!(1.5f32.secs().whole_secs(), 1);
    /// debug_assert_eq!(1.9f32.secs().whole_secs(), 1);
    /// debug_assert_eq!(2.0f32.secs().whole_secs(), 2);
    /// ```
    pub fn whole_secs(self) -> i32 { self.secs().round_toward_zero().to_i32() }

    /// Can be used to display seconds in a timer
    ///
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5f32.secs().timer_secs(), 10);
    ///
    /// debug_assert_eq!(59f32.secs().timer_secs(), 59);
    /// debug_assert_eq!(60f32.secs().timer_secs(), 0);
    /// debug_assert_eq!(61f32.secs().timer_secs(), 1);
    /// debug_assert_eq!(125f32.secs().timer_secs(), 5);
    /// ```
    pub fn timer_secs(self) -> i32 { self.secs().abs().floor().to_i32() % 60 }

    /// minutes
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(60.mins(), 1.hours());
    /// ```
    pub fn from_mins(min : T) -> Self { Self::from_secs(min * T::SIXTY) }

    /// minutes
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1f32.hours().mins(), 60.);
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
    /// debug_assert_eq!(24f32.hours(), 1.days());
    /// ```
    pub fn from_hours(hours : T) -> Self { Self::from_secs(hours * (T::SIXTY * T::SIXTY)) }
    /// hours
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1f32.days().hours(), 24.);
    /// ```
    pub fn hours(self) -> T { self.0 / (T::SIXTY * T::SIXTY) }


    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5f32.hours().whole_hours(), 0);
    /// debug_assert_eq!(1.0f32.hours().whole_hours(), 1);
    /// debug_assert_eq!(1.5f32.hours().whole_hours(), 1);
    /// debug_assert_eq!(1.9f32.hours().whole_hours(), 1);
    /// debug_assert_eq!(2.0f32.hours().whole_hours(), 2);
    ///
    /// debug_assert_eq!(-0.5f32.hours().whole_hours(),  0);
    /// debug_assert_eq!(-2.0f32.hours().whole_hours(), -2);
    /// debug_assert_eq!(-1.9f32.hours().whole_hours(), -1);
    /// ```
    pub fn whole_hours(self) -> i32 { self.hours().round_toward_zero().to_i32() }


    /// Can be used to display hours in a timer
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5f32.hours().timer_hours(), 10);
    ///
    /// debug_assert_eq!(23f32.hours().timer_hours(), 23);
    /// debug_assert_eq!(24f32.hours().timer_hours(), 0);
    /// debug_assert_eq!(25f32.hours().timer_hours(), 1);
    /// debug_assert_eq!((48f32+5.).hours().timer_hours(), 5);
    /// ```
    pub fn timer_hours(self) -> i32 { self.hours().abs().floor().to_i32() % 24 }

    /// days
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(1f32.days(), (3600*24).secs());
    /// ```
    pub fn from_days(day : T) -> Self { Self::from_secs(day * (T::SIXTY * T::SIXTY * T::TWENTY_FOUR)) }
    /// days
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(24f32.hours().days(), 1.);
    /// ```
    pub fn days(self) -> T { self.0 / (T::SIXTY * T::SIXTY * T::TWENTY_FOUR) }
    /// Whole days
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(0.5f32.days().whole_days(), 0);
    /// debug_assert_eq!(1.0f32.days().whole_days(), 1);
    /// debug_assert_eq!(1.5f32.days().whole_days(), 1);
    /// debug_assert_eq!(1.9f32.days().whole_days(), 1);
    /// debug_assert_eq!(2.0f32.days().whole_days(), 2);
    ///
    /// debug_assert_eq!(-0.5f32.days().whole_days(),  0);
    /// debug_assert_eq!(-2.0f32.days().whole_days(), -2);
    /// debug_assert_eq!(-1.9f32.days().whole_days(), -1);
    /// ```
    pub fn whole_days(self) -> i32 { self.days().round_toward_zero().to_i32() }


    /// Can be used to display days in a timer
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!(10.5f32.days().timer_days(), 10);
    ///
    /// debug_assert_eq!(364f32.days().timer_days(), 364);
    /// debug_assert_eq!(365f32.days().timer_days(), 365);
    /// debug_assert_eq!(366f32.days().timer_days(), 366);
    /// debug_assert_eq!(900f32.days().timer_days(), 900);
    /// ```
    pub fn timer_days(self) -> i32 { self.days().abs().floor().to_i32() }
}


impl<T: Float> RangeDefault for TimeOf<T> where T: RangeDefault
{
    const RANGE_MIN  : Self = Self(T::RANGE_MIN);
    const RANGE_HALF : Self = Self(T::RANGE_HALF);
    const RANGE_MAX  : Self = Self(T::RANGE_MAX);
    const RANGE      : Self = Self(T::RANGE);
}

#[cfg(feature = "serde")]
impl<T> Serialize for TimeOf<T> where T: Float + Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    { self.secs().serialize(serializer) }
}


#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for TimeOf<T> where T: Float + Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        //if deserializer.is_human_readeable
        Ok(Self::from_secs(T::deserialize(deserializer)?))
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
                TimeInput::Second(s) => Ok(Self::from_secs(s)),
                TimeInput::Prefix { ms, s, min, h, d } =>
                {
                    if ms.is_none() && s.is_none() && min.is_none() && h.is_none() && d.is_none()
                    {
                        Err(serde::de::Error::custom("Missing `s`, `ms`, `min`, `h` or `d`"))
                    }else
                    {
                        Ok
                        (
                            Self::from_millis(ms.unwrap_or_zero()) +
                            Self::from_secs(s.unwrap_or_zero()) +
                            Self::from_mins(min.unwrap_or_zero()) +
                            Self::from_hours(h.unwrap_or_zero()) +
                            Self::from_days(d.unwrap_or_zero())
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
            return TimeOf::from_secs(F::cast_from(elapsed.as_secs_f64()));
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
            return TimeOf::from_secs(F::cast_from(elapsed.as_secs_f64()));
        }
    }
}