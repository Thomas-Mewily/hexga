
mod load_save;

pub mod io
{
    pub use hexga_io::*;

    pub use super::load_save::*;

    pub mod prelude
    {
        pub use hexga_io::prelude::*;
        pub use super::traits::*;
    }

    pub mod traits
    {
        pub use hexga_io::traits::*;
        pub use crate::load_save::*;
    }
}

pub mod encoding
{
    pub use hexga_encoding::*;
}



use prelude::*;
pub mod prelude
{
    pub use super::{io::prelude::*, encoding::prelude::*};
}

pub mod traits
{
    pub use super::{io::traits::*, encoding::traits::*};
}
