#![no_std]

//! A poor wrapper arround [bytemuck](https://docs.rs/bytemuck/latest/bytemuck/).
//!
//! A lot of code and documentation was taken from bytemuck.
use hexga_map_on::prelude::*;
use core::{marker::{PhantomData, PhantomPinned}, num::*};

mod zeroable;
pub use zeroable::*;

mod zeroable_in_option;
pub use zeroable_in_option::*;


#[cfg(feature = "extern_crate_alloc")]
extern crate alloc;

#[cfg(feature = "extern_crate_std")]
extern crate std;

pub mod prelude
{
    pub use super::{Zeroable,Zeroed};

    #[cfg(feature = "derive")]
    pub use hexga_core_mem_derive::Zeroable;
}

