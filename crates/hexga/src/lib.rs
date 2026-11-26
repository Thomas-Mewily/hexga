pub mod experimental {
    pub use hexga_arena as arena;
}

pub use bit_flags as bitflags;
pub use hexga_ansi_color as ansi_color;
pub use hexga_bit_flags as bit_flags;
pub use hexga_bit_set as bit_set;
pub use hexga_core::*;
//pub use hexga_core as core;
pub use hexga_array_vec as array_vec;
pub use hexga_generational as generational;
pub use hexga_image::{color, image};
pub use hexga_map_on as map_on;
pub use hexga_math as math;
pub use hexga_utils as utils;
//pub use hexga_random as random;
pub use hexga_encoding as encoding;
pub use hexga_io as io;
//pub use hexga_save::{io,encoding};
pub use hexga_asset as asset;

// Having a module `str` and a type `str` seem to cause some problem, hence the explicit `str` type import
pub use hexga_core::primitive::str;

//pub use hexga_undo_redo as undo;

//#[cfg(feature = "serde")]

//#[cfg(feature = "hexga_asset")]
//pub use hexga_asset as asset;

//#[cfg(all(test, feature = "serde"))]
//mod serde_test;

pub mod core_prelude {}

pub mod prelude {
    pub use super::hexga_prelude::*;
    pub use hexga_core::prelude::*;

    #[allow(hidden_glob_reexports)]
    pub(crate) mod prelude {}
}

pub mod traits {
    pub use crate::hexga_traits::*;
}

#[doc(hidden)]
/// Hexga specific prelude without the core.
///
/// You are probably looking for the `prelude` module.
pub mod hexga_prelude {
    pub use crate::array_vec::prelude::*;
    pub use crate::bit_flags::bit_index;
    pub use crate::bit_set::prelude::*;
    pub use crate::generational::prelude::*;
    pub use crate::math::prelude::*;
    pub use crate::utils::prelude::*;
    pub use hexga_image::prelude::*;

    //pub use crate::random::*;
    // #[allow(unused_imports)]
    pub use crate::asset::prelude::*;
    pub use crate::encoding::prelude::*;
    pub use crate::io::prelude::*;

    #[allow(unused_imports)]
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor, ser::SerializeStruct};

    #[allow(hidden_glob_reexports)]
    pub(crate) mod prelude {}

    pub use super::hexga_traits::*;
}

#[doc(hidden)]
pub mod hexga_traits {
    #[allow(unused_imports)] // The bit-set traits module is currently empty.
    pub use crate::bit_set::traits::*;
    pub use crate::generational::traits::*;
    pub use crate::math::traits::*;
    pub use crate::utils::traits::*;
    pub use hexga_image::traits::*;

    pub use crate::asset::traits::*;
    pub use crate::encoding::traits::*;
    pub use crate::io::traits::*;

    pub use super::fullname_crate::*;
}

pub use fullname_crate::*;

#[doc(hidden)]
pub mod fullname_crate {
    #[doc(hidden)] // For macro like new_unit
    pub use hexga_math;
}
