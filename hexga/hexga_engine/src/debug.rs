pub use log::{info, warn, error, debug};

#[macro_export]
macro_rules! dbg_here {
    () => {
        log::debug!(
            "At {}:{} in {}",
            file!(),
            line!(),
            std::module_path!()
        )
    };
}

pub mod prelude
{
    pub use super::{info,warn,error,debug};
    pub use crate::dbg_here;
}