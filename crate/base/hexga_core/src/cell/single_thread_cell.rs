use super::*;

use std::{thread::ThreadId};

/// A value that can be read and written by only one thread.
///
/// Ownership is granted to the first thread that accesses it, first come, first served.
/// Once a thread owns the value, no other thread can read or modify it.
///
/// Even though `SingleThread<T>` implements [`Sync`] and [`Send`], it can only be accessed
/// by the first thread that uses it. Any attempt to access it from a different thread will fail.
pub struct SingleThreadCell<T>
{
    value: RefCell<T>,
    id: std::sync::OnceLock<ThreadId>,
}
unsafe impl<T> Sync for SingleThreadCell<T> {}
unsafe impl<T> Send for SingleThreadCell<T> {}

impl<T> SingleThreadCell<T>
{
    pub const fn new(value: T) -> Self
    {
        Self {
            value: RefCell::new(value),
            id: std::sync::OnceLock::new(),
        }
    }
    #[inline(always)]
    pub fn is_same_thread(&self) -> Result<(), DifferentThreadError>
    {
        if std::thread::current().id() == *self.id.get_or_init(|| std::thread::current().id())
        {
            Ok(())
        }
        else
        {
            Err(DifferentThreadError)
        }
    }
    #[inline(always)]
    pub(crate) fn assert_same_thread(&self) { self.is_same_thread().unwrap(); }
}



#[derive(Debug)]
pub enum SingleThreadError
{
    DifferentThread(DifferentThreadError),
    Borrow(BorrowError),
    NotInit(NotInitError),
}
impl From<DifferentThreadError> for SingleThreadError
{
    fn from(value: DifferentThreadError) -> Self { Self::DifferentThread(value) }
}
impl From<BorrowError> for SingleThreadError
{
    fn from(value: BorrowError) -> Self { Self::Borrow(value) }
}

impl<T> Guarded<T> for SingleThreadCell<T>
{
    type Guard<'a>
        = Ref<'a, T>
    where
        Self: 'a;

    fn get<'a>(&'a self) -> Self::Guard<'a>
    {
        self.assert_same_thread();
        self.value.borrow()
    }
}
impl<T> SingleThreadCell<T>
{
    #[inline]
    pub fn get<'a>(&'a self) -> Ref<'a, T>
    {
        Guarded::get(self)
    }
}

impl<T> TryGuarded<T> for SingleThreadCell<T>
{
    type Error<'a>
        = SingleThreadError
    where
        Self: 'a;
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>>
    {
        self.is_same_thread()?;
        Ok(self.value.try_borrow()?)
    }
}
impl<T> SingleThreadCell<T>
{
    #[inline]
    pub fn try_get<'a>(&'a self) -> Result<<Self as Guarded<T>>::Guard<'a>, <Self as TryGuarded<T>>::Error<'a>>
    {
        TryGuarded::try_get(self)
    }
}

#[derive(Debug)]
pub enum SingleThreadMutError
{
    DifferentThread(DifferentThreadError),
    BorrowMut(BorrowMutError),
    NotInit(NotInitError),
}
impl From<DifferentThreadError> for SingleThreadMutError
{
    fn from(value: DifferentThreadError) -> Self { Self::DifferentThread(value) }
}
impl From<BorrowMutError> for SingleThreadMutError
{
    fn from(value: BorrowMutError) -> Self { Self::BorrowMut(value) }
}
impl<T> GuardedMut<T> for SingleThreadCell<T>
{
    type GuardMut<'a>
        = RefMut<'a, T>
    where
        Self: 'a;
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a>
    {
        self.assert_same_thread();
        self.value.borrow_mut()
    }
}
impl<T> SingleThreadCell<T>
{
    #[inline]
    pub fn get_mut<'a>(&'a mut self) -> RefMut<'a, T>
    {
        GuardedMut::get_mut(self)
    }
}
impl<T> TryGuardedMut<T> for SingleThreadCell<T>
{
    type Error<'a>
        = SingleThreadMutError
    where
        Self: 'a;
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>>
    {
        self.is_same_thread()?;
        Ok(self.value.try_borrow_mut()?)
    }
}
impl<T> SingleThreadCell<T>
{
    #[inline]
    pub fn try_get_mut<'a>(&'a mut self) -> Result<<Self as GuardedMut<T>>::GuardMut<'a>, <Self as TryGuardedMut<T>>::Error<'a>>
    {
        TryGuardedMut::try_get_mut(self)
    }
}