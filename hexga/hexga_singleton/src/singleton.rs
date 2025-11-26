use hexga_core::cell::SingleThread;

use super::*;

pub type SingletonSingleThread<T> = Singleton<SingleThread<LazyCell<T>>>;
pub type SingletonMultiThread<T> = Singleton<RwLock<LazyLock<T>>>;



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
impl<T> ReadGuard for Singleton<T> where T: ReadGuard
{
    type Target = <T as ReadGuard>::Target;
    type ReadGuard<'a> = T::ReadGuard<'a> where Self: 'a;
    fn read<'a>(&'a self) -> Self::ReadGuard<'a> { self.inner.read() }
}
impl<T> TryReadGuard for Singleton<T> where T: TryReadGuard
{
    type Error<'a> = T::Error<'a> where Self: 'a;
    fn try_read<'a>(&'a self) -> Result<Self::ReadGuard<'a>, Self::Error<'a>> { self.inner.try_read() }
}
impl<T> WriteGuard for Singleton<T> where T: WriteGuard
{
    type WriteGuard<'a> = T::WriteGuard<'a> where Self: 'a;
    fn write<'a>(&'a self) -> Self::WriteGuard<'a> { self.inner.write() }
}
impl<T> TryWriteGuard for Singleton<T> where T: TryWriteGuard
{
    type Error<'a> = T::Error<'a> where Self: 'a;
    fn try_write<'a>(&'a self) -> Result<Self::WriteGuard<'a>, Self::Error<'a>> { self.inner.try_write() }
}

impl<T> Singleton<T>
{
    pub const fn from_inner(inner: T) -> Self { Self { inner } }
}

unsafe impl<T> Sync for Singleton<T> where T: Sync {}
unsafe impl<T> Send for Singleton<T> where T: Send {}

impl<T> SingletonSingleThread<T>
{
    pub const fn new(f: fn() -> T) -> Self { Self::from_inner(SingleThread::new(LazyCell::new(f))) }
}
impl<T> SingletonMultiThread<T>
{
    pub const fn new(f: fn() -> T) -> Self { Self::from_inner(RwLock::new(LazyLock::new(f))) }
}
