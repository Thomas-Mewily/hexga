use core::{cell::UnsafeCell, pin::Pin};

#[cfg(not(feature = "std"))]
use ::{alloc::boxed::Box, core::cell::RefCell};
#[cfg(feature = "std")]
use std::{
    cell::RefCell,
    sync::{Mutex, RwLock},
};

pub trait Wrapper
{
    type Inside;
}

pub trait WrapperNew : Wrapper
{
    fn new_wrapper(value: Self::Inside) -> Self;
}

pub trait WrapperIntoInner : Wrapper
{
    fn into_inner(self) -> Self::Inside;
}

pub trait WrapperTryIntoInner : Wrapper
{
    fn into_inner(self) -> Option<Self::Inside>;
}
impl<T> WrapperTryIntoInner for T where T: WrapperIntoInner
{
    fn into_inner(self) -> Option<Self::Inside> {
        Some(WrapperIntoInner::into_inner(self))
    }
}
pub trait WrapperTryIntoInnerOrClone : WrapperTryIntoInner
{
    fn into_inner_or_clone(self) -> Self::Inside;
}

impl<T> Wrapper for Option<T>
{
    type Inside = T;
}
impl<T> WrapperNew for Option<T>
{
    fn new_wrapper(value: Self::Inside) -> Self { Self::Some(value) }
}
impl<T> WrapperTryIntoInner for Option<T>
{
    fn into_inner(self) -> Option<Self::Inside> { self }
}

impl<T, E> Wrapper for Result<T, E>
{
    type Inside = T;
}
impl<T, E> WrapperNew for Result<T, E>
{
    fn new_wrapper(value: Self::Inside) -> Self { Self::Ok(value) }
}
impl<T, E> WrapperTryIntoInner for Result<T, E>
{
    fn into_inner(self) -> Option<Self::Inside> {
        self.ok()
    }
}

impl<T> Wrapper for Box<T>
{
    type Inside = T;
}
impl<T> WrapperNew for Box<T>
{
    fn new_wrapper(value: Self::Inside) -> Self { Self::new(value) }
}
impl<T> WrapperIntoInner for Box<T>
{
    fn into_inner(self) -> Self::Inside {
        *self
    }
}


impl<T> Wrapper for Pin<Box<T>>
{
    type Inside = T;
}
impl<T> WrapperNew for Pin<Box<T>>
{
    fn new_wrapper(value: Self::Inside) -> Self { Box::pin(value) }
}
impl<T: core::marker::Unpin> WrapperIntoInner for Pin<Box<T>>
{
    fn into_inner(self) -> Self::Inside {
        *Pin::into_inner(self)
    }
}

impl<T> Wrapper for UnsafeCell<T>
{
    type Inside = T;
}
impl<T> WrapperNew for UnsafeCell<T>
{
    fn new_wrapper(value: Self::Inside) -> Self { Self::new(value) }
}
impl<T> WrapperIntoInner for UnsafeCell<T>
{
    fn into_inner(self) -> Self::Inside {
        self.into_inner()
    }
}

impl<T> Wrapper for RefCell<T>
{
    type Inside = T;
}
impl<T> WrapperNew for RefCell<T>
{
    fn new_wrapper(value: Self::Inside) -> Self { Self::new(value) }
}
impl<T> WrapperIntoInner for RefCell<T>
{
    fn into_inner(self) -> Self::Inside {
        RefCell::into_inner(self)
    }
}

#[cfg(feature = "std")]
mod std_impl
{
    use super::*;

    impl<T> Wrapper for Mutex<T>
    {
        type Inside = T;
    }
    impl<T> WrapperNew for Mutex<T>
    {
        fn new_wrapper(value: Self::Inside) -> Self { Self::new(value) }
    }
    // Maybe I should just impl `WrapperTryIntoInner` and return None when poisoned
    impl<T> WrapperIntoInner for Mutex<T>
    {
        fn into_inner(self) -> Self::Inside 
        {
            match Mutex::into_inner(self)
            {
                Ok(v) => v,
                Err(v) => v.into_inner(),
            }
        }
    }
    impl<T> WrapperTryIntoInnerOrClone for Mutex<T> where T: Clone
    {
        fn into_inner_or_clone(self) -> Self::Inside {
            WrapperIntoInner::into_inner(self)
        }
    }

    impl<T> Wrapper for RwLock<T>
    {
        type Inside = T;
    }
    impl<T> WrapperNew for RwLock<T>
    {
        fn new_wrapper(value: Self::Inside) -> Self { Self::new(value) }
    }
    impl<T> WrapperIntoInner for RwLock<T>
    {
        fn into_inner(self) -> Self::Inside {
            match RwLock::into_inner(self)
            {
                Ok(v) => v,
                Err(v) => v.into_inner(),
            }
        }
    }
    impl<T> WrapperTryIntoInnerOrClone for RwLock<T> where T: Clone
    {
        fn into_inner_or_clone(self) -> Self::Inside {
            WrapperIntoInner::into_inner(self)
        }
    }
}

pub mod prelude
{
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{Wrapper, WrapperNew, WrapperIntoInner, WrapperTryIntoInner, WrapperTryIntoInnerOrClone};
}

/*
// unstable library feature
impl<T> Wrapper for SyncUnsafeCell<T>
{
    type Inside=T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}
*/
