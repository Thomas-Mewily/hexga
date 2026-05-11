
use super::*;
use crate::{cell::{SingleThreadCell, SingleThreadError, SingleThreadMutError, Ref, RefMut}, identity::Identity};
use std::{
    cell::{LazyCell, OnceCell},
    sync::{
        LazyLock, MappedMutexGuard, MappedRwLockReadGuard, MappedRwLockWriteGuard, Mutex, OnceLock, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError
    },
};


// Todo: impl Default for singleton (init with default value) when const trait are here

//pub type Singleton<T> = SingletonMutex<T>;

mod rw;
pub use rw::*;

mod mutex;
pub use mutex::*;

mod cell;
pub use cell::*;

/*
mod once_cell;
pub use once_cell::*;

mod once_lock;
pub use once_lock::*;
*/

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotInitError;

impl Debug for NotInitError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("Not init")
    }
}

pub mod prelude
{
    pub(crate) use super::NotInitError;
    pub use super::traits::*;
    //pub use super::{SingletonLazyMutex, SingletonLazyRw, SingletonLazyCell};
}

pub mod traits
{
    pub use super::SingletonOptionable;
}

/// Single interface for every singleton type.
pub struct SingletonOf<T>
{
    pub(crate) guarded: T,
}
impl<T> SingletonOf<T>
{
    pub const fn from_guard(guard: T) -> Self { Self { guarded: guard }}
}
impl<T> Deref for SingletonOf<T>
where
    T: Deref,
{
    type Target = T::Target;
    #[inline(always)]
    #[track_caller]
    fn deref(&self) -> &Self::Target { self.guarded.deref() }
}
impl<T> DerefMut for SingletonOf<T>
where
    T: DerefMut,
{
    #[inline(always)]
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target { self.guarded.deref_mut() }
}


#[doc(hidden)]
#[macro_export]
macro_rules! impl_singleton_methods {
    ($singleton_type:ident) => {
        impl_singleton_methods_get!($singleton_type);
        impl_singleton_methods_get_mut!($singleton_type);
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! impl_singleton_methods_get_mut {
    ($singleton_type:ident) => {
        impl<T> $singleton_type<T> {
            /// Returns a mutable guard that dereferences to the inner value.
            ///
            /// May panic on exceptional behavior (ex poisoned mutex, wrong thread for SingleThreadCell)
            #[track_caller]
            #[inline]
            pub fn get_mut<'a>(&'a self) -> <Self as GuardedMut<T>>::GuardMut<'a> {
                GuardedMut::get_mut(self)
            }

            /// Tries to return a mutable guard that dereferences to the inner value.
            /// 
            /// This function never panics.
            #[track_caller]
            #[inline]
            pub fn try_get_mut<'a>(&'a self) -> Result<
                <Self as GuardedMut<T>>::GuardMut<'a>,
                <Self as TryGuardedMut<T>>::Error<'a>
            > {
                TryGuardedMut::try_get_mut(self)
            }
        }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! impl_singleton_methods_get {
    ($singleton_type:ident) => {
        impl<T> $singleton_type<T> {
            /// Returns a guard that dereferences to the inner value.
            /// 
            /// May panic on exceptional behavior (ex poisoned mutex, wrong thread for SingleThreadCell...)
            #[track_caller]
            #[inline]
            pub fn get<'a>(&'a self) -> <Self as Guarded<T>>::Guard<'a> {
                Guarded::get(self)
            }

            /// Tries to return a guard that dereferences to the inner value.
            /// 
            /// This function never panics.
            #[track_caller]
            #[inline]
            pub fn try_get<'a>(&'a self) -> Result<
                <Self as Guarded<T>>::Guard<'a>,
                <Self as TryGuarded<T>>::Error<'a>
            > {
                TryGuarded::try_get(self)
            }
        }
    };
}




pub trait SingletonOptionable<T>: 
    TryGuardedMut<T>
{
    /// Tries to initialize the singleton using a factory function.
    /// If already initialized, returns the existing guard without calling the function.
    /// Always returns a guard on success, regardless of whether initialization occurred.
    /// 
    /// # Returns
    /// - `Ok(guard)` - The singleton is now initialized (either just now or already)
    /// - `Err((error, init))` - Failed to acquire mutable access
    fn init_from_fn<'a,F>(&'a self, init: F) -> Result<Self::GuardMut<'a>, (Self::Error<'a>, F)> 
        where F: FnOnce() -> T;

    /// Swaps the value with an external Option.
    /// The singleton's value is exchanged with the provided Option.
    ///
    /// # Returns
    /// - `Ok(())` - Swap successful
    /// - `Err(error)` - Failed to acquire mutable access
    fn swap<'a>(&'a self, other: &mut Option<T>) -> Result<(), Self::Error<'a>>;

    /// Ensures the singleton is initialized with the given value.
    /// If already initialized, the existing value is kept and the provided value is discarded.
    /// Always returns a guard on success.
    ///
    /// # Returns
    /// - `Ok(guard)` - The singleton is initialized (kept existing or set new)
    /// - `Err((error, value))` - Failed to acquire mutable access
    fn init<'a>(&'a self, value: T) -> Result<Self::GuardMut<'a>, (Self::Error<'a>, T)> 
    {
        match self.init_from_fn(|| value)
        {
            Ok(v) => Ok(v),
            Err((e, init)) => Err((e, init())),
        }
    }


    /// Replaces the current value with the given option, returning the previous value.
    /// If `None` is provided, the singleton becomes uninitialized.
    ///
    /// # Returns
    /// - `Ok(Some(old))` - Previous value was replaced with `Some(new)`
    /// - `Ok(None)` - Was uninitialized (or became uninitialized if `None` provided)
    /// - `Err(error)` - Failed to acquire mutable access
    fn replace<'a>(&'a self, mut value: Option<T>) -> Result<Option<T>, Self::Error<'a>> {
        match self.swap(&mut value)
        {
            Ok(_) => Ok(value),
            Err(e) => Err(e),
        }
    }

    /// Takes the value out of the singleton, leaving it uninitialized.
    /// Equivalent to `replace(None)`.
    ///
    /// # Returns
    /// - `Ok(Some(value))` - The value that was taken
    /// - `Ok(None)` - Was already uninitialized
    /// - `Err(error)` - Failed to acquire mutable access
    fn take<'a>(&'a self) -> Result<Option<T>, Self::Error<'a>> {
        self.replace(None)
    }

    /// Resets the singleton to uninitialized state.
    /// Alias for `take()`.
    fn reset<'a>(&'a self) -> Result<Option<T>, Self::Error<'a>> {
        self.take()
    }

    /// Resets the singleton to uninitialized state.
    /// 
    /// # Panics
    /// Panics if the singleton cannot acquire mutable access.
    fn reset_or_panic(&self) -> Option<T> {
        self.reset().unwrap_or_else(|e| {
            panic!("SingletonOptionCell<{}> reset failed: {:?}", std::any::type_name::<T>(), e)
        })
    }
}