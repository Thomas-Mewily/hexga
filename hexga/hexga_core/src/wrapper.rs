use core::{
    cell::UnsafeCell,
    pin::Pin,
};

#[cfg(feature = "std")]
use std::{
    cell::RefCell,
    sync::{Mutex, RwLock},
};
#[cfg(not(feature = "std"))]
use alloc::{
    boxed::Box,
    cell::RefCell,
};

pub trait Wrapper
{
    type Inside;
    fn new(value: Self::Inside) -> Self;
}

impl<T> Wrapper for Option<T>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::Some(value) }
}
impl<T, E> Wrapper for Result<T, E>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::Ok(value) }
}
impl<T> Wrapper for Box<T>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}
impl<T> Wrapper for Pin<Box<T>>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Box::pin(value) }
}
impl<T> Wrapper for UnsafeCell<T>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}
impl<T> Wrapper for RefCell<T>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}
#[cfg(feature = "std")]
impl<T> Wrapper for Mutex<T>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}
#[cfg(feature = "std")]
impl<T> Wrapper for RwLock<T>
{
    type Inside = T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}

/*
// unstable library feature
impl<T> Wrapper for SyncUnsafeCell<T>
{
    type Inside=T;
    fn new(value: Self::Inside) -> Self { Self::new(value) }
}
*/
