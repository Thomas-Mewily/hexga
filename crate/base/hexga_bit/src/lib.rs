#![no_std]
//! A poor wrapper arround [bytemuck](https://docs.rs/bytemuck/latest/bytemuck/).
//! Seriously don't use it and use bytemuck instead.
//!
//! A lot of code and documentation was taken from bytemuck
//! (mostly for trait safety, study it, and doing some renaming...).
//! Some comment aren't fixed...

use core::{
    marker::{PhantomData, PhantomPinned},
    num::*,
};
use hexga_map_on::prelude::*;

mod bit_zero;
pub use bit_zero::*;

mod bit_zero_in_option;
pub use bit_zero_in_option::*;

mod bit_pattern;
pub use bit_pattern::*;

mod pod;
pub use pod::*;

mod result;
pub use result::*;

mod pod_in_option;
pub use pod_in_option::*;

mod bit_any_pattern;
pub use bit_any_pattern::*;

mod bit_all_used;
pub use bit_all_used::*;

#[cfg(feature = "extern_crate_alloc")]
extern crate alloc;

#[cfg(feature = "extern_crate_std")]
extern crate std;

pub mod prelude
{
    pub use super::{
        BitAllUsed, BitAnyPattern, BitError, BitPattern, BitResult, BitZero, BitZeroed, Pod,
    };

    #[cfg(feature = "derive")]
    pub use hexga_bit_derive::{BitAllUsed, BitAnyPattern, BitPattern, BitZero, Pod};
}
