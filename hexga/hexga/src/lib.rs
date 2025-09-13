pub use hexga_core as core;

pub use hexga_generational as generational;
pub use hexga_math as math;
pub use hexga_bitflags as bitflags;
pub use hexga_ansi_color as ansi_color;
pub use hexga_utils as utils;
pub use hexga_map_on as map_on;
pub use hexga_graphics as graphics;
pub use hexga_random as random;
pub use hexga_singleton as singleton;

#[cfg(feature = "hexga_io")]
pub use hexga_io as io;
//pub use hexga_undo_redo as undo;


pub mod prelude
{
    pub use hexga_core::prelude::*;

    pub use crate::generational::prelude::*;
    pub use crate::math::prelude::*;
    pub use crate::bitflags::bitindex;
    pub use crate::utils::prelude::*;
    pub use crate::graphics::prelude::*;
    pub use crate::random::*;
    pub use crate::singleton::prelude::*;
    //pub use crate::map_on::*;

    #[cfg(feature = "hexga_io")]
    pub use crate::io::prelude::*;

    #[allow(hidden_glob_reexports)]
    pub(crate) mod prelude{}
}