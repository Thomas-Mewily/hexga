use core::{cell::UnsafeCell, pin::Pin};

#[cfg(not(feature = "std"))]
use ::{alloc::boxed::Box, core::cell::RefCell};
#[cfg(feature = "std")]
use std::{
    cell::RefCell,
    sync::{Mutex, RwLock},
};

pub trait NewWrapper
{
    type Inside;
    fn new(value: Self::Inside) -> Self;
}

impl<T> NewWrapper for Option<T>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::Some(value) }
}
impl<T, E> NewWrapper for Result<T, E>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::Ok(value) }
}
impl<T> NewWrapper for Box<T>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}
impl<T> NewWrapper for Pin<Box<T>>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Box::pin(value) }
}
impl<T> NewWrapper for UnsafeCell<T>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}
impl<T> NewWrapper for RefCell<T>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}
#[cfg(feature = "std")]
impl<T> NewWrapper for Mutex<T>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}
#[cfg(feature = "std")]
impl<T> NewWrapper for RwLock<T>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}

pub mod prelude {}

pub mod traits
{
    pub use super::NewWrapper;
}
/*
// unstable library feature
impl<T> Wrapper for SyncUnsafeCell<T>
{
    type Inside=T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}
*/
