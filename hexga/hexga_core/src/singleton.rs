use super::*;
use crate::cell::{SingleThreadCell, SingleThreadError, SingleThreadMutError};
use std::{
    cell::LazyCell,
    sync::{
        LazyLock, MappedMutexGuard, MappedRwLockReadGuard, MappedRwLockWriteGuard, Mutex,
        PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError,
    },
};

/// Single thread singleton
pub type SingletonSingleThread<T> = SingletonOf<SingleThreadCell<LazyCell<T>>>;
//pub type SingletonRwLock<T> = SingletonOf<Mutex<LazyLock<T>>>;
/// Multi thread singleton
pub type SingletonRwLock<T> = SingletonOf<RwLock<LazyLock<T>>>;
/// Multi thread singleton
pub type SingletonMutex<T> = SingletonOf<Mutex<LazyLock<T>>>;

pub type Singleton<T> = SingletonMutex<T>;

pub mod prelude
{
    pub use super::{Singleton, SingletonMutex, SingletonRwLock, SingletonSingleThread};
}

pub struct SingletonOf<T>
{
    inner: T,
}
impl<T> From<T> for SingletonOf<T>
{
    fn from(value: T) -> Self { Self::from_inner(value) }
}
impl<T> Debug for SingletonOf<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Singleton({:?})", self.inner)
    }
}
impl<T> Deref for SingletonOf<T>
where
    T: Deref,
{
    type Target = T::Target;
    #[inline(always)]
    #[track_caller]
    fn deref(&self) -> &Self::Target { self.inner.deref() }
}
impl<T> DerefMut for SingletonOf<T>
where
    T: DerefMut,
{
    #[inline(always)]
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target { self.inner.deref_mut() }
}

// Single Thread: Todo: merge it with multi thread with generic if possible
pub type SingletonSingleThreadReadOf<'a, T> = SingletonGuard<std::cell::Ref<'a, T>>;
pub type SingletonSingleThreadReadError = SingleThreadError;
pub type SingletonSingleThreadReadResultOf<'a, T> =
    Result<SingletonSingleThreadReadOf<'a, T>, SingletonSingleThreadReadError>;

pub type SingletonSingleThreadRead<T> = SingletonSingleThreadReadOf<'static, T>;
pub type SingletonSingleThreadReadResult<T> = SingletonSingleThreadReadResultOf<'static, T>;

pub type SingletonSingleThreadWriteOf<'a, T> = SingletonGuard<std::cell::RefMut<'a, T>>;
pub type SingletonSingleThreadWriteError = SingleThreadMutError;
pub type SingletonSingleThreadWriteResultOf<'a, T> =
    Result<SingletonSingleThreadWriteOf<'a, T>, SingletonSingleThreadWriteError>;

pub type SingletonSingleThreadWrite<T> = SingletonSingleThreadWriteOf<'static, T>;
pub type SingletonSingleThreadWriteResult<T> = SingletonSingleThreadWriteResultOf<'static, T>;

impl<T> Guarded<T> for SingletonSingleThread<T>
{
    type Guard<'a>
        = SingletonSingleThreadReadOf<'a, T>
    where
        Self: 'a;
    fn get<'a>(&'a self) -> Self::Guard<'a>
    {
        SingletonSingleThreadReadOf::new(self.inner.get().guard_map(Deref::deref))
    }
}
impl<T> TryGuarded<T> for SingletonSingleThread<T>
{
    type Error<'a>
        = SingletonSingleThreadReadError
    where
        Self: 'a;
    fn try_get<'a>(&'a self) -> SingletonSingleThreadReadResultOf<'a, T>
    {
        Ok(SingletonSingleThreadReadOf::new(
            self.inner.try_get()?.guard_map(Deref::deref),
        ))
    }
}
impl<T> GuardedMut<T> for SingletonSingleThread<T>
{
    type GuardMut<'a>
        = SingletonSingleThreadWriteOf<'a, T>
    where
        Self: 'a;
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a>
    {
        SingletonSingleThreadWriteOf::new(self.inner.get_mut().guard_map_mut(DerefMut::deref_mut))
    }
}
impl<T> TryGuardedMut<T> for SingletonSingleThread<T>
{
    type Error<'a>
        = SingletonSingleThreadWriteError
    where
        Self: 'a;
    fn try_get_mut<'a>(&'a self) -> SingletonSingleThreadWriteResultOf<'a, T>
    {
        Ok(SingletonSingleThreadWriteOf::new(
            self.inner.try_get_mut()?.guard_map_mut(DerefMut::deref_mut),
        ))
    }
}

// Multi Thread RwLock
pub type SingletonRwLockReadOf<'a, T> = SingletonGuard<MappedRwLockReadGuard<'a, T>>;
pub type SingletonRwLockReadErrorOf<'a, T> = PoisonError<RwLockReadGuard<'a, LazyLock<T>>>;
pub type SingletonRwLockReadResultOf<'a, T> =
    Result<SingletonRwLockReadOf<'a, T>, SingletonRwLockReadErrorOf<'a, T>>;

pub type SingletonRwLockRead<T> = SingletonRwLockReadOf<'static, T>;
pub type SingletonRwLockReadError<T> = SingletonRwLockReadErrorOf<'static, T>;
pub type SingletonRwLockReadResult<T> = SingletonRwLockReadResultOf<'static, T>;

pub type SingletonRwLockWriteOf<'a, T> = SingletonGuard<MappedRwLockWriteGuard<'a, T>>;
pub type SingletonRwLockWriteErrorOf<'a, T> = TryLockError<RwLockWriteGuard<'a, LazyLock<T>>>;
pub type SingletonRwLockWriteResultOf<'a, T> =
    Result<SingletonRwLockWriteOf<'a, T>, SingletonRwLockWriteErrorOf<'a, T>>;

pub type SingletonRwLockWrite<T> = SingletonRwLockWriteOf<'static, T>;
pub type SingletonRwLockWriteError<T> = SingletonRwLockWriteErrorOf<'static, T>;
pub type SingletonRwLockWriteResult<T> = SingletonRwLockWriteResultOf<'static, T>;

impl<T> Guarded<T> for SingletonRwLock<T>
{
    type Guard<'a>
        = SingletonRwLockReadOf<'a, T>
    where
        Self: 'a;
    fn get<'a>(&'a self) -> Self::Guard<'a>
    {
        SingletonRwLockReadOf::new(self.inner.get().guard_map(Deref::deref))
    }
}
impl<T> TryGuarded<T> for SingletonRwLock<T>
{
    type Error<'a>
        = SingletonRwLockReadErrorOf<'a, T>
    where
        Self: 'a;
    fn try_get<'a>(&'a self) -> SingletonRwLockReadResultOf<'a, T>
    {
        Ok(SingletonRwLockReadOf::new(
            self.inner.try_get()?.guard_map(Deref::deref),
        ))
    }
}
impl<T> GuardedMut<T> for SingletonRwLock<T>
{
    type GuardMut<'a>
        = SingletonRwLockWriteOf<'a, T>
    where
        Self: 'a;
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a>
    {
        SingletonRwLockWriteOf::new(self.inner.get_mut().guard_map_mut(DerefMut::deref_mut))
    }
}
impl<T> TryGuardedMut<T> for SingletonRwLock<T>
{
    type Error<'a>
        = SingletonRwLockWriteErrorOf<'a, T>
    where
        Self: 'a;
    fn try_get_mut<'a>(&'a self) -> SingletonRwLockWriteResultOf<'a, T>
    {
        Ok(SingletonRwLockWriteOf::new(
            self.inner.try_get_mut()?.guard_map_mut(DerefMut::deref_mut),
        ))
    }
}

// Multi Thread Mutex
pub type SingletonMutexWriteOf<'a, T> = SingletonGuard<MappedMutexGuard<'a, T>>;
pub type SingletonMutexWriteErrorOf<'a, T> = PoisonError<std::sync::MutexGuard<'a, LazyLock<T>>>;
pub type SingletonMutexWriteResultOf<'a, T> =
    Result<SingletonMutexWriteOf<'a, T>, SingletonMutexWriteErrorOf<'a, T>>;

pub type SingletonMutexWrite<T> = SingletonMutexWriteOf<'static, T>;
pub type SingletonMutexWriteError<T> = SingletonMutexWriteErrorOf<'static, T>;
pub type SingletonMutexWriteResult<T> = SingletonMutexWriteResultOf<'static, T>;

impl<T> GuardedMut<T> for SingletonMutex<T>
{
    type GuardMut<'a>
        = SingletonMutexWriteOf<'a, T>
    where
        Self: 'a;
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a>
    {
        SingletonMutexWriteOf::new(self.inner.get_mut().guard_map_mut(DerefMut::deref_mut))
    }
}
impl<T> TryGuardedMut<T> for SingletonMutex<T>
{
    type Error<'a>
        = SingletonMutexWriteErrorOf<'a, T>
    where
        Self: 'a;
    fn try_get_mut<'a>(&'a self) -> SingletonMutexWriteResultOf<'a, T>
    {
        Ok(SingletonMutexWriteOf::new(
            self.inner.try_get_mut()?.guard_map_mut(DerefMut::deref_mut),
        ))
    }
}

pub struct SingletonGuard<G>
{
    guard: G,
}
impl<D> SingletonGuard<D>
{
    pub(crate) fn new(guard: D) -> Self { Self { guard } }
}
impl<D> Deref for SingletonGuard<D>
where
    D: Deref,
{
    type Target = D::Target;
    fn deref(&self) -> &Self::Target { self.guard.deref() }
}
impl<D> DerefMut for SingletonGuard<D>
where
    D: DerefMut,
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.guard.deref_mut() }
}
impl<'a, D, T> Guard<'a, T> for SingletonGuard<D>
where
    D: Guard<'a, T>,
{
    type Mapped<U>
        = SingletonGuard<D::Mapped<U>>
    where
        U: 'a + ?Sized;

    fn guard_map<U, F>(self, f: F) -> Self::Mapped<U>
    where
        F: FnOnce(&T) -> &U,
        U: 'a + ?Sized,
    {
        SingletonGuard::new(self.guard.guard_map(f))
    }
}
impl<'a, D, T> GuardMut<'a, T> for SingletonGuard<D>
where
    D: GuardMut<'a, T>,
{
    type MappedMut<U>
        = SingletonGuard<D::MappedMut<U>>
    where
        U: 'a + ?Sized;

    fn guard_map_mut<U, F>(self, f: F) -> Self::MappedMut<U>
    where
        F: FnOnce(&mut T) -> &mut U,
        U: 'a + ?Sized,
    {
        SingletonGuard::new(self.guard.guard_map_mut(f))
    }
}

impl<T> SingletonOf<T>
{
    pub const fn from_inner(inner: T) -> Self { Self { inner } }
    pub fn into_inner(self) -> T { self.inner }
    pub fn inner(&self) -> &T { &self.inner }
}

unsafe impl<T> Sync for SingletonOf<T> where T: Sync {}
unsafe impl<T> Send for SingletonOf<T> where T: Send {}

impl<T> SingletonSingleThread<T>
{
    pub const fn new(f: fn() -> T) -> Self
    {
        Self::from_inner(SingleThreadCell::new(LazyCell::new(f)))
    }
}
impl<T> SingletonRwLock<T>
{
    pub const fn new(f: fn() -> T) -> Self { Self::from_inner(RwLock::new(LazyLock::new(f))) }
}
impl<T> SingletonMutex<T>
{
    pub const fn new(f: fn() -> T) -> Self { Self::from_inner(Mutex::new(LazyLock::new(f))) }
}
