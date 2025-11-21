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
impl<'a,T> ReadGuard for &'a Singleton<T> where &'a T: ReadGuard
{
    type Target = <&'a T as ReadGuard>::Target;
    type ReadGuard = <&'a T as ReadGuard>::ReadGuard;
    fn read(self) -> Self::ReadGuard { self.inner.read() }
}
impl<'a,T> TryReadGuard for &'a Singleton<T> where &'a T: TryReadGuard
{
    type Error = <&'a T as TryReadGuard>::Error;
    fn try_read(self) -> Result<Self::ReadGuard, Self::Error> { self.inner.try_read() }
}
impl<'a,T> WriteGuard for &'a Singleton<T> where &'a T: WriteGuard
{
    type WriteGuard = <&'a T as WriteGuard>::WriteGuard;
    fn write(self) -> Self::WriteGuard { self.inner.write() }
}
impl<'a,T> TryWriteGuard for &'a Singleton<T> where &'a T: TryWriteGuard
{
    type Error = <&'a T as TryWriteGuard>::Error;
    fn try_write(self) -> Result<Self::WriteGuard, Self::Error> { self.inner.try_write() }
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
