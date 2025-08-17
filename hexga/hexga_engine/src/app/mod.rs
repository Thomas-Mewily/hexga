use crate::*;

mod app;
pub use app::*;

mod run;
pub use run::*;

mod event;
pub use event::*;

mod param;
pub use param::*;

pub mod prelude
{
    pub use super::{App,AppRun,IInput,IUserEvent};
}

