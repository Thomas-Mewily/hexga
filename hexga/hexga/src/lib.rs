pub use hexga_core::*;
pub use hexga_generational as generational;
pub use hexga_math as math;
pub use hexga_bitflags as bitflags;
pub use hexga_ansi_color as ansi_color;
pub use hexga_utils as utils;
pub use hexga_map_on as map_on;
pub use hexga_image::{color,image};
//pub use hexga_random as random;
pub use hexga_encoding as encoding;

//pub use hexga_undo_redo as undo;

#[cfg(feature = "serde")]
pub use hexga_io as io;

#[cfg(feature = "hexga_asset")]
pub use hexga_asset as asset;

#[cfg(all(test, feature = "serde"))]
mod serde_test;

pub mod core_prelude{}



pub mod prelude
{
    pub use hexga_core::prelude::*;
    pub use super::hexga_prelude::*;

    #[allow(hidden_glob_reexports)]
    pub(crate) mod prelude {}
}

#[doc(hidden)]
/// Hexga specific prelude without the core.
///
/// You are probably looking for the `prelude` module.
pub mod hexga_prelude
{

    pub use crate::generational::prelude::*;
    pub use crate::math::prelude::*;
    pub use crate::bitflags::bitindex;
    pub use crate::utils::prelude::*;
    pub use hexga_image::prelude::*;
    //pub use crate::random::*;
    pub use crate::singleton::prelude::*;
    pub use crate::encoding::prelude::*;
    //pub use crate::map_on::prelude::*;

    /*
    #[allow(unused_imports)]
    #[cfg(feature = "hexga_io")]
    pub use hexga_io;*/
    // #[allow(unused_imports)]
    // #[cfg(feature = "hexga_io")]
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
        hexga_io::prelude::*,
    };

    #[cfg(feature = "hexga_asset")]
    pub use hexga_asset::prelude::*;

    #[allow(hidden_glob_reexports)]
    pub(crate) mod prelude {}
}