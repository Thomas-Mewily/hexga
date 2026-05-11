use super::*;
use core::ops::{Deref, DerefMut};

// Todo: do a NonBlockingGuarded / NonBlockingGuardedMut

pub trait Guarded<T: ?Sized>
{
    type Guard<'a>: Deref<Target = T>
    where
        Self: 'a;
    
    type Error<'a>: Debug
    where
        Self: 'a;

    /// Tries to return a guard that dereferences to the inner value,
    /// while blocking the current thread until it is able to do so.
    /// 
    /// This function never panics.
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>>;

    /// Returns a guard that dereferences to the inner value,
    /// while blocking the current thread until it is able to do so.
    /// 
    /// May panic on exceptional behavior (ex poisoned mutex, wrong thread for SingleThreadCell...)
    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a> { self.try_get().expect("poisoned") }
}

pub trait GuardedMut<T: ?Sized>
{
    type GuardMut<'a>: DerefMut<Target = T>
    where
        Self: 'a;
    type Error<'a>: Debug
    where
        Self: 'a;

    /// Tries to return a mutable guard that dereferences to the inner value,
    /// while blocking the current thread until it is able to do so.
    /// 
    /// This function never panics.
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>>;

    /// Returns a mutable guard that dereferences to the inner value,
    /// while blocking the current thread until it is able to do so.
    ///
    /// May panic on exceptional behavior (ex poisoned mutex, wrong thread for SingleThreadCell)
    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> { self.write().expect("poisoned") }
}
pub trait Guard<'a, T>: Deref<Target = T>
where
    T: ?Sized,
{
    type Mapped<U>
    where
        U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U>
    where
        F: FnOnce(&T) -> &U,
        U: 'a + ?Sized;
}
pub trait GuardMut<'a, T>: DerefMut<Target = T>
where
    T: ?Sized,
{
    type MappedMut<U>
    where
        U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U>
    where
        F: FnOnce(&mut T) -> &mut U,
        U: 'a + ?Sized;
}

pub fn map<'a, G, T, U, F>(guard: G, f: F) -> G::Mapped<U>
where
    F: FnOnce(&T) -> &U,
    T: ?Sized,
    G: Guard<'a, T>,
{
    G::guard_map(guard, f)
}

pub fn map_mut<'a, G, T, U, F>(guard: G, f: F) -> G::MappedMut<U>
where
    F: FnOnce(&mut T) -> &mut U,
    T: ?Sized,
    G: GuardMut<'a, T>,
{
    G::guard_map_mut(guard, f)
}

// implementations:

impl<'a, T> Guard<'a, T> for &'a T
{
    type Mapped<U>
        = &'a U
    where
        U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U>
    where
        F: FnOnce(&'a T) -> &'a U,
        U: 'a + ?Sized,
    {
        f(self)
    }
}
impl<'a, T> Guard<'a, T> for &'a mut T
{
    type Mapped<U>
        = &'a U
    where
        U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U>
    where
        F: FnOnce(&'a T) -> &'a U,
        U: 'a + ?Sized,
    {
        f(self)
    }
}
impl<'a, T> GuardMut<'a, T> for &'a mut T
{
    type MappedMut<U>
        = &'a mut U
    where
        U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U>
    where
        F: FnOnce(&'a mut T) -> &'a mut U,
        U: 'a + ?Sized,
    {
        f(self)
    }
}

#[cfg(feature = "std")]
impl<T> Guarded<T> for std::cell::RefCell<T>
{
    type Guard<'a>
        = std::cell::Ref<'a, T>
    where
        Self: 'a;
    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a> { self.borrow() }

    type Error<'a>
        = std::cell::BorrowError
    where
        Self: 'a;
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { self.try_borrow() }
}
#[cfg(feature = "std")]
impl<T> GuardedMut<T> for std::cell::RefCell<T>
{
    type GuardMut<'a>
        = std::cell::RefMut<'a, T>
    where
        Self: 'a;
    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> { self.borrow_mut() }

    type Error<'a>
        = std::cell::BorrowMutError
    where
        Self: 'a;
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>>
    {
        self.try_borrow_mut()
    }
}


#[cfg(feature = "std")]
impl<T> Guarded<T> for std::sync::Mutex<T>
{
    type Guard<'a>
        = std::sync::MutexGuard<'a, T>
    where
        Self: 'a;

    type Error<'a>
        = std::sync::PoisonError<std::sync::MutexGuard<'a, T>>
    where
        Self: 'a;
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { self.lock() }
}

#[cfg(feature = "std")]
impl<T> GuardedMut<T> for std::sync::Mutex<T>
{
    type GuardMut<'a>
        = std::sync::MutexGuard<'a, T>
    where
        Self: 'a;

    type Error<'a>
        = std::sync::PoisonError<std::sync::MutexGuard<'a, T>>
    where
        Self: 'a;
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> { self.lock() }
}


#[cfg(feature = "std")]
impl<T> Guarded<T> for std::sync::RwLock<T>
{
    type Guard<'a>
        = std::sync::RwLockReadGuard<'a, T>
    where
        Self: 'a;

    type Error<'a>
        = std::sync::PoisonError<std::sync::RwLockReadGuard<'a, T>>
    where
        Self: 'a;
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { self.read() }
}
#[cfg(feature = "std")]
impl<T> GuardedMut<T> for std::sync::RwLock<T>
{
    type GuardMut<'a>
        = std::sync::RwLockWriteGuard<'a, T>
    where
        Self: 'a;

    type Error<'a>
        = std::sync::PoisonError<std::sync::RwLockWriteGuard<'a, T>>
    where
        Self: 'a;
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> { self.write() }
}

// TODO: impl it for SyncUnsafeCell<T> and all the std::sync::nonpoison::... type once they are stabilized

#[cfg(feature = "std")]
impl<'a, T: ?Sized> Guard<'a, T> for std::cell::Ref<'a, T>
{
    type Mapped<U>
        = std::cell::Ref<'a, U>
    where
        U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U>
    where
        F: FnOnce(&T) -> &U,
        U: 'a + ?Sized,
    {
        Self::map(self, f)
    }
}
// Todo: impl Guard for RefMut when downgrade() will be available
#[cfg(feature = "std")]
impl<'a, T: ?Sized> GuardMut<'a, T> for std::cell::RefMut<'a, T>
{
    type MappedMut<U>
        = std::cell::RefMut<'a, U>
    where
        U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U>
    where
        F: FnOnce(&mut T) -> &mut U,
        U: 'a + ?Sized,
    {
        Self::map(self, f)
    }
}

#[cfg(feature = "std")]
impl<'a, T: ?Sized> Guard<'a, T> for std::sync::RwLockReadGuard<'a, T>
{
    type Mapped<U>
        = std::sync::MappedRwLockReadGuard<'a, U>
    where
        U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U>
    where
        F: FnOnce(&T) -> &U,
        U: 'a + ?Sized,
    {
        Self::map(self, f)
    }
}

#[cfg(feature = "std")]
impl<'a, T: ?Sized> Guard<'a, T> for std::sync::RwLockWriteGuard<'a, T>
{
    type Mapped<U>
        = std::sync::MappedRwLockReadGuard<'a, U>
    where
        U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U>
    where
        F: FnOnce(&T) -> &U,
        U: 'a + ?Sized,
    {
        Self::downgrade(self).guard_map(f)
    }
}

#[cfg(feature = "std")]
impl<'a, T: ?Sized> GuardMut<'a, T> for std::sync::RwLockWriteGuard<'a, T>
{
    type MappedMut<U>
        = std::sync::MappedRwLockWriteGuard<'a, U>
    where
        U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U>
    where
        F: FnOnce(&mut T) -> &mut U,
        U: 'a + ?Sized,
    {
        Self::map(self, f)
    }
}

#[cfg(feature = "std")]
impl<'a, T> Guard<'a, T> for std::sync::MappedRwLockReadGuard<'a, T>
{
    type Mapped<U>
        = std::sync::MappedRwLockReadGuard<'a, U>
    where
        U: 'a + ?Sized;
    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U>
    where
        F: FnOnce(&T) -> &U,
        U: 'a + ?Sized,
    {
        Self::map(self, f)
    }
}
// Todo: impl Guard for MappedRwLockWriteGuard when downgrade() will be available
#[cfg(feature = "std")]
impl<'a, T> GuardMut<'a, T> for std::sync::MappedRwLockWriteGuard<'a, T>
{
    type MappedMut<U>
        = std::sync::MappedRwLockWriteGuard<'a, U>
    where
        U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U>
    where
        F: FnOnce(&mut T) -> &mut U,
        U: 'a + ?Sized,
    {
        Self::map(self, f)
    }
}

// Can't impl yet Guard for std::sync::MutexGuard and std::sync::MappedMutexGuard
// by delegating it to GuardMut, because fn guard_map expect a fn that return a mutable reference.
// More generally, you can't map a const reference with mutex guard:
// https://doc.rust-lang.org/std/sync/struct.MutexGuard.html#method.map is `&mut` only

#[cfg(feature = "std")]
impl<'a, T> GuardMut<'a, T> for std::sync::MutexGuard<'a, T>
{
    type MappedMut<U>
        = std::sync::MappedMutexGuard<'a, U>
    where
        U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U>
    where
        F: FnOnce(&mut T) -> &mut U,
        U: 'a + ?Sized,
    {
        Self::map(self, f)
    }
}
#[cfg(feature = "std")]
impl<'a, T> GuardMut<'a, T> for std::sync::MappedMutexGuard<'a, T>
{
    type MappedMut<U>
        = std::sync::MappedMutexGuard<'a, U>
    where
        U: 'a + ?Sized;
    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U>
    where
        F: FnOnce(&mut T) -> &mut U,
        U: 'a + ?Sized,
    {
        Self::map(self, f)
    }
}
