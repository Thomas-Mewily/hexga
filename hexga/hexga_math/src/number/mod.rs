use super::*;

pub use hexga_number::*;

mod extension;
mod floating;
mod integer;

pub mod prelude
{
    pub use hexga_number::prelude::*;
    pub use super::extension::prelude::*;
    pub use super::floating::prelude::*;
    pub use super::integer::prelude::*;
}