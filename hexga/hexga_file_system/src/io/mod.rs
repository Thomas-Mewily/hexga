use super::*;

mod fs_path;
pub use fs_path::*;

mod result;
pub use result::*;

pub mod prelude
{
    pub use super::
    {
        fs_path::*,
        result::*,
    };
}
