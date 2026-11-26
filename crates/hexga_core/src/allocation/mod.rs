use super::*;

pub use core::alloc::Allocator;

#[cfg(not(feature = "std"))]
pub use ::alloc::alloc::Global as AllocGlobal;

#[cfg(feature = "std")]
pub use std::alloc::Global as AllocGlobal;

#[cfg(feature = "std")]
pub use std::alloc::System as AllocSystem;

pub mod prelude
{
    #[cfg(feature = "std")]
    pub use super::AllocSystem;
    pub use super::{AllocGlobal, traits::*};
}

pub mod traits
{
    pub use super::alloc::{
        Alloc,
        AllocFromLayout,
        AllocFromLayoutRaw,
        DeallocFromLayout,
        DeallocFromLayoutRaw, //MemoryAlloc,MemoryRealloc,MemoryDealloc,
    };
    pub use super::layout::FromAllocLayout;
    pub use super::Allocator;
}

mod alloc;
pub use alloc::*;

mod layout;
pub use layout::*;

mod block;
pub use block::*;
