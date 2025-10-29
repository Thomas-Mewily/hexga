use super::*;

mod result;
pub use result::*;

mod save;
pub use save::*;

mod load;
pub use load::*;

mod base64;
pub use base64::*;

mod url_data;
pub use url_data::*;


pub mod prelude
{
    pub use super::
    {
        result::*,
        save::*,
        load::*,
        url_data::prelude::*,
    };
}