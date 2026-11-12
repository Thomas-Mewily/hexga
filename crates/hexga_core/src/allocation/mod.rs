use super::*;

pub mod prelude
{
    pub use super::traits::*;
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
