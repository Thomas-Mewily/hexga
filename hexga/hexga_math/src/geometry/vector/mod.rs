use super::*;

mod array_extension;
pub use array_extension::*;

mod vector_n;
pub use vector_n::*;

mod vector1;
pub use vector1::*;

mod vector2;
pub use vector2::*;

mod vector3;
pub use vector3::*;

mod vector4;
pub use vector4::*;

mod vector_iter;
pub use vector_iter::*;

mod xyzw;
pub use xyzw::*;

mod typedef;
pub use typedef::*;


pub mod prelude
{
    pub use super::
    {
        array_extension::*,
        xyzw::*,
        typedef::*,
        vector_n::prelude::*,
        vector1::prelude::*,
        vector2::prelude::*,
        vector3::prelude::*,
        vector4::prelude::*,
        vector_iter::IterIndex,
    };
}