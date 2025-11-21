use std::ops::{Deref,DerefMut};
use super::*;

pub trait ReadGuard : Sized
{
    type Target;
    type ReadGuard : Deref<Target = Self::Target>;
    /// Can panic on exceptional behavior (ex poisoned mutex)
    fn read(self) -> Self::ReadGuard;
}
pub trait TryReadGuard : ReadGuard
{
    type Error: Debug;
    fn try_read(self) -> Result<Self::ReadGuard, Self::Error>;
}

pub trait WriteGuard : ReadGuard
{
    type WriteGuard : DerefMut<Target = Self::Target>;
    /// Can panic on exceptional behavior (ex poisoned mutex)
    fn write(self) -> Self::WriteGuard;
}
pub trait TryWriteGuard : WriteGuard
{
    type Error: Debug;
    fn try_write(self) -> Result<Self::WriteGuard, Self::Error>;
}

impl<'a,T> ReadGuard for &'a std::cell::RefCell<T>
{
    type Target=T;
    type ReadGuard=std::cell::Ref<'a,T>;
    fn read(self) -> Self::ReadGuard { self.borrow() }
}
impl<'a,T> TryReadGuard for &'a std::cell::RefCell<T>
{
    type Error = std::cell::BorrowError;
    fn try_read(self) -> Result<Self::ReadGuard, Self::Error> { self.try_borrow() }
}
impl<'a,T> WriteGuard for &'a std::cell::RefCell<T>
{
    type WriteGuard = std::cell::RefMut<'a,T>;
    fn write(self) -> Self::WriteGuard { self.borrow_mut() }
}
impl<'a,T> TryWriteGuard for &'a std::cell::RefCell<T>
{
    type Error = std::cell::BorrowMutError;
    fn try_write(self) -> Result<Self::WriteGuard, Self::Error> { self.try_borrow_mut() }
}



impl<'a,T> ReadGuard for &'a std::sync::Mutex<T>
{
    type Target = T;
    type ReadGuard = std::sync::MutexGuard<'a,T>;
    fn read(self) -> Self::ReadGuard { self.lock().expect("poisoned") }
}
impl<'a,T> TryReadGuard for &'a std::sync::Mutex<T>
{
    type Error = std::sync::PoisonError<std::sync::MutexGuard<'a,T>>;
    fn try_read(self) -> Result<Self::ReadGuard, Self::Error> { self.lock() }
}
impl<'a,T> WriteGuard for &'a std::sync::Mutex<T>
{
    type WriteGuard=std::sync::MutexGuard<'a,T>;
    fn write(self) -> Self::WriteGuard { self.lock().expect("poisoned") }
}
impl<'a,T> TryWriteGuard for &'a std::sync::Mutex<T>
{
    type Error = std::sync::PoisonError<std::sync::MutexGuard<'a,T>>;
    fn try_write(self) -> Result<Self::WriteGuard, Self::Error> { self.lock() }
}


impl<'a,T> ReadGuard for &'a std::sync::RwLock<T>
{
    type Target = T;
    type ReadGuard = std::sync::RwLockReadGuard<'a,T>;
    fn read(self) -> Self::ReadGuard { self.read().expect("poisoned") }
}
impl<'a,T> TryReadGuard for &'a std::sync::RwLock<T>
{
    type Error = std::sync::PoisonError<std::sync::RwLockReadGuard<'a,T>>;
    fn try_read(self) -> Result<Self::ReadGuard, Self::Error> { self.read() }
}
impl<'a,T> WriteGuard for &'a std::sync::RwLock<T>
{
    type WriteGuard=std::sync::RwLockWriteGuard<'a,T>;
    fn write(self) -> Self::WriteGuard { self.write().expect("poisoned") }
}
impl<'a,T> TryWriteGuard for &'a std::sync::RwLock<T>
{
    type Error = std::sync::PoisonError<std::sync::RwLockWriteGuard<'a,T>>;
    fn try_write(self) -> Result<Self::WriteGuard, Self::Error> { self.write() }
}

// TODO: impl it for SyncUnsafeCell<T> and all the std::sync::nonpoison::... type once they are stabilized
