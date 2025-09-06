use crate::*;

pub use hexga_number::*;

// TODO: remove this
mod composite;
pub use composite::*;

mod extension;
pub use extension::*;

mod floating;
pub use floating::*;

mod integer;
pub use integer::*;


pub mod prelude
{
    pub use hexga_number::prelude::*;
    pub use crate::number::composite::*;
    pub use crate::number::extension::*;
    pub use crate::number::floating::*;
    pub use crate::number::integer::*;
}