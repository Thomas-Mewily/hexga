use super::*;

mod result;
pub use result::*;

mod save;
pub use save::*;

mod load;
pub use load::*;

mod base64;
pub use base64::*;

mod url;
pub use url::*;


pub mod prelude
{
    pub use super::
    {
        result::*,
        save::*,
        load::*,
        url::prelude::*,
    };
}