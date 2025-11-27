use super::*;
use std::{ops::{Deref,DerefMut}, thread::ThreadId, cell::UnsafeCell};

/// A value that can be read and written by only one thread.
///
/// Ownership is granted to the first thread that accesses it, first come, first served.
/// Once a thread owns the value, no other thread can read or modify it.
///
/// Even though `SingleThread<T>` implements [`Sync`] and [`Send`], it can only be accessed
/// by the first thread that uses it. Any attempt to access it from a different thread will fail.
pub struct SingleThread<T>
{
    value: UnsafeCell<T>,
    id: std::sync::OnceLock<ThreadId>,
}
unsafe impl<T> Sync for SingleThread<T> {}
unsafe impl<T> Send for SingleThread<T> {}

impl<T> SingleThread<T>
{
    pub const fn new(value: T) -> Self
    {
        Self { value: UnsafeCell::new(value), id: std::sync::OnceLock::new() }
    }
    #[inline(always)]
    pub(crate) fn same_thread(&self) -> Result<(),DifferentThreadError>
    {
        if std::thread::current().id() == *self.id.get_or_init(|| std::thread::current().id()) { Ok(()) } else { Err(DifferentThreadError) }
    }
    #[inline(always)]
    pub(crate) fn asset_same_thread(&self)
    {
        self.same_thread().unwrap();
    }
}



pub struct DifferentThreadError;
impl Debug for DifferentThreadError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "different thread")
    }
}

impl<T> ReadGuard for SingleThread<T>
{
    type Target=T;
    type ReadGuard<'a> = ReferenceReadGuard<'a,T> where Self: 'a;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.asset_same_thread();
        ReferenceReadGuard::new(unsafe { self.value.as_ref_unchecked() })
    }
}
impl<T> TryReadGuard for SingleThread<T>
{
    type Error<'a> = DifferentThreadError where Self: 'a;
    fn try_read<'a>(&'a self) -> Result<Self::ReadGuard<'a>, Self::Error<'a>> {
        self.same_thread()?;
        Ok(ReferenceReadGuard::new(unsafe { self.value.as_ref_unchecked() }))
    }
}
impl<T> WriteGuard for SingleThread<T>
{
    type WriteGuard<'a> = ReferenceWriteGuard<'a, T> where Self: 'a;
    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.asset_same_thread();
        ReferenceWriteGuard::new(unsafe { self.value.as_mut_unchecked() })
    }
}
impl<T> TryWriteGuard for SingleThread<T>
{
    type Error<'a> = DifferentThreadError where Self: 'a;
    fn try_write<'a>(&'a self) -> Result<Self::WriteGuard<'a>, Self::Error<'a>> {
        self.same_thread()?;
        Ok(ReferenceWriteGuard::new(unsafe { self.value.as_mut_unchecked() }))
    }
}
impl<T> Deref for SingleThread<T>
{
    type Target=T;
    #[inline(always)]
    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.read().inner_reference
    }
}
impl<T> DerefMut for SingleThread<T>
{
    #[inline(always)]
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.write().inner_reference
    }
}