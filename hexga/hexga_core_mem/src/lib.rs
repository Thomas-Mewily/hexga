//#![no_std]
//! A poor wrapper arround bytemuck.

pub mod prelude
{
    pub use super::Foo;

    #[cfg(feature = "derive")]
    pub use hexga_core_mem_derive::Foo;
}

pub trait Foo
{
    fn foo()
    {
        println!("foo");
    }
}

/*
pub unsafe trait Zeroable {}

pub unsafe trait Pod {}
//pub unsafe trait ByteEq {}


pub mod prelude
{
    pub use super::{Pod,Zeroable};
    #[cfg(feature = "derive")]
    pub use super::{derive};
}
    */