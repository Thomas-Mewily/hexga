use std::ops::{Deref,DerefMut};
use super::*;

pub trait ReadGuard<T: ?Sized> : Sized
{
    type ReadGuard<'a> : Deref<Target = T> where Self: 'a;
    /// Can panic on exceptional behavior (ex poisoned mutex)
    fn read<'a>(&'a self) -> Self::ReadGuard<'a>;
}
pub trait TryReadGuard<T: ?Sized> : ReadGuard<T>
{
    type Error<'a>: Debug where Self: 'a;
    fn try_read<'a>(&'a self) -> Result<Self::ReadGuard<'a>, Self::Error<'a>>;
}

pub trait WriteGuard<T: ?Sized> : ReadGuard<T>
{
    type WriteGuard<'a> : DerefMut<Target = T> where Self: 'a;
    /// Can panic on exceptional behavior (ex poisoned mutex)
    fn write<'a>(&'a self) -> Self::WriteGuard<'a>;
}
pub trait TryWriteGuard<T: ?Sized> : WriteGuard<T>
{
    type Error<'a>: Debug where Self: 'a;
    fn try_write<'a>(&'a self) -> Result<Self::WriteGuard<'a>, Self::Error<'a>>;
}


pub trait Guard<'a, T> : Deref<Target = T> where T: ?Sized
{
    type Mapped<U> where U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U>
        where F: FnOnce(&T) -> &U, U: 'a + ?Sized;
}
pub trait GuardMut<'a, T> : DerefMut<Target = T> where T: ?Sized
{
    type MappedMut<U> where U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U>
        where F: FnOnce(&mut T) -> &mut U, U: 'a + ?Sized;
}

pub fn map<'a, G, T, U, F>(guard: G, f: F) -> G::Mapped<U>
    where
        F: FnOnce(&T) -> &U,
        T: ?Sized,
        G: Guard<'a,T>
{
    G::guard_map(guard, f)
}

pub fn map_mut<'a, G, T, U, F>(guard: G, f: F) -> G::MappedMut<U>
    where
        F: FnOnce(&mut T) -> &mut U,
        T: ?Sized,
        G: GuardMut<'a,T>
{
    G::guard_map_mut(guard, f)
}


// implementations:


impl<'a,T> Guard<'a,T> for &'a T
{
    type Mapped<U> = &'a U where U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U>
        where F: FnOnce(&'a T) -> &'a U, U: 'a + ?Sized {
        f(self)
    }
}
impl<'a,T> Guard<'a,T> for &'a mut T
{
    type Mapped<U> = &'a U where U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U>
        where F: FnOnce(&'a T) -> &'a U, U: 'a + ?Sized {
        f(self)
    }
}
impl<'a,T> GuardMut<'a,T> for &'a mut T
{
    type MappedMut<U> = &'a mut U where U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U>
        where F: FnOnce(&'a mut T) -> &'a mut U, U: 'a + ?Sized {
        f(self)
    }
}


impl<T> ReadGuard<T> for std::cell::RefCell<T>
{
    type ReadGuard<'a> = std::cell::Ref<'a,T> where Self: 'a;
    fn read<'a>(&'a self) -> Self::ReadGuard<'a> { self.borrow() }
}
impl<T> TryReadGuard<T> for std::cell::RefCell<T>
{
    type Error<'a> = std::cell::BorrowError where Self: 'a;
    fn try_read<'a>(&'a self) -> Result<Self::ReadGuard<'a>, Self::Error<'a>> { self.try_borrow() }
}
impl<T> WriteGuard<T> for std::cell::RefCell<T>
{
    type WriteGuard<'a> = std::cell::RefMut<'a,T> where Self: 'a;
    fn write<'a>(&'a self) -> Self::WriteGuard<'a> { self.borrow_mut() }
}
impl<T> TryWriteGuard<T> for std::cell::RefCell<T>
{
    type Error<'a> = std::cell::BorrowMutError where Self: 'a;
    fn try_write<'a>(&'a self) -> Result<Self::WriteGuard<'a>, Self::Error<'a>> { self.try_borrow_mut() }
}



impl<T> ReadGuard<T> for std::sync::Mutex<T>
{
    type ReadGuard<'a> = std::sync::MutexGuard<'a,T> where Self: 'a;
    fn read<'a>(&'a self) -> Self::ReadGuard<'a> { self.lock().expect("poisoned") }
}
impl<T> TryReadGuard<T> for std::sync::Mutex<T>
{
    type Error<'a> = std::sync::PoisonError<std::sync::MutexGuard<'a,T>> where Self: 'a;
    fn try_read<'a>(&'a self) -> Result<Self::ReadGuard<'a>, Self::Error<'a>> { self.lock() }
}
impl<T> WriteGuard<T> for std::sync::Mutex<T>
{
    type WriteGuard<'a>=std::sync::MutexGuard<'a,T> where Self: 'a;
    fn write<'a>(&'a self) -> Self::WriteGuard<'a> { self.lock().expect("poisoned") }
}
impl<T> TryWriteGuard<T> for std::sync::Mutex<T>
{
    type Error<'a> = std::sync::PoisonError<std::sync::MutexGuard<'a,T>> where Self: 'a;
    fn try_write<'a>(&'a self) -> Result<Self::WriteGuard<'a>, Self::Error<'a>> { self.lock() }
}


impl<T> ReadGuard<T> for std::sync::RwLock<T>
{
    type ReadGuard<'a> = std::sync::RwLockReadGuard<'a,T> where Self: 'a;
    fn read<'a>(&'a self) -> Self::ReadGuard<'a> { self.read().expect("poisoned") }
}
impl<T> TryReadGuard<T> for std::sync::RwLock<T>
{
    type Error<'a> = std::sync::PoisonError<std::sync::RwLockReadGuard<'a,T>> where Self: 'a;
    fn try_read<'a>(&'a self) -> Result<Self::ReadGuard<'a>, Self::Error<'a>> { self.read() }
}
impl<T> WriteGuard<T> for std::sync::RwLock<T>
{
    type WriteGuard<'a> = std::sync::RwLockWriteGuard<'a,T> where Self: 'a;
    fn write<'a>(&'a self) -> Self::WriteGuard<'a> { self.write().expect("poisoned") }
}
impl<T> TryWriteGuard<T> for std::sync::RwLock<T>
{
    type Error<'a> = std::sync::PoisonError<std::sync::RwLockWriteGuard<'a,T>> where Self: 'a;
    fn try_write<'a>(&'a self) -> Result<Self::WriteGuard<'a>, Self::Error<'a>> { self.write() }
}

// TODO: impl it for SyncUnsafeCell<T> and all the std::sync::nonpoison::... type once they are stabilized

impl<'a, T: ?Sized> Guard<'a,T> for std::cell::Ref<'a,T>
{
    type Mapped<U> = std::cell::Ref<'a,U> where U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&T) -> &U, U: 'a + ?Sized { Self::map(self, f) }
}
// Todo: impl Guard for RefMut when downgrade() will be available
impl<'a, T: ?Sized> GuardMut<'a,T> for std::cell::RefMut<'a,T>
{
    type MappedMut<U> = std::cell::RefMut<'a, U> where U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U> where F: FnOnce(&mut T) -> &mut U, U: 'a + ?Sized { Self::map(self, f) }
}


impl<'a, T: ?Sized> Guard<'a, T> for std::sync::RwLockReadGuard<'a, T>
{
    type Mapped<U> = std::sync::MappedRwLockReadGuard<'a, U> where U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&T) -> &U, U: 'a + ?Sized { Self::map(self, f) }
}

impl<'a, T: ?Sized> Guard<'a, T> for std::sync::RwLockWriteGuard<'a, T>
{
    type Mapped<U> = std::sync::MappedRwLockReadGuard<'a, U> where U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&T) -> &U, U: 'a + ?Sized { Self::downgrade(self).guard_map(f) }
}
impl<'a, T: ?Sized> GuardMut<'a, T> for std::sync::RwLockWriteGuard<'a, T>
{
    type MappedMut<U> = std::sync::MappedRwLockWriteGuard<'a, U> where U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U> where F: FnOnce(&mut T) -> &mut U, U: 'a + ?Sized { Self::map(self, f) }
}

impl<'a, T> Guard<'a, T> for std::sync::MappedRwLockReadGuard<'a, T>
{
    type Mapped<U> = std::sync::MappedRwLockReadGuard<'a, U> where U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&T) -> &U, U: 'a + ?Sized { Self::map(self, f) }
}
// Todo: impl Guard for MappedRwLockWriteGuard when downgrade() will be available
impl<'a, T> GuardMut<'a, T> for std::sync::MappedRwLockWriteGuard<'a, T>
{
    type MappedMut<U> = std::sync::MappedRwLockWriteGuard<'a, U> where U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U> where F: FnOnce(&mut T) -> &mut U, U: 'a + ?Sized { Self::map(self, f) }
}

impl<'a, T> GuardMut<'a, T> for std::sync::MutexGuard<'a, T>
{
    type MappedMut<U> = std::sync::MappedMutexGuard<'a, U> where U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U> where F: FnOnce(&mut T) -> &mut U, U: 'a + ?Sized { Self::map(self, f) }
}
impl<'a, T> GuardMut<'a, T> for std::sync::MappedMutexGuard<'a, T>
{
    type MappedMut<U> = std::sync::MappedMutexGuard<'a, U> where U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U> where F: FnOnce(&mut T) -> &mut U, U: 'a + ?Sized { Self::map(self, f) }
}


