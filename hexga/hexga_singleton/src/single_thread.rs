//! Define some Singleton traits and macro.
//!
use std::thread::ThreadId;
use super::*;

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
    pub(crate) fn same_thread(&self) -> Result<(),()>
    {
        if std::thread::current().id() == *self.id.get_or_init(|| std::thread::current().id()) { Ok(()) } else { Err(()) }
    }
}
impl<'a,T> TryBorrow for &'a SingleThread<T>
{
    type Borrow = &'a T;
    type Error = ();

    fn try_borrow(self) -> Result<Self::Borrow, Self::Error> {
        self.same_thread()?;
        Ok(unsafe { self.value.as_ref_unchecked() })
    }
}
impl<'a,T> TryBorrowMut for &'a SingleThread<T>
{
    type Borrow=&'a mut T;
    type Error = ();

    fn try_borrow_mut(self) -> Result<Self::Borrow, Self::Error> {
        self.same_thread()?;
        Ok(unsafe { self.value.as_mut_unchecked() })
    }
}
impl<T> Deref for SingleThread<T>
{
    type Target=T;
    #[inline(always)]
    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.borrow()
    }
}
impl<T> DerefMut for SingleThread<T>
{
    #[inline(always)]
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.borrow_mut()
    }
}