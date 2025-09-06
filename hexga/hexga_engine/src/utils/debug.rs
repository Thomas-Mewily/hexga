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
    pub use crate::dbg_here;
}