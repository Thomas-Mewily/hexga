
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

mod once_cell;
pub use once_cell::*;

mod once_lock;
pub use once_lock::*;

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
    pub use super::{SingletonOptionable, SingletonEmptyStruct};
}

/// Single interface for every singleton type.
/// The generally wrap std type, but it make sure that it expose only the same set of methods for reading/writting the value,
/// regardless of how the singleton should be initiliazed,
/// and how it should be guarded.
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
                <Self as GuardedMut<T>>::Error<'a>
            > {
                GuardedMut::try_get_mut(self)
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
                <Self as Guarded<T>>::Error<'a>
            > {
                Guarded::try_get(self)
            }
        }
    };
}




pub trait SingletonOptionable<T>: 
    GuardedMut<T>
{
    /// Tries to initialize the singleton from a fn, 
    /// while blocking the current thread until it is able to do so.
    /// 
    /// If already initialized, returns the existing guard without calling the function.
    fn init_from_fn<'a,F>(&'a self, init: F) -> Result<Self::GuardMut<'a>, F> 
        where F: FnOnce() -> T;

    /// Swaps the value with an external value, 
    /// while blocking the current thread until it is able to do so.
    fn swap<'a>(&'a self, other: &mut Option<T>) -> Result<(), Self::Error<'a>>;

    /// Ensures the singleton is initialized with the given value, 
    /// while blocking the current thread until it is able to do so.
    /// 
    /// If already initialized, returns the existing guard without replacing the value.
    fn init<'a>(&'a self, value: T) -> Result<Self::GuardMut<'a>, T> 
    {
        match self.init_from_fn(|| value)
        {
            Ok(v) => Ok(v),
            Err(init) => Err(init()),
        }
    }


    /// Replaces the current value with the given option, returning the previous value, 
    /// while blocking the current thread until it is able to do so.
    /// 
    /// If `None` is provided, the singleton becomes uninitialized.
    fn replace<'a>(&'a self, mut value: Option<T>) -> Result<Option<T>, Self::Error<'a>> {
        self.swap(&mut value)?;
        Ok(value)
    }

    /// Takes the value out of the singleton, leaving it uninitialized,
    /// while blocking the current thread until it is able to do so.
    /// 
    /// Equivalent to `replace(None)`.
    fn take<'a>(&'a self) -> Result<Option<T>, Self::Error<'a>> {
        self.replace(None)
    }

    /// Resets the singleton to uninitialized state,
    /// while blocking the current thread until it is able to do so.
    /// 
    /// Alias for `take()`.
    fn reset<'a>(&'a self) -> Result<Option<T>, Self::Error<'a>> {
        self.take()
    }
}

// Similar to SingletonOptionable, except initialization return a Guard and not a GuardMut
pub trait SingletonOnceable<T>: Guarded<T> 
{
    /// Tries to initialize the singleton from a fn, 
    /// while blocking the current thread until it is able to do so.
    /// 
    /// If already initialized, returns the existing guard without calling the function.
    fn init_from_fn<'a,F>(&'a self, init: F) -> Result<Self::Guard<'a>, F> 
        where F: FnOnce() -> T;
    /// Ensures the singleton is initialized with the given value, 
    /// while blocking the current thread until it is able to do so.
    /// 
    /// If already initialized, returns the existing guard without replacing the value.
    fn init<'a>(&'a self, value: T) -> Result<Self::Guard<'a>, T> 
    {
        match self.init_from_fn(|| value)
        {
            Ok(v) => Ok(v),
            Err(init) => Err(init()),
        }
    }
}

/// A trait for singletons that exist as a zero-sized type (ZST).
///
/// It is typically implemented for empty structs that delegate to a real static singleton instance.
pub trait SingletonEmptyStruct : Copy
{
    fn is_init() -> bool;
    fn is_not_init() -> bool { !Self::is_init() }
}