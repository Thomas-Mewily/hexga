use hexga_core::{alloc::*, collections::singly_linked::SinglyLinkedNode, prelude::*};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub mod prelude
{
    pub use hexga_core::alloc::prelude::*;
    pub use super::{Arena, Arenable};
}

pub trait Arenable:
    Length
    + AllocFromLayout<AllocLayout, Output = AllocOutput>
    + Capacity
    + WithCapacity
    + From<AllocLayout>
    + ManagedBox
{
    /// Returns the number of bytes currently used in the buffer.
    fn nb_used(&self) -> usize { self.len() }

    /// Returns the number of bytes remaining in the buffer.
    fn remaining(&self) -> usize { self.capacity() - self.len() }

    fn used_coef(&self) -> f32 { self.nb_used() as f32 / self.capacity() as f32 }
    fn used_pourcent(&self) -> f32 { self.used_coef() * 100. }

    fn contains(&self, ptr: NonNull<u8>) -> bool;
}
//impl<T> Arena for T where T: Length + Alloc + Capacity + WithCapacity + From<AllocLayout> {}

mod arena;
pub use arena::*;

mod arena_buffer;
pub use arena_buffer::*;
