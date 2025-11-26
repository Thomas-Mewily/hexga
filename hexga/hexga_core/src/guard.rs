use std::ops::{Deref,DerefMut};
use super::*;

pub trait ReadGuard : Sized
{
    type Target;
    type ReadGuard<'a> : Deref<Target = Self::Target> where Self: 'a;
    /// Can panic on exceptional behavior (ex poisoned mutex)
    fn read<'a>(&'a self) -> Self::ReadGuard<'a>;
}
pub trait TryReadGuard : ReadGuard
{
    type Error<'a>: Debug where Self: 'a;
    fn try_read<'a>(&'a self) -> Result<Self::ReadGuard<'a>, Self::Error<'a>>;
}

pub trait WriteGuard : ReadGuard
{
    type WriteGuard<'a> : DerefMut<Target = Self::Target> where Self: 'a;
    /// Can panic on exceptional behavior (ex poisoned mutex)
    fn write<'a>(&'a self) -> Self::WriteGuard<'a>;
}
pub trait TryWriteGuard : WriteGuard
{
    type Error<'a>: Debug where Self: 'a;
    fn try_write<'a>(&'a self) -> Result<Self::WriteGuard<'a>, Self::Error<'a>>;
}




pub struct ReferenceReadGuard<'a,T>
{
    pub value: &'a T,
}
impl<'a,T> ReferenceReadGuard<'a,T>
{
    pub const fn new(value: &'a T) -> Self { Self { value }}
}
impl<'a,T> Deref for ReferenceReadGuard<'a, T>
{
    type Target=T;
    fn deref(&self) -> &Self::Target { self.value }
}

pub struct ReferenceWriteGuard<'a,T>
{
    pub value: &'a mut T,
}
impl<'a,T> ReferenceWriteGuard<'a,T>
{
    pub const fn new(value: &'a mut T) -> Self { Self { value }}
}
impl<'a,T> Deref for ReferenceWriteGuard<'a, T>
{
    type Target=T;
    fn deref(&self) -> &Self::Target { self.value }
}
impl<'a,T> DerefMut for ReferenceWriteGuard<'a, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.value }
}







impl<T> ReadGuard for std::cell::RefCell<T>
{
    type Target=T;
    type ReadGuard<'a> = std::cell::Ref<'a,T> where Self: 'a;
    fn read<'a>(&'a self) -> Self::ReadGuard<'a> { self.borrow() }
}
impl<T> TryReadGuard for std::cell::RefCell<T>
{
    type Error<'a> = std::cell::BorrowError where Self: 'a;
    fn try_read<'a>(&'a self) -> Result<Self::ReadGuard<'a>, Self::Error<'a>> { self.try_borrow() }
}
impl<T> WriteGuard for std::cell::RefCell<T>
{
    type WriteGuard<'a> = std::cell::RefMut<'a,T> where Self: 'a;
    fn write<'a>(&'a self) -> Self::WriteGuard<'a> { self.borrow_mut() }
}
impl<T> TryWriteGuard for std::cell::RefCell<T>
{
    type Error<'a> = std::cell::BorrowMutError where Self: 'a;
    fn try_write<'a>(&'a self) -> Result<Self::WriteGuard<'a>, Self::Error<'a>> { self.try_borrow_mut() }
}



impl<T> ReadGuard for std::sync::Mutex<T>
{
    type Target = T;
    type ReadGuard<'a> = std::sync::MutexGuard<'a,T> where Self: 'a;
    fn read<'a>(&'a self) -> Self::ReadGuard<'a> { self.lock().expect("poisoned") }
}
impl<T> TryReadGuard for std::sync::Mutex<T>
{
    type Error<'a> = std::sync::PoisonError<std::sync::MutexGuard<'a,T>> where Self: 'a;
    fn try_read<'a>(&'a self) -> Result<Self::ReadGuard<'a>, Self::Error<'a>> { self.lock() }
}
impl<T> WriteGuard for std::sync::Mutex<T>
{
    type WriteGuard<'a>=std::sync::MutexGuard<'a,T> where Self: 'a;
    fn write<'a>(&'a self) -> Self::WriteGuard<'a> { self.lock().expect("poisoned") }
}
impl<T> TryWriteGuard for std::sync::Mutex<T>
{
    type Error<'a> = std::sync::PoisonError<std::sync::MutexGuard<'a,T>> where Self: 'a;
    fn try_write<'a>(&'a self) -> Result<Self::WriteGuard<'a>, Self::Error<'a>> { self.lock() }
}


impl<T> ReadGuard for std::sync::RwLock<T>
{
    type Target = T;
    type ReadGuard<'a> = std::sync::RwLockReadGuard<'a,T> where Self: 'a;
    fn read<'a>(&'a self) -> Self::ReadGuard<'a> { self.read().expect("poisoned") }
}
impl<T> TryReadGuard for std::sync::RwLock<T>
{
    type Error<'a> = std::sync::PoisonError<std::sync::RwLockReadGuard<'a,T>> where Self: 'a;
    fn try_read<'a>(&'a self) -> Result<Self::ReadGuard<'a>, Self::Error<'a>> { self.read() }
}
impl<T> WriteGuard for std::sync::RwLock<T>
{
    type WriteGuard<'a> = std::sync::RwLockWriteGuard<'a,T> where Self: 'a;
    fn write<'a>(&'a self) -> Self::WriteGuard<'a> { self.write().expect("poisoned") }
}
impl<T> TryWriteGuard for std::sync::RwLock<T>
{
    type Error<'a> = std::sync::PoisonError<std::sync::RwLockWriteGuard<'a,T>> where Self: 'a;
    fn try_write<'a>(&'a self) -> Result<Self::WriteGuard<'a>, Self::Error<'a>> { self.write() }
}

// TODO: impl it for SyncUnsafeCell<T> and all the std::sync::nonpoison::... type once they are stabilized
