pub use hexga_core as core;

pub use hexga_generational as generational;
pub use hexga_math as math;
pub use hexga_bitflags as bitflags;
pub use hexga_ansi_color as ansi_color;
pub use hexga_utils as utils;
pub use hexga_map_on as map_on;
pub use hexga_graphics as graphics;
//pub use hexga_random as random;
pub use hexga_singleton as singleton;
pub use hexga_encoding as encoding;

//pub use hexga_undo_redo as undo;

#[cfg(feature = "serde")]
pub use hexga_file_system::*;
#[cfg(feature = "serde")]
pub use hexga_serde::*;

#[cfg(all(test, feature = "serde"))]
mod serde_test;

pub mod prelude
{
    pub use hexga_core::prelude::*;

    pub use crate::generational::prelude::*;
    pub use crate::math::prelude::*;
    pub use crate::bitflags::bitindex;
    pub use crate::utils::prelude::*;
    pub use crate::graphics::prelude::*;
    //pub use crate::random::*;
    pub use crate::singleton::prelude::*;
    pub use crate::encoding::prelude::*;
    //pub use crate::map_on::*;

    /*
    #[allow(unused_imports)]
    #[cfg(feature = "hexga_file_system")]
    pub use hexga_file_system;*/
    // #[allow(unused_imports)]
    // #[cfg(feature = "hexga_file_system")]
    // pub use crate::io::prelude::*;

    /*
    #[allow(unused_imports)]
    #[cfg(feature = "serde")]
    pub use serde;*/
    #[allow(unused_imports)]
    #[cfg(feature = "serde")]
    pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};
    #[cfg(feature = "serde")]
    pub use
    {
        hexga_file_system::prelude::*,
        hexga_serde::prelude::*
    };

    #[allow(hidden_glob_reexports)]
    pub(crate) mod prelude {}
}