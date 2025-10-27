use super::*;

mod extensions;
pub use extensions::*;

mod path_extension;
pub use path_extension::*;

mod magic_file_extension;
pub use magic_file_extension::*;

mod result;
pub use result::*;

pub mod prelude
{
    pub use super::
    {
        path_extension::*,
        magic_file_extension::*,
        result::*,
    };
}
