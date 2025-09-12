use super::*;


pub trait TimeNow : UnitArithmetic
{
    // Return the time elapsed since the first measurement
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