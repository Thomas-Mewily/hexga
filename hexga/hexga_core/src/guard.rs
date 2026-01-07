use std::ops::{Deref,DerefMut};
use super::*;

pub trait Guarded<T: ?Sized>
{
    type Guard<'a> : Deref<Target = T> where Self: 'a;
    /// Can panic on exceptional behavior (ex poisoned mutex)
    fn get<'a>(&'a self) -> Self::Guard<'a>;
}
pub trait TryGuarded<T: ?Sized> : Guarded<T>
{
    type Error<'a>: Debug where Self: 'a;
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>>;
}

pub trait GuardedMut<T: ?Sized>
{
    type GuardMut<'a> : DerefMut<Target = T> where Self: 'a;
    /// Can panic on exceptional behavior (ex poisoned mutex)
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a>;
}
pub trait TryGuardedMut<T: ?Sized> : GuardedMut<T>
{
    type Error<'a>: Debug where Self: 'a;
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>>;
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


impl<T> Guarded<T> for std::cell::RefCell<T>
{
    type Guard<'a> = std::cell::Ref<'a,T> where Self: 'a;
    fn get<'a>(&'a self) -> Self::Guard<'a> { self.borrow() }
}
impl<T> TryGuarded<T> for std::cell::RefCell<T>
{
    type Error<'a> = std::cell::BorrowError where Self: 'a;
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { self.try_borrow() }
}
impl<T> GuardedMut<T> for std::cell::RefCell<T>
{
    type GuardMut<'a> = std::cell::RefMut<'a,T> where Self: 'a;
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> { self.borrow_mut() }
}
impl<T> TryGuardedMut<T> for std::cell::RefCell<T>
{
    type Error<'a> = std::cell::BorrowMutError where Self: 'a;
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> { self.try_borrow_mut() }
}



impl<T> Guarded<T> for std::sync::Mutex<T>
{
    type Guard<'a> = std::sync::MutexGuard<'a,T> where Self: 'a;
    fn get<'a>(&'a self) -> Self::Guard<'a> { self.lock().expect("poisoned") }
}
impl<T> TryGuarded<T> for std::sync::Mutex<T>
{
    type Error<'a> = std::sync::PoisonError<std::sync::MutexGuard<'a,T>> where Self: 'a;
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { self.lock() }
}
impl<T> GuardedMut<T> for std::sync::Mutex<T>
{
    type GuardMut<'a>=std::sync::MutexGuard<'a,T> where Self: 'a;
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> { self.lock().expect("poisoned") }
}
impl<T> TryGuardedMut<T> for std::sync::Mutex<T>
{
    type Error<'a> = std::sync::PoisonError<std::sync::MutexGuard<'a,T>> where Self: 'a;
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> { self.lock() }
}


impl<T> Guarded<T> for std::sync::RwLock<T>
{
    type Guard<'a> = std::sync::RwLockReadGuard<'a,T> where Self: 'a;
    fn get<'a>(&'a self) -> Self::Guard<'a> { self.read().expect("poisoned") }
}
impl<T> TryGuarded<T> for std::sync::RwLock<T>
{
    type Error<'a> = std::sync::PoisonError<std::sync::RwLockReadGuard<'a,T>> where Self: 'a;
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { self.read() }
}
impl<T> GuardedMut<T> for std::sync::RwLock<T>
{
    type GuardMut<'a> = std::sync::RwLockWriteGuard<'a,T> where Self: 'a;
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> { self.write().expect("poisoned") }
}
impl<T> TryGuardedMut<T> for std::sync::RwLock<T>
{
    type Error<'a> = std::sync::PoisonError<std::sync::RwLockWriteGuard<'a,T>> where Self: 'a;
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> { self.write() }
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


