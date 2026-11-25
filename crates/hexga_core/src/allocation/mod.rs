use super::*;

pub use core::alloc::Allocator;

#[cfg(not(feature = "std"))]
pub type AllocGlobal = ::alloc::alloc::Global;
#[cfg(feature = "std")]
pub type AllocGlobal = std::alloc::Global;
#[cfg(feature = "std")]
pub type AllocSystem = std::alloc::System;

pub mod prelude
{
    #[cfg(feature = "std")]
    pub use super::AllocSystem;
    pub use super::{Allocator, AllocGlobal, traits::*};
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
}

mod alloc;
pub use alloc::*;

mod layout;
pub use layout::*;

mod block;
pub use block::*;
