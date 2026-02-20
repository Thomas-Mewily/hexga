use super::*;

pub mod prelude
{
    pub use super::{
        Alloc,
        AllocFromLayout,
        AllocFromLayoutRaw,
        DeallocFromLayout,
        DeallocFromLayoutRaw, //MemoryAlloc,MemoryRealloc,MemoryDealloc,
        FromAllocLayout,
    };
}

mod alloc;
pub use alloc::*;

mod layout;
pub use layout::*;

mod block;
pub use block::*;
