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


pub struct ReferenceGuard<T>
{
    value: T,
}
impl<T> Default for ReferenceGuard<T> where T: Default
{
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl<T> ReferenceGuard<T>
{
    pub const fn new(value: T) -> Self { Self { value }}
    pub fn into_value(this: Self) -> T { this.value }
}
impl<'a,T> Deref for ReferenceGuard<&'a T>
{
    type Target=T;
    fn deref(&self) -> &Self::Target { &self.value }
}
impl<'a,T> Deref for ReferenceGuard<&'a mut T>
{
    type Target=T;
    fn deref(&self) -> &Self::Target { &self.value }
}
impl<'a, T> DerefMut for ReferenceGuard<&'a mut T>
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}


pub struct DifferentThreadError;
impl Debug for DifferentThreadError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "different thread")
    }
}

impl<'a,T> ReadGuard for &'a SingleThread<T>
{
    type Target=T;
    type ReadGuard = ReferenceGuard<&'a T>;

    fn read(self) -> Self::ReadGuard {
        self.asset_same_thread();
        ReferenceGuard::new(unsafe { self.value.as_ref_unchecked() })
    }
}
impl<'a,T> TryReadGuard for &'a SingleThread<T>
{
    type Error = DifferentThreadError;
    fn try_read(self) -> Result<Self::ReadGuard, Self::Error> {
        self.same_thread()?;
        Ok(ReferenceGuard::new(unsafe { self.value.as_ref_unchecked() }))
    }
}
impl<'a,T> WriteGuard for &'a SingleThread<T>
{
    type WriteGuard = ReferenceGuard<&'a mut T>;
    fn write(self) -> Self::WriteGuard {
        self.asset_same_thread();
        ReferenceGuard::new(unsafe { self.value.as_mut_unchecked() })
    }
}
impl<'a,T> TryWriteGuard for &'a SingleThread<T>
{
    type Error = DifferentThreadError;
    fn try_write(self) -> Result<Self::WriteGuard, Self::Error> {
        self.same_thread()?;
        Ok(ReferenceGuard::new(unsafe { self.value.as_mut_unchecked() }))
    }
}
impl<T> Deref for SingleThread<T>
{
    type Target=T;
    #[inline(always)]
    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.read().value
    }
}
impl<T> DerefMut for SingleThread<T>
{
    #[inline(always)]
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.write().value
    }
}