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


pub trait MapReadGuard<'a, T> where T: ?Sized
{
    type Mapped<U> where U: 'a;
    fn map<U, F>(self, f: F) -> Self::Mapped<U>
        where F: FnOnce(&T) -> &U;
}
pub trait MapWriteGuard<'a, T> where T: ?Sized
{
    type Mapped<U> where U: 'a;
    fn map<U, F>(self, f: F) -> Self::Mapped<U>
        where F: FnOnce(&mut T) -> &mut U;
}

pub fn map<'a, G, T, U, F>(guard: G, f: F) -> G::Mapped<U>
    where
        F: FnOnce(&T) -> &U,
        T: ?Sized,
        G: MapReadGuard<'a,T>
{
    G::map(guard, f)
}

pub fn map_mut<'a, G, T, U, F>(guard: G, f: F) -> G::Mapped<U>
    where
        F: FnOnce(&mut T) -> &mut U,
        T: ?Sized,
        G: MapWriteGuard<'a,T>
{
    G::map(guard, f)
}


// implementations:

pub struct ReferenceReadGuard<'a,T> where T: ?Sized
{
    #[doc(hidden)]
    pub inner_reference: &'a T,
}
impl<'a,T> ReferenceReadGuard<'a,T> where T: ?Sized
{
    pub const fn new(value: &'a T) -> Self { Self { inner_reference: value } }

    #[inline(always)]
    pub fn map<U, F>(orig: Self, f: F) -> ReferenceReadGuard<'a, U>
    where
        F: FnOnce(&T) -> &U,
        U: ?Sized
    {
        ReferenceReadGuard::new(f(orig.inner_reference))
    }
}
impl<'a,T> Deref for ReferenceReadGuard<'a, T> where T: ?Sized
{
    type Target=T;
    fn deref(&self) -> &Self::Target { self.inner_reference }
}
impl<'a,T> From<&'a T> for ReferenceReadGuard<'a,T> where T: ?Sized
{
    fn from(value: &'a T) -> Self {
        Self::new(value)
    }
}

pub struct ReferenceWriteGuard<'a,T> where T: ?Sized
{
    #[doc(hidden)]
    pub inner_reference: &'a mut T,
}
impl<'a,T> ReferenceWriteGuard<'a,T> where T: ?Sized
{
    pub const fn new(value: &'a mut T) -> Self { Self { inner_reference: value } }

    #[inline(always)]
    pub fn map<U, F>(orig: Self, f: F) -> ReferenceWriteGuard<'a, U>
    where
        F: FnOnce(&mut T) -> &mut U,
        U: ?Sized
    {
        ReferenceWriteGuard::new(f(orig.inner_reference))
    }
}
impl<'a,T> Deref for ReferenceWriteGuard<'a, T> where T: ?Sized
{
    type Target=T;
    fn deref(&self) -> &Self::Target { self.inner_reference }
}
impl<'a,T> DerefMut for ReferenceWriteGuard<'a, T> where T: ?Sized
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.inner_reference }
}
impl<'a,T> From<&'a mut T> for ReferenceWriteGuard<'a,T> where T: ?Sized
{
    fn from(value: &'a mut T) -> Self {
        Self::new(value)
    }
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

impl<'a, T> MapReadGuard<'a, T> for std::cell::Ref<'a, T> {
    type Mapped<U> = std::cell::Ref<'a, U> where U: 'a;
    fn map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&T) -> &U { std::cell::Ref::map(self, f) }
}

impl<'a, T> MapWriteGuard<'a, T> for std::cell::RefMut<'a, T> {
    type Mapped<U> = std::cell::RefMut<'a, U> where U: 'a;
    fn map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&mut T) -> &mut U, { std::cell::RefMut::map(self, f) }
}

impl<'a, T> MapReadGuard<'a, T> for ReferenceReadGuard<'a, T> {
    type Mapped<U> = ReferenceReadGuard<'a, U> where U: 'a;
    fn map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&T) -> &U { ReferenceReadGuard::map(self, f) }
}

impl<'a, T> MapWriteGuard<'a, T> for ReferenceWriteGuard<'a, T> {
    type Mapped<U> = ReferenceWriteGuard<'a, U> where U: 'a;
    fn map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&mut T) -> &mut U, { ReferenceWriteGuard::map(self, f) }
}




impl<'a, T> MapReadGuard<'a, T> for std::sync::RwLockReadGuard<'a, T> {
    type Mapped<U> = std::sync::MappedRwLockReadGuard<'a, U> where U: 'a;
    fn map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&T) -> &U { std::sync::RwLockReadGuard::map(self, f) }
}

impl<'a, T> MapWriteGuard<'a, T> for std::sync::RwLockWriteGuard<'a, T> {
    type Mapped<U> = std::sync::MappedRwLockWriteGuard<'a, U> where U: 'a;
    fn map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&mut T) -> &mut U, { std::sync::RwLockWriteGuard::map(self, f) }
}
impl<'a, T> MapReadGuard<'a, T> for std::sync::MappedRwLockReadGuard<'a, T> {
    type Mapped<U> = std::sync::MappedRwLockReadGuard<'a, U> where U: 'a;
    fn map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&T) -> &U { std::sync::MappedRwLockReadGuard::map(self, f) }
}

impl<'a, T> MapWriteGuard<'a, T> for std::sync::MappedRwLockWriteGuard<'a, T> {
    type Mapped<U> = std::sync::MappedRwLockWriteGuard<'a, U> where U: 'a;
    fn map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&mut T) -> &mut U, { std::sync::MappedRwLockWriteGuard::map(self, f) }
}

impl<'a, T> MapWriteGuard<'a, T> for std::sync::MutexGuard<'a, T> {
    type Mapped<U> = std::sync::MappedMutexGuard<'a, U> where U: 'a;
    fn map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&mut T) -> &mut U { std::sync::MutexGuard::map(self, f) }
}
impl<'a, T> MapWriteGuard<'a, T> for std::sync::MappedMutexGuard<'a, T> {
    type Mapped<U> = std::sync::MappedMutexGuard<'a, U> where U: 'a;
    fn map<U, F>(self, f: F) -> Self::Mapped<U> where F: FnOnce(&mut T) -> &mut U { std::sync::MappedMutexGuard::map(self, f) }
}


