use super::*;
use std::{cell::LazyCell, sync::{LazyLock, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard}};
use crate::cell::{SingleThreadCell, SingleThreadError, SingleThreadMutError};

pub type SingletonSingleThread<T> = Singleton<SingleThreadCell<LazyCell<T>>>;
pub type SingletonMultiThread<T> = Singleton<RwLock<LazyLock<T>>>;

pub mod prelude
{
    pub use super::{SingletonSingleThread,SingletonMultiThread};
}

pub struct Singleton<T>
{
    inner: T,
}
impl<T> From<T> for Singleton<T>
{
    fn from(value: T) -> Self {
        Self::from_inner(value)
    }
}
impl<T> Debug for Singleton<T> where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Singleton({:?})", self.inner) }
}
impl<T> Deref for Singleton<T> where T: Deref
{
    type Target=T::Target;
    #[inline(always)]
    #[track_caller]
    fn deref(&self) -> &Self::Target { self.inner.deref() }
}
impl<T> DerefMut for Singleton<T> where T: DerefMut
{
    #[inline(always)]
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target { self.inner.deref_mut() }
}


// Single Thread: Todo: merge it with multi thread with generic if possible
pub type SingletonSingleThreadRead<'a,T> = SingletonGuard<std::cell::Ref<'a,T>>;
pub type SingletonSingleThreadReadError = SingleThreadError;
pub type SingletonSingleThreadReadResult<'a,T> = Result<SingletonSingleThreadRead<'a,T>,SingletonSingleThreadReadError>;

pub type SingletonSingleThreadWrite<'a,T> = SingletonGuard<std::cell::RefMut<'a,T>>;
pub type SingletonSingleThreadWriteError = SingleThreadMutError;
pub type SingletonSingleThreadWriteResult<'a,T> = Result<SingletonSingleThreadWrite<'a,T>, SingletonSingleThreadWriteError>;

impl<T> ReadGuard<T> for SingletonSingleThread<T>
{
    type ReadGuard<'a> = SingletonSingleThreadRead<'a,T> where Self: 'a;
    fn read<'a>(&'a self) -> Self::ReadGuard<'a> { SingletonSingleThreadRead::new(self.inner.read().guard_map(Deref::deref)) }
}
impl<T> TryReadGuard<T> for SingletonSingleThread<T>
{
    type Error<'a> = SingletonSingleThreadReadError where Self: 'a;
    fn try_read<'a>(&'a self) -> SingletonSingleThreadReadResult<'a,T>
    {
        Ok(SingletonSingleThreadRead::new(self.inner.try_read()?.guard_map(Deref::deref)))
    }
}
impl<T> WriteGuard<T> for SingletonSingleThread<T>
{
    type WriteGuard<'a> = SingletonSingleThreadWrite<'a,T> where Self: 'a;
    fn write<'a>(&'a self) -> Self::WriteGuard<'a> { SingletonSingleThreadWrite::new(self.inner.write().guard_map_mut(DerefMut::deref_mut)) }
}
impl<T> TryWriteGuard<T> for SingletonSingleThread<T>
{
    type Error<'a> = SingletonSingleThreadWriteError where Self: 'a;
    fn try_write<'a>(&'a self) -> SingletonSingleThreadWriteResult<'a, T>
    {
        Ok(SingletonSingleThreadWrite::new(self.inner.try_write()?.guard_map_mut(DerefMut::deref_mut)))
    }
}

// Multi Thread
pub type SingletonMultiThreadRead<'a,T> = SingletonGuard<std::sync::MappedRwLockReadGuard<'a,T>>;
pub type SingletonMultiThreadReadError<'a,T> = PoisonError<RwLockReadGuard<'a, LazyLock<T>>>;
pub type SingletonMultiThreadReadResult<'a,T> = Result<SingletonMultiThreadRead<'a,T>,SingletonMultiThreadReadError<'a,T>>;

pub type SingletonMultiThreadWrite<'a,T> = SingletonGuard<std::sync::MappedRwLockWriteGuard<'a,T>>;
pub type SingletonMultiThreadWriteError<'a,T> = std::sync::TryLockError<RwLockWriteGuard<'a, LazyLock<T>>>;
pub type SingletonMultiThreadWriteResult<'a,T> = Result<SingletonMultiThreadWrite<'a,T>,SingletonMultiThreadWriteError<'a,T>>;


impl<T> ReadGuard<T> for SingletonMultiThread<T>
{
    type ReadGuard<'a> = SingletonMultiThreadRead<'a,T> where Self: 'a;
    fn read<'a>(&'a self) -> Self::ReadGuard<'a> { SingletonMultiThreadRead::new(self.inner.read().expect("can't read").guard_map(Deref::deref)) }
}
impl<T> TryReadGuard<T> for SingletonMultiThread<T>
{
    type Error<'a> = SingletonMultiThreadReadError<'a, T> where Self: 'a;
    fn try_read<'a>(&'a self) -> SingletonMultiThreadReadResult<'a, T> { Ok(SingletonMultiThreadRead::new(self.inner.read()?.guard_map(Deref::deref))) }
}
impl<T> WriteGuard<T> for SingletonMultiThread<T>
{
    type WriteGuard<'a> = SingletonMultiThreadWrite<'a,T> where Self: 'a;
    fn write<'a>(&'a self) -> Self::WriteGuard<'a> { SingletonMultiThreadWrite::new(self.inner.write().expect("can't write").guard_map_mut(DerefMut::deref_mut)) }
}
impl<T> TryWriteGuard<T> for SingletonMultiThread<T>
{
    type Error<'a> = SingletonMultiThreadWriteError<'a,T> where Self: 'a;
    fn try_write<'a>(&'a self) -> SingletonMultiThreadWriteResult<'a, T> { Ok(SingletonMultiThreadWrite::new(self.inner.try_write()?.guard_map_mut(DerefMut::deref_mut))) }
}


pub struct SingletonGuard<D>
{
    inner: D,
}
impl<D> SingletonGuard<D>
{
    pub(crate) fn new(inner: D) -> Self { Self { inner }}
}
impl<D> Deref for SingletonGuard<D> where D: Deref
{
    type Target=D::Target;
    fn deref(&self) -> &Self::Target { self.inner.deref() }
}
impl<D> DerefMut for SingletonGuard<D> where D: DerefMut
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.inner.deref_mut() }
}

/*
impl<T> ReadGuard<T> for SingletonSingleThread<T> where T: ReadGuard<T>
{
    type ReadGuard<'a> = T::ReadGuard<'a> where Self: 'a;
    fn read<'a>(&'a self) -> Self::ReadGuard<'a> { self.inner.read() }
}

impl<T> ReadGuard<T> for Singleton<SingleThreadCell<T>>
{
    type ReadGuard<'a> = <SingleThreadCell::<T> as ReadGuard<T>>::ReadGuard<'a> where Self: 'a;
    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.inner.read()
    }
}
*/



/*
impl<T> ReadGuard<T> for Singleton<T> where T: ReadGuard<T>
{
    type ReadGuard<'a> = T::ReadGuard<'a> where Self: 'a;
    fn read<'a>(&'a self) -> Self::ReadGuard<'a> { self.inner.read() }
}
impl<T> TryReadGuard<T> for Singleton<T> where T: TryReadGuard<T>
{
    type Error<'a> = T::Error<'a> where Self: 'a;
    fn try_read<'a>(&'a self) -> Result<Self::ReadGuard<'a>, Self::Error<'a>> { self.inner.try_read() }
}
impl<T> WriteGuard<T> for Singleton<T> where T: WriteGuard<T>
{
    type WriteGuard<'a> = T::WriteGuard<'a> where Self: 'a;
    fn write<'a>(&'a self) -> Self::WriteGuard<'a> { self.inner.write() }
}
impl<T> TryWriteGuard<T> for Singleton<T> where T: TryWriteGuard<T>
{
    type Error<'a> = T::Error<'a> where Self: 'a;
    fn try_write<'a>(&'a self) -> Result<Self::WriteGuard<'a>, Self::Error<'a>> { self.inner.try_write() }
}
*/

impl<T> Singleton<T>
{
    pub const fn from_inner(inner: T) -> Self { Self { inner } }
    pub fn into_inner(self) -> T { self.inner }
}

unsafe impl<T> Sync for Singleton<T> where T: Sync {}
unsafe impl<T> Send for Singleton<T> where T: Send {}

impl<T> SingletonSingleThread<T>
{
    pub const fn new(f: fn() -> T) -> Self { Self::from_inner(SingleThreadCell::new(LazyCell::new(f))) }
}
impl<T> SingletonMultiThread<T>
{
    pub const fn new(f: fn() -> T) -> Self { Self::from_inner(RwLock::new(LazyLock::new(f))) }
}
