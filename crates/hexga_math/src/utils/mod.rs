use super::*;

pub mod time;
use time::*;

pub mod default;
use default::*;

pub(crate) mod prelude {
    pub use super::traits::*;
    pub use super::{
        default::*,
        time::{Duration, DurationOf, Time, TimeOf},
    };
}

pub(crate) mod traits {
    pub use super::{
        default::*,
        time::{TimeNow, ToTime, ToTimeComposite},
    };
}
